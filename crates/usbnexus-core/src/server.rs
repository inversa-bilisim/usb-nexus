// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! USB Nexus server: authenticates clients and exports local USB devices.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::{Context, Result};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::timeout;
use tokio_rustls::TlsAcceptor;
use tracing::{debug, info, warn};

use crate::backend::ExportBackend;
use crate::control::{ClientMsg, ErrorCode, ExportedDevice, ServerMsg};
use crate::frame::{read_frame, write_frame};
use crate::identity::Identity;
use crate::pairing::{PairingWindow, Pake, Role};
use crate::relay::relay;
use crate::trust::{now_unix, Peer, TrustStore};
use crate::{tls, CONTROL_VERSION};

const HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(15);
const IDLE_TIMEOUT: Duration = Duration::from_secs(120);

pub struct ServerConfig {
    pub name: String,
    pub identity: Identity,
    pub trust: TrustStore,
    pub backend: Arc<dyn ExportBackend>,
}

/// Events reported to the embedding application (CLI or GUI).
#[derive(Debug, Clone)]
pub enum ServerEvent {
    Paired { name: String, fingerprint: String },
    PairingFailed { addr: SocketAddr },
    Exported { busid: String, client: String },
    Released { busid: String, client: String },
}

type EventSink = Arc<dyn Fn(ServerEvent) + Send + Sync>;

struct Inner {
    name: String,
    fingerprint: String,
    acceptor: TlsAcceptor,
    trust: TrustStore,
    backend: Arc<dyn ExportBackend>,
    pairing: Mutex<Option<PairingWindow>>,
    /// busid -> name of the client using it.
    in_use: Mutex<HashMap<String, String>>,
    events: EventSink,
}

#[derive(Clone)]
pub struct Server {
    inner: Arc<Inner>,
}

impl Server {
    pub fn new(cfg: ServerConfig) -> Result<Self> {
        Self::with_events(cfg, Arc::new(|_| {}))
    }

    pub fn with_events(cfg: ServerConfig, events: EventSink) -> Result<Self> {
        let acceptor = TlsAcceptor::from(tls::server_config(&cfg.identity)?);
        Ok(Server {
            inner: Arc::new(Inner {
                name: cfg.name,
                fingerprint: cfg.identity.fingerprint(),
                acceptor,
                trust: cfg.trust,
                backend: cfg.backend,
                pairing: Mutex::new(None),
                in_use: Mutex::new(HashMap::new()),
                events,
            }),
        })
    }

    pub fn fingerprint(&self) -> &str {
        &self.inner.fingerprint
    }

    pub fn name(&self) -> &str {
        &self.inner.name
    }

    /// Opens a pairing window with a random PIN and returns the PIN.
    pub fn open_pairing(&self, duration: Duration) -> String {
        let w = PairingWindow::open(duration);
        let pin = w.pin.clone();
        *self.inner.pairing.lock().unwrap() = Some(w);
        pin
    }

    /// Opens a pairing window with a caller-chosen PIN.
    pub fn open_pairing_with_pin(&self, pin: &str, duration: Duration) {
        *self.inner.pairing.lock().unwrap() = Some(PairingWindow::with_pin(pin, duration));
    }

    pub fn close_pairing(&self) {
        *self.inner.pairing.lock().unwrap() = None;
    }

    /// Devices currently used by clients: busid -> client name.
    pub fn in_use(&self) -> HashMap<String, String> {
        self.inner.in_use.lock().unwrap().clone()
    }

    /// Remaining PIN and lifetime of an open pairing window.
    pub fn pairing(&self) -> Option<(String, Duration)> {
        let guard = self.inner.pairing.lock().unwrap();
        let w = guard.as_ref().filter(|w| w.is_active())?;
        Some((w.pin.clone(), w.expires.saturating_duration_since(std::time::Instant::now())))
    }

    pub fn pairing_open(&self) -> bool {
        self.inner.pairing.lock().unwrap().as_ref().is_some_and(|w| w.is_active())
    }

