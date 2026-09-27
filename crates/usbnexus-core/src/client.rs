// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! USB Nexus client: connects to servers, pairs, lists and attaches devices.

use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use tokio::net::TcpStream;
use tokio::time::timeout;
use tokio_rustls::client::TlsStream;
use tokio_rustls::TlsConnector;
use tracing::{debug, info, warn};
use usbnexus_proto::DeviceInfo;

use crate::backend::ImportBackend;
use crate::backoff::Backoff;
use crate::control::{ClientMsg, ErrorCode, ExportedDevice, RemoteError, ServerMsg};
use crate::frame::{read_frame, write_frame};
use crate::identity::Identity;
use crate::pairing::{Pake, Role};
use crate::relay::{relay, RelayEnd};
use crate::trust::{now_unix, Peer, TrustStore};
use crate::{discovery, tls, CONTROL_VERSION, DEFAULT_PORT};

const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const REPLY_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Clone)]
pub struct ClientConfig {
    pub name: String,
    pub identity: Identity,
    pub trust: TrustStore,
}

/// Errors callers may want to handle specifically.
#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("pairing required with server {name} ({fingerprint})")]
    PairingRequired { name: String, fingerprint: String },
    #[error("server {0} could not be found")]
    NotFound(String),
}

/// An authenticated control session with a server.
pub struct Session {
    stream: TlsStream<TcpStream>,
    pub server_name: String,
    pub server_fingerprint: String,
    pub addr: String,
}

/// Adds the default port when `addr` has none.
pub fn with_default_port(addr: &str) -> String {
    let has_port =
        if let Some(rest) = addr.strip_prefix('[') { rest.contains("]:") } else { addr.matches(':').count() == 1 };
    if has_port {
        addr.to_string()
    } else if addr.contains(':') {
        format!("[{addr}]:{DEFAULT_PORT}")
    } else {
        format!("{addr}:{DEFAULT_PORT}")
    }
}

fn remote(code: ErrorCode, message: String) -> anyhow::Error {
    RemoteError { code, message, queue_position: None }.into()
}

async fn expect<T>(s: &mut TlsStream<TcpStream>, f: impl FnOnce(ServerMsg) -> Option<T>) -> Result<T> {
    let msg: ServerMsg = timeout(REPLY_TIMEOUT, read_frame(s)).await.context("server did not reply")??;
    if let ServerMsg::Error { code, message, queue_position } = msg {
        return Err(RemoteError { code, message, queue_position }.into());
    }
    f(msg).ok_or_else(|| remote(ErrorCode::Protocol, "unexpected reply".into()))
}

/// Connects to `addr` and authenticates. When the server is not yet paired
/// (in either direction) `pin` is required and pairing is performed.
pub async fn connect(cfg: &ClientConfig, addr: &str, pin: Option<&str>) -> Result<Session> {
    let addr = with_default_port(addr);
    let tcp = timeout(CONNECT_TIMEOUT, TcpStream::connect(&addr))
        .await
        .with_context(|| format!("connecting to {addr} timed out"))?
        .with_context(|| format!("connecting to {addr}"))?;
    tcp.set_nodelay(true)?;
    let connector = TlsConnector::from(tls::client_config(&cfg.identity)?);
    let mut stream = timeout(CONNECT_TIMEOUT, connector.connect(tls::server_name(), tcp))
        .await
        .context("TLS handshake timed out")??;
    let (server_fp, binding) = {
        let conn = stream.get_ref().1;
        (tls::peer_fingerprint(conn)?, tls::session_binding_client(conn)?)
    };

    write_frame(&mut stream, &ClientMsg::Hello { version: CONTROL_VERSION, name: cfg.name.clone() }).await?;
    let (server_name, client_trusted) = expect(&mut stream, |m| match m {
        ServerMsg::Welcome { version, name, client_trusted, .. } if version == CONTROL_VERSION => {
            Some((name, client_trusted))
        }
        _ => None,
    })
    .await?;
    let server_name: String = server_name.chars().filter(|c| !c.is_control()).take(64).collect();

    let server_trusted = cfg.trust.is_trusted(&server_fp);
    if !(server_trusted && client_trusted) {
        let Some(pin) = pin else {
            return Err(ClientError::PairingRequired { name: server_name, fingerprint: server_fp }.into());
        };
        let (pake, msg) = Pake::start(Role::Client, pin);
        write_frame(&mut stream, &ClientMsg::PairStart { spake: msg }).await?;
        let server_msg = expect(&mut stream, |m| match m {
            ServerMsg::PairStart { spake } => Some(spake),
            _ => None,
        })
        .await?;
        let confirm = pake.finish(&server_msg, &binding)?;
        write_frame(&mut stream, &ClientMsg::PairConfirm { mac: confirm.own_mac() }).await?;
        let mac = expect(&mut stream, |m| match m {
            ServerMsg::PairConfirm { mac } => Some(mac),
            _ => None,
        })
        .await?;
        if !confirm.verify_peer(&mac) {
            return Err(remote(ErrorCode::PairingFailed, "server confirmation invalid".into()));
        }
        cfg.trust.add(Peer {
            name: server_name.clone(),
            fingerprint: server_fp.clone(),
            last_addr: Some(addr.clone()),
            paired_at: now_unix(),
        })?;
        info!(server = %server_name, "paired with server");
    } else {
        cfg.trust.set_last_addr(&server_fp, &addr)?;
    }

    Ok(Session { stream, server_name, server_fingerprint: server_fp, addr })
}

