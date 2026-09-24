// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! USB Nexus client: connects to servers, pairs, lists and attaches devices.

use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use usbnexus_proto::DeviceInfo;
use tokio::net::TcpStream;
use tokio::time::timeout;
use tokio_rustls::client::TlsStream;
use tokio_rustls::TlsConnector;
use tracing::{debug, info, warn};

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
    RemoteError { code, message }.into()
}

async fn expect<T>(s: &mut TlsStream<TcpStream>, f: impl FnOnce(ServerMsg) -> Option<T>) -> Result<T> {
    let msg: ServerMsg = timeout(REPLY_TIMEOUT, read_frame(s)).await.context("server did not reply")??;
    if let ServerMsg::Error { code, message } = msg {
        return Err(remote(code, message));
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

    /// Imports `busid`. The returned stream carries raw USB/IP URB traffic.
    pub async fn import(mut self, busid: &str) -> Result<(DeviceInfo, TlsStream<TcpStream>)> {
        write_frame(&mut self.stream, &ClientMsg::Import { busid: busid.to_string() }).await?;
        let device = expect(&mut self.stream, |m| match m {
            ServerMsg::Imported { device } => Some(device),
            _ => None,
        })
        .await?;
        Ok((device, self.stream))
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

    pub async fn resolve(&self, trust: &TrustStore) -> Result<String> {
        match self {
            Target::Addr(a) => Ok(a.clone()),
            Target::Peer { fingerprint } => {
                match discovery::find(fingerprint, Duration::from_secs(2)).await {
                    Ok(Some(found)) => {
                        if let Some(a) = found.addrs.first() {
                            return Ok(a.to_string());
                        }
                    }
                    Ok(None) => {}
                    Err(e) => debug!("mDNS lookup failed: {e:#}"),
                }
                trust
                    .get(fingerprint)
                    .and_then(|p| p.last_addr)
                    .ok_or_else(|| ClientError::NotFound(fingerprint.clone()).into())
            }
        }
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
    },
    Disconnected {
        reason: String,
    },
    Retrying {
        delay: Duration,
    },
    /// The device was detached locally; the loop has ended.
    Detached,
}

/// Whether an error cannot be fixed by retrying.
fn is_permanent(e: &anyhow::Error) -> bool {
    if e.downcast_ref::<ClientError>().is_some_and(|c| matches!(c, ClientError::PairingRequired { .. })) {
        return true;
    }
    if let Some(r) = e.downcast_ref::<RemoteError>() {
        return matches!(
            r.code,
            ErrorCode::NotTrusted | ErrorCode::NoSuchDevice | ErrorCode::Version | ErrorCode::PairingFailed
        );
    }
    false
}

/// Attaches `busid` from `target` and keeps it attached: when the network
/// connection drops, it reconnects with backoff and re-attaches the device.
/// Returns when the device is detached locally or on a permanent error.
pub async fn attach_forever(
    cfg: &ClientConfig,
    target: &Target,
    busid: &str,
    backend: Arc<dyn ImportBackend>,
    events: impl Fn(AttachEvent),
) -> Result<()> {
    let mut backoff = Backoff::default();
    loop {
        let attempt = async {
            let addr = target.resolve(&cfg.trust).await?;
            events(AttachEvent::Connecting { addr: addr.clone() });
            let session = connect(cfg, &addr, None).await?;
            session.import(busid).await
        };
        let (device, stream) = match attempt.await {
            Ok(v) => v,
            Err(e) if is_permanent(&e) => return Err(e),
            Err(e) => {
                warn!("attach attempt failed: {e:#}");
                events(AttachEvent::Disconnected { reason: format!("{e:#}") });
                let delay = backoff.next_delay();
                events(AttachEvent::Retrying { delay });
                tokio::time::sleep(delay).await;
                continue;
            }
        };

        let (port, local) = backend.attach(&device).context("attaching device locally")?;
        backoff.reset();
        events(AttachEvent::Attached { port, device: device.clone() });

        match relay(stream, local).await {
            RelayEnd::Local(_) => {
                events(AttachEvent::Detached);
                return Ok(());
            }
            RelayEnd::Remote(err) => {
                // Closing our socket already makes the kernel drop the port;
                // an explicit detach is a best-effort cleanup.
                let _ = backend.detach(port);
                let reason = err.map(|e| e.to_string()).unwrap_or_else(|| "connection closed".into());
                events(AttachEvent::Disconnected { reason });
                let delay = backoff.next_delay();
                events(AttachEvent::Retrying { delay });
                tokio::time::sleep(delay).await;
            }
        }
    }
}

/// Finds a device by busid in a list, for friendlier error messages.
pub fn find_device<'a>(devices: &'a [ExportedDevice], busid: &str) -> Result<&'a ExportedDevice> {
    devices.iter().find(|d| d.info.busid == busid).ok_or_else(|| anyhow!("no device {busid} on server"))
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