    /// Accepts connections forever.
    pub async fn serve(&self, listener: TcpListener) -> Result<()> {
        loop {
            let (tcp, addr) = listener.accept().await.context("accepting connection")?;
            let inner = self.inner.clone();
            tokio::spawn(async move {
                if let Err(e) = handle(inner, tcp, addr).await {
                    debug!(%addr, "connection ended: {e:#}");
                }
            });
        }
    }
}

async fn send_error<S>(s: &mut S, code: ErrorCode, message: impl Into<String>) -> Result<()>
where
    S: AsyncWrite + Unpin,
{
    write_frame(s, &ServerMsg::Error { code, message: message.into() }).await
}

async fn handle(inner: Arc<Inner>, tcp: TcpStream, addr: SocketAddr) -> Result<()> {
    tcp.set_nodelay(true)?;
    let mut tls = timeout(HANDSHAKE_TIMEOUT, inner.acceptor.accept(tcp)).await.context("TLS handshake timed out")??;
    let (client_fp, binding) = {
        let conn = tls.get_ref().1;
        (tls::peer_fingerprint(conn)?, tls::session_binding_server(conn)?)
    };

    let hello: ClientMsg = timeout(HANDSHAKE_TIMEOUT, read_frame(&mut tls)).await.context("hello timed out")??;
    let ClientMsg::Hello { version, name: client_name } = hello else {
        return send_error(&mut tls, ErrorCode::Protocol, "expected hello").await;
    };
    if version != CONTROL_VERSION {
        return send_error(&mut tls, ErrorCode::Version, format!("server speaks version {CONTROL_VERSION}")).await;
    }
    let client_name: String = client_name.chars().filter(|c| !c.is_control()).take(64).collect();

    let mut trusted = inner.trust.is_trusted(&client_fp);
    let pairing_open = inner.pairing.lock().unwrap().as_ref().is_some_and(|w| w.is_active());
    write_frame(
        &mut tls,
        &ServerMsg::Welcome {
            version: CONTROL_VERSION,
            name: inner.name.clone(),
            client_trusted: trusted,
            pairing_open,
        },
    )
    .await?;
    debug!(%addr, client = %client_name, trusted, "client connected");

    loop {
        let msg: ClientMsg = match timeout(IDLE_TIMEOUT, read_frame(&mut tls)).await {
            Ok(Ok(m)) => m,
            Ok(Err(_)) | Err(_) => return Ok(()),
        };
        match msg {
            ClientMsg::Hello { .. } => return send_error(&mut tls, ErrorCode::Protocol, "duplicate hello").await,
            ClientMsg::PairStart { spake } => {
                if pair(&inner, &mut tls, &spake, &binding, &client_fp, &client_name, addr).await? {
                    trusted = true;
                } else {
                    return Ok(());
                }
            }
            ClientMsg::PairConfirm { .. } => {
                return send_error(&mut tls, ErrorCode::Protocol, "unexpected confirmation").await
            }
            _ if !trusted => return send_error(&mut tls, ErrorCode::NotTrusted, "pair with this server first").await,
            ClientMsg::ListDevices => {
                let devices = match inner.backend.list() {
                    Ok(d) => d,
                    Err(e) => {
                        warn!("listing devices failed: {e:#}");
                        send_error(&mut tls, ErrorCode::Internal, "could not list devices").await?;
                        continue;
                    }
                };
                let in_use = inner.in_use.lock().unwrap().clone();
                let devices = devices
                    .into_iter()
                    .map(|d| ExportedDevice {
                        in_use: in_use.contains_key(&d.info.busid),
                        info: d.info,
                        product: d.product,
                        manufacturer: d.manufacturer,
                    })
                    .collect();
                write_frame(&mut tls, &ServerMsg::Devices { devices }).await?;
            }
            ClientMsg::Import { busid } => return import(&inner, tls, &busid, &client_name).await,
        }
    }
}