impl Session {
    pub async fn list(&mut self) -> Result<Vec<ExportedDevice>> {
        write_frame(&mut self.stream, &ClientMsg::ListDevices).await?;
        expect(&mut self.stream, |m| match m {
            ServerMsg::Devices { devices } => Some(devices),
            _ => None,
        })
        .await
    }

    /// Imports `device` (a device identity, or a bus id). Returns the
    /// device, its identity, and the stream, which from now on carries raw
    /// USB/IP URB traffic.
    pub async fn import(mut self, device: &str) -> Result<(DeviceInfo, String, TlsStream<TcpStream>)> {
        write_frame(&mut self.stream, &ClientMsg::Import { device: device.to_string() }).await?;
        let (info, id) = expect(&mut self.stream, |m| match m {
            ServerMsg::Imported { device, id } => Some((device, id)),
            _ => None,
        })
        .await?;
        // Older servers do not report the identity.
        Ok((info, id.unwrap_or_else(|| device.to_string()), self.stream))
    }
}

/// How to reach a server.
#[derive(Debug, Clone)]
pub enum Target {
    /// A fixed `host[:port]`.
    Addr(String),
    /// A paired server, found via mDNS by fingerprint, falling back to its
    /// last known address. Survives DHCP address changes.
    Peer { fingerprint: String },
}

impl Target {
    /// Resolves a user-supplied string: a paired peer's name or fingerprint,
    /// otherwise an address.
    pub fn parse(s: &str, trust: &TrustStore) -> Target {
        match trust.find(s) {
            Some(p) => Target::Peer { fingerprint: p.fingerprint },
            None => Target::Addr(s.to_string()),
        }
    }

    /// Connects to the target. For a paired peer the last known address is
    /// tried first; if it is unreachable the peer is looked up via mDNS. The
    /// server must present the expected fingerprint.
    pub async fn connect(&self, cfg: &ClientConfig, events: &impl Fn(AttachEvent)) -> Result<Session> {
        let fingerprint = match self {
            Target::Addr(a) => {
                events(AttachEvent::Connecting { addr: a.clone() });
                return connect(cfg, a, None).await;
            }
            Target::Peer { fingerprint } => fingerprint,
        };
        let check = |s: Session| -> Result<Session> {
            if s.server_fingerprint != *fingerprint {
                return Err(ClientError::NotFound(fingerprint.clone()).into());
            }
            Ok(s)
        };
        let last = cfg.trust.get(fingerprint).and_then(|p| p.last_addr);
        let mut last_err = None;
        if let Some(addr) = &last {
            events(AttachEvent::Connecting { addr: addr.clone() });
            match connect(cfg, addr, None).await.and_then(check) {
                Ok(s) => return Ok(s),
                Err(e) => last_err = Some(e),
            }
        }
        match discovery::find(fingerprint, Duration::from_secs(3)).await {
            Ok(Some(found)) => {
                for addr in found.addrs.iter().map(|a| a.to_string()).filter(|a| Some(a) != last.as_ref()) {
                    events(AttachEvent::Connecting { addr: addr.clone() });
                    match connect(cfg, &addr, None).await.and_then(check) {
                        Ok(s) => return Ok(s),
                        Err(e) => last_err = Some(e),
                    }
                }
            }
            Ok(None) => {}
            Err(e) => debug!("mDNS lookup failed: {e:#}"),
        }
        Err(last_err.unwrap_or_else(|| ClientError::NotFound(fingerprint.clone()).into()))
    }
}

/// Progress of [`attach_forever`], for display in the CLI or GUI.
#[derive(Debug, Clone)]
pub enum AttachEvent {
    Connecting {
        addr: String,
    },
    Attached {
        port: u32,
        device: DeviceInfo,
        /// Identity of the device on the server. When the attachment was
        /// asked for by bus id, later attempts use this instead.
        id: String,
    },
    /// The connection failed or dropped; `error.code` says why.
    Disconnected {
        error: crate::api::ApiError,
    },
    Retrying {
        delay: Duration,
    },
    /// Another computer uses the device; this one is `position` in its
    /// queue (1 = next) and asks again soon.
    Queued {
        position: u32,
    },
    /// The device was detached locally; the loop has ended.
    Detached,
}

/// How often to ask again for a device that is not plugged in.
const DEVICE_WAIT: Duration = Duration::from_secs(5);
/// How often a queued client asks again: well within the server's queue
/// lifetime and its reservation for the next in line.
const QUEUE_WAIT: Duration = Duration::from_secs(2);
/// Pause after a connection was lost while a device was attached, so the
/// local USB stack has removed the old virtual device (and, after a
/// hotplug, the server's operating system has finished enumerating the
/// device) before it is attached again.
const REATTACH_GRACE: Duration = Duration::from_secs(3);

/// Whether an error cannot be fixed by retrying. A missing device or a
/// missing permission is not permanent: the device may be plugged in, or
/// permission granted, later.
fn is_permanent(e: &anyhow::Error) -> bool {
    if e.downcast_ref::<ClientError>().is_some_and(|c| matches!(c, ClientError::PairingRequired { .. })) {
        return true;
    }
    if let Some(r) = e.downcast_ref::<RemoteError>() {
        return matches!(r.code, ErrorCode::NotTrusted | ErrorCode::Version | ErrorCode::PairingFailed);
    }
    false
}

/// Attaches `device` (a device identity, or a bus id) from `target` and
/// keeps it attached: when the network connection drops, or the device is
/// unplugged, it retries and re-attaches the device when it is back.
/// Returns when the device is detached locally or on a permanent error.
pub async fn attach_forever(
    cfg: &ClientConfig,
    target: &Target,
    device: &str,
    backend: Arc<dyn ImportBackend>,
    events: impl Fn(AttachEvent),
) -> Result<()> {
    let mut backoff = Backoff::default();
    let mut wanted = device.to_string();
    loop {
        let attempt = async {
            let session = target.connect(cfg, &events).await?;
            session.import(&wanted).await
        };
        let (device, id, stream) = match attempt.await {
            Ok(v) => v,
            Err(e) if is_permanent(&e) => return Err(e),
            Err(e) => {
                let error = crate::api::ApiError::from_anyhow(&e);
                let missing = error.code == "no_such_device";
                let queued = e.downcast_ref::<RemoteError>().and_then(|r| r.queue_position);
                if let Some(position) = queued {
                    debug!(position, "waiting in the device queue");
                    events(AttachEvent::Queued { position });
                    tokio::time::sleep(QUEUE_WAIT).await;
                    continue;
                }
                if missing {
                    debug!("waiting for the device: {e:#}");
                } else {
                    warn!("attach attempt failed: {e:#}");
                }
                events(AttachEvent::Disconnected { error });
                let delay = if missing { backoff.next_delay().min(DEVICE_WAIT) } else { backoff.next_delay() };
                events(AttachEvent::Retrying { delay });
                tokio::time::sleep(delay).await;
                continue;
            }
        };

        wanted.clone_from(&id);
        let (port, local) = backend.attach(&device).await.context("attaching device locally")?;
        backoff.reset();
        info!(device = %id, busid = %device.busid, port, "device attached");
        events(AttachEvent::Attached { port, device: device.clone(), id });

        match relay(stream, local).await {
            RelayEnd::Local(err) => {
                info!(port, "device detached locally: {}", describe_end(err.as_ref()));
                events(AttachEvent::Detached);
                return Ok(());
            }
            RelayEnd::Remote(err) => {
                info!(port, "connection to the device lost: {}; attaching again", describe_end(err.as_ref()));
                // Closing our socket already makes the kernel drop the port;
                // an explicit detach is a best-effort cleanup.
                let _ = backend.detach(port);
                let reason = err.map(|e| e.to_string()).unwrap_or_else(|| "connection closed".into());
                events(AttachEvent::Disconnected { error: crate::api::ApiError::new("connection_lost", reason) });
                let delay = backoff.next_delay().max(REATTACH_GRACE);
                events(AttachEvent::Retrying { delay });
                tokio::time::sleep(delay).await;
            }
        }
    }
}

fn describe_end(err: Option<&std::io::Error>) -> String {
    err.map(|e| e.to_string()).unwrap_or_else(|| "connection closed".into())
}

/// Finds a device by identity or bus id in a list, for friendlier error messages.
pub fn find_device<'a>(devices: &'a [ExportedDevice], device: &str) -> Result<&'a ExportedDevice> {
    devices
        .iter()
        .find(|d| d.id == device || (d.present && d.info.busid == device))
        .ok_or_else(|| anyhow!("no device {device} on server"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_port() {
        assert_eq!(with_default_port("10.0.0.1"), "10.0.0.1:3241");
        assert_eq!(with_default_port("host:99"), "host:99");
        assert_eq!(with_default_port("fe80::1"), "[fe80::1]:3241");
        assert_eq!(with_default_port("[fe80::1]:5"), "[fe80::1]:5");
    }
}