/// Runs the server side of PIN pairing. Returns whether pairing succeeded.
async fn pair<S>(
    inner: &Inner,
    tls: &mut S,
    client_msg: &[u8],
    binding: &[u8],
    client_fp: &str,
    client_name: &str,
    addr: SocketAddr,
) -> Result<bool>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let pin = inner.pairing.lock().unwrap().as_ref().filter(|w| w.is_active()).map(|w| w.pin.clone());
    let Some(pin) = pin else {
        send_error(tls, ErrorCode::PairingClosed, "pairing is not open").await?;
        return Ok(false);
    };

    let (pake, msg) = Pake::start(Role::Server, &pin);
    write_frame(tls, &ServerMsg::PairStart { spake: msg }).await?;
    let confirm = pake.finish(client_msg, binding);
    let reply: ClientMsg = timeout(HANDSHAKE_TIMEOUT, read_frame(tls)).await.context("pairing timed out")??;
    let ok = match (&confirm, reply) {
        (Ok(c), ClientMsg::PairConfirm { mac }) => c.verify_peer(&mac),
        _ => false,
    };

    if !ok {
        if let Some(w) = inner.pairing.lock().unwrap().as_mut() {
            w.failures += 1;
        }
        warn!(%addr, "pairing attempt failed");
        (inner.events)(ServerEvent::PairingFailed { addr });
        send_error(tls, ErrorCode::PairingFailed, "pairing failed").await?;
        return Ok(false);
    }

    inner.trust.add(Peer {
        name: client_name.to_string(),
        fingerprint: client_fp.to_string(),
        last_addr: None,
        paired_at: now_unix(),
    })?;
    // One successful pairing per window.
    *inner.pairing.lock().unwrap() = None;
    let own = confirm.expect("checked above").own_mac();
    write_frame(tls, &ServerMsg::PairConfirm { mac: own }).await?;
    info!(client = %client_name, "paired new client");
    (inner.events)(ServerEvent::Paired { name: client_name.to_string(), fingerprint: client_fp.to_string() });
    Ok(true)
}

/// Removes a busid from the in-use set when dropped.
struct InUse<'a> {
    set: &'a Mutex<HashMap<String, String>>,
    busid: String,
}

impl Drop for InUse<'_> {
    fn drop(&mut self) {
        self.set.lock().unwrap().remove(&self.busid);
    }
}

async fn import<S>(inner: &Inner, mut tls: S, busid: &str, client_name: &str) -> Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let device = match inner.backend.list()?.into_iter().find(|d| d.info.busid == busid) {
        Some(d) => d,
        None => return send_error(&mut tls, ErrorCode::NoSuchDevice, format!("{busid} is not exported")).await,
    };
    let claimed = {
        let mut in_use = inner.in_use.lock().unwrap();
        !in_use.contains_key(busid) && in_use.insert(busid.to_string(), client_name.to_string()).is_none()
    };
    if !claimed {
        return send_error(&mut tls, ErrorCode::DeviceBusy, format!("{busid} is in use")).await;
    }
    let _guard = InUse { set: &inner.in_use, busid: busid.to_string() };

    let local = match inner.backend.export(busid).await {
        Ok(s) => s,
        Err(e) => {
            warn!(busid, "export failed: {e:#}");
            let code = match e.downcast_ref::<crate::api::ApiError>().map(|a| a.code.as_str()) {
                Some("device_busy") => ErrorCode::DeviceBusy,
                Some("no_such_device") => ErrorCode::NoSuchDevice,
                _ => ErrorCode::Internal,
            };
            return send_error(&mut tls, code, format!("could not export {busid}")).await;
        }
    };
    write_frame(&mut tls, &ServerMsg::Imported { device: device.info }).await?;
    info!(busid, client = %client_name, "device exported");
    (inner.events)(ServerEvent::Exported { busid: busid.to_string(), client: client_name.to_string() });

    let end = relay(tls, local).await;
    info!(busid, client = %client_name, ?end, "device released");
    (inner.events)(ServerEvent::Released { busid: busid.to_string(), client: client_name.to_string() });
    Ok(())
}
