// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! USB Nexus server: authenticates clients and exports local USB devices.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Notify;
use tokio::time::timeout;
use tokio_rustls::TlsAcceptor;
use tracing::{debug, info, warn};

use crate::backend::{ExportBackend, Offered};
use crate::control::{ClientMsg, ErrorCode, ExportedDevice, ServerMsg};
use crate::device_id::DeviceId;
use crate::frame::{read_frame, write_frame};
use crate::identity::Identity;
use crate::pairing::{PairingWindow, Pake, Role};
use crate::relay::{relay, Activity, Watched};
use crate::trust::{now_unix, Peer, TrustStore};
use crate::{tls, CONTROL_VERSION};

const HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(15);
const IDLE_TIMEOUT: Duration = Duration::from_secs(120);
/// A waiting client asks again every few seconds; one that stopped asking
/// for this long has left the queue.
const QUEUE_TTL: Duration = Duration::from_secs(15);
/// How long a device that became free is kept for the first in the queue.
const RESERVE_FOR: Duration = Duration::from_secs(10);
/// How often an idle session is checked for handover.
const HANDOVER_CHECK: Duration = Duration::from_secs(1);

pub struct ServerConfig {
    pub name: String,
    pub identity: Identity,
    pub trust: TrustStore,
    pub backend: Arc<dyn ExportBackend>,
}

/// A computer and a device, as reported in [`ServerEvent`]s.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceUse {
    /// Device identity (see [`DeviceId`]).
    pub device: String,
    /// Current bus id, while the device is plugged in.
    pub busid: Option<String>,
    /// Human readable device name, when known.
    pub device_name: Option<String>,
    /// Name the client computer gave itself.
    pub client: String,
    pub fingerprint: String,
    pub addr: SocketAddr,
}

/// Events reported to the embedding application (CLI or GUI).
#[derive(Debug, Clone)]
pub enum ServerEvent {
    Paired {
        name: String,
        fingerprint: String,
        addr: SocketAddr,
    },
    PairingFailed {
        addr: SocketAddr,
        name: String,
        fingerprint: String,
    },
    Exported(DeviceUse),
    Released {
        usage: DeviceUse,
        duration: Duration,
    },
    /// A paired computer asked for a device it may not use.
    Denied(DeviceUse),
}

/// A device being used by a client.
struct ActiveUse {
    client: String,
    fingerprint: String,
    since: std::time::SystemTime,
    /// Ends the session when notified.
    stop: Arc<Notify>,
}

/// A client waiting for a busy device.
struct Waiter {
    client: String,
    fingerprint: String,
    last_asked: Instant,
}

/// Who uses a device, as reported to interfaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceSession {
    pub client: String,
    pub fingerprint: String,
    pub since: std::time::SystemTime,
}

/// Device queues: waiting clients in order, and devices kept for the first
/// of them after they became free.
#[derive(Default)]
struct Queues {
    waiting: HashMap<String, Vec<Waiter>>,
    /// Device identity -> (fingerprint, until).
    reserved: HashMap<String, (String, Instant)>,
}

impl Queues {
    fn prune(&mut self, now: Instant) {
        for q in self.waiting.values_mut() {
            q.retain(|w| now.duration_since(w.last_asked) < QUEUE_TTL);
        }
        self.waiting.retain(|_, q| !q.is_empty());
        self.reserved.retain(|_, (_, until)| *until > now);
    }
}

type EventSink = Arc<dyn Fn(ServerEvent) + Send + Sync>;

struct Inner {
    name: String,
    fingerprint: String,
    acceptor: TlsAcceptor,
    trust: TrustStore,
    backend: Arc<dyn ExportBackend>,
    pairing: Mutex<Option<PairingWindow>>,
    /// Device identity -> the client using it. Lock before `queues`.
    in_use: Mutex<HashMap<String, ActiveUse>>,
    queues: Mutex<Queues>,
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
                queues: Mutex::default(),
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

    /// Devices currently used by clients: device identity -> (client name,
    /// client fingerprint).
    pub fn in_use(&self) -> HashMap<String, (String, String)> {
        let in_use = self.inner.in_use.lock().unwrap();
        in_use.iter().map(|(id, u)| (id.clone(), (u.client.clone(), u.fingerprint.clone()))).collect()
    }

    /// Who uses which device (by identity), and since when.
    pub fn sessions(&self) -> HashMap<String, DeviceSession> {
        let in_use = self.inner.in_use.lock().unwrap();
        in_use
            .iter()
            .map(|(id, u)| {
                let s = DeviceSession { client: u.client.clone(), fingerprint: u.fingerprint.clone(), since: u.since };
                (id.clone(), s)
            })
            .collect()
    }

    /// Computers waiting for a device, first in line first: (name,
    /// fingerprint).
    pub fn queue(&self, device: &str) -> Vec<(String, String)> {
        let mut queues = self.inner.queues.lock().unwrap();
        queues.prune(Instant::now());
        queues
            .waiting
            .get(device)
            .map(|q| q.iter().map(|w| (w.client.clone(), w.fingerprint.clone())).collect())
            .unwrap_or_default()
    }

    /// Ends the sessions for which `stop(device identity, client
    /// fingerprint)` is true, e.g. after permission was revoked. Returns how
    /// many were ended.
    pub fn disconnect(&self, stop: impl Fn(&str, &str) -> bool) -> usize {
        let in_use = self.inner.in_use.lock().unwrap();
        let mut n = 0;
        for (id, u) in in_use.iter() {
            if stop(id, &u.fingerprint) {
                u.stop.notify_one();
                n += 1;
            }
        }
        n
    }

    /// Ends sessions that the current sharing and access settings no longer
    /// allow (device unshared, permission revoked, computer forgotten).
    pub fn enforce(&self) -> Result<usize> {
        let offered = self.inner.backend.list()?;
        let trust = &self.inner.trust;
        Ok(self.disconnect(|id, fp| {
            let wanted = DeviceId::parse(id);
            let allowed = offered.iter().find(|o| o.is(&wanted)).is_some_and(|o| o.allowed.allows(fp));
            !allowed || !trust.is_trusted(fp)
        }))
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
    write_frame(s, &ServerMsg::Error { code, message: message.into(), queue_position: None }).await
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
                let devices = {
                    let in_use = inner.in_use.lock().unwrap();
                    devices.into_iter().map(|o| exported(o, &client_fp, &in_use)).collect()
                };
                write_frame(&mut tls, &ServerMsg::Devices { devices }).await?;
            }
            ClientMsg::Import { device } => {
                let client = Client { name: &client_name, fingerprint: &client_fp, addr };
                return import(&inner, tls, &device, client).await;
            }
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
        (inner.events)(ServerEvent::PairingFailed {
            addr,
            name: client_name.to_string(),
            fingerprint: client_fp.to_string(),
        });
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
    (inner.events)(ServerEvent::Paired { name: client_name.to_string(), fingerprint: client_fp.to_string(), addr });
    Ok(true)
}

/// Builds the listing entry of an offered device for one client.
fn exported(o: Offered, client_fp: &str, in_use: &HashMap<String, ActiveUse>) -> ExportedDevice {
    let id = o.id.to_string();
    let present = o.device.is_some();
    let info = match o.device {
        Some(d) => d.info,
        None => {
            let (vendor, product) = o.id.ids().unwrap_or_default();
            usbnexus_proto::DeviceInfo {
                path: String::new(),
                busid: o.id.busid().unwrap_or_default().to_string(),
                busnum: 0,
                devnum: 0,
                speed: usbnexus_proto::Speed::Unknown,
                id_vendor: vendor,
                id_product: product,
                bcd_device: 0,
                device_class: 0,
                device_subclass: 0,
                device_protocol: 0,
                configuration_value: 0,
                num_configurations: 0,
                interfaces: vec![],
            }
        }
    };
    ExportedDevice {
        in_use: in_use.contains_key(&id),
        allowed: o.allowed.allows(client_fp),
        id,
        info,
        present,
        product: o.product,
        manufacturer: o.manufacturer,
    }
}

/// The client of a connection.
#[derive(Clone, Copy)]
struct Client<'a> {
    name: &'a str,
    fingerprint: &'a str,
    addr: SocketAddr,
}

/// Removes a device from the in-use set when dropped, and keeps it for the
/// first waiting client for a moment.
struct InUse<'a> {
    inner: &'a Inner,
    id: String,
}

impl Drop for InUse<'_> {
    fn drop(&mut self) {
        let mut in_use = self.inner.in_use.lock().unwrap();
        in_use.remove(&self.id);
        let mut queues = self.inner.queues.lock().unwrap();
        let now = Instant::now();
        queues.prune(now);
        if let Some(next) = queues.waiting.get(&self.id).and_then(|q| q.first()) {
            let reservation = (next.fingerprint.clone(), now + RESERVE_FOR);
            queues.reserved.insert(self.id.clone(), reservation);
        }
    }
}

/// Resolves once the session should end so the device goes to the next
/// computer: someone waits and the device has been idle for its handover
/// time (read again every check, so setting changes apply at once).
async fn handover_due(inner: &Inner, id: &str, activity: &Activity) {
    loop {
        tokio::time::sleep(HANDOVER_CHECK).await;
        let idle = inner.backend.list().ok().and_then(|o| o.into_iter().find(|o| o.id.to_string() == id)?.handover);
        let Some(idle) = idle else { continue };
        if activity.idle() < idle {
            continue;
        }
        let mut queues = inner.queues.lock().unwrap();
        queues.prune(Instant::now());
        if queues.waiting.get(id).is_some_and(|q| !q.is_empty()) {
            return;
        }
    }
}

async fn import<S>(inner: &Inner, mut tls: S, wanted: &str, client: Client<'_>) -> Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let wanted_id = DeviceId::parse(wanted);
    let Some(offered) = inner.backend.list()?.into_iter().find(|o| o.is(&wanted_id)) else {
        return send_error(&mut tls, ErrorCode::NoSuchDevice, format!("{wanted} is not shared")).await;
    };
    let id = offered.id.to_string();
    let usage = DeviceUse {
        device: id.clone(),
        busid: offered.device.as_ref().map(|d| d.info.busid.clone()),
        device_name: [offered.manufacturer.clone(), offered.product.clone()]
            .into_iter()
            .flatten()
            .reduce(|a, b| format!("{a} {b}")),
        client: client.name.to_string(),
        fingerprint: client.fingerprint.to_string(),
        addr: client.addr,
    };
    if !offered.allowed.allows(client.fingerprint) {
        info!(device = %id, client = %client.name, "access denied");
        (inner.events)(ServerEvent::Denied(usage));
        return send_error(&mut tls, ErrorCode::AccessDenied, format!("not allowed to use {wanted}")).await;
    }
    let Some(device) = offered.device else {
        return send_error(&mut tls, ErrorCode::NoSuchDevice, format!("{wanted} is not plugged in")).await;
    };
    let busid = device.info.busid.clone();
    let stop = Arc::new(Notify::new());
    // Claim the device, or join (or stay in) its queue.
    let claimed = {
        let mut in_use = inner.in_use.lock().unwrap();
        let mut queues = inner.queues.lock().unwrap();
        let now = Instant::now();
        queues.prune(now);
        let kept_for_other = queues.reserved.get(&id).is_some_and(|(fp, _)| fp != client.fingerprint);
        if in_use.contains_key(&id) || kept_for_other {
            let q = queues.waiting.entry(id.clone()).or_default();
            let pos = match q.iter().position(|w| w.fingerprint == client.fingerprint) {
                Some(i) => i,
                None => {
                    q.push(Waiter {
                        client: client.name.to_string(),
                        fingerprint: client.fingerprint.to_string(),
                        last_asked: now,
                    });
                    q.len() - 1
                }
            };
            q[pos].last_asked = now;
            Err(pos as u32 + 1)
        } else {
            if let Some(q) = queues.waiting.get_mut(&id) {
                q.retain(|w| w.fingerprint != client.fingerprint);
            }
            queues.reserved.remove(&id);
            let active = ActiveUse {
                client: client.name.to_string(),
                fingerprint: client.fingerprint.to_string(),
                since: std::time::SystemTime::now(),
                stop: stop.clone(),
            };
            in_use.insert(id.clone(), active);
            Ok(())
        }
    };
    if let Err(position) = claimed {
        let msg = ServerMsg::Error {
            code: ErrorCode::DeviceBusy,
            message: format!("{wanted} is in use"),
            queue_position: Some(position),
        };
        return write_frame(&mut tls, &msg).await;
    }
    let _guard = InUse { inner, id: id.clone() };

    let local = match inner.backend.export(&busid).await {
        Ok(s) => s,
        Err(e) => {
            warn!(busid, "export failed: {e:#}");
            let code = match e.downcast_ref::<crate::api::ApiError>().map(|a| a.code.as_str()) {
                Some("device_busy") => ErrorCode::DeviceBusy,
                Some("no_such_device") => ErrorCode::NoSuchDevice,
                _ => ErrorCode::Internal,
            };
            return send_error(&mut tls, code, format!("could not export {wanted}")).await;
        }
    };
    write_frame(&mut tls, &ServerMsg::Imported { device: device.info, id: Some(id.clone()) }).await?;
    info!(busid, device = %id, client = %client.name, "device exported");
    (inner.events)(ServerEvent::Exported(usage.clone()));
    let since = Instant::now();

    let activity = Activity::default();
    tokio::select! {
        end = relay(tls, Watched::new(local, activity.clone())) => {
            info!(busid, client = %client.name, ?end, "device released")
        }
        _ = handover_due(inner, &id, &activity) => {
            info!(busid, client = %client.name, "idle; handed over to the next computer")
        }
        // Dropping the relay closes both sockets.
        _ = stop.notified() => info!(busid, client = %client.name, "session ended by the server"),
    }
    (inner.events)(ServerEvent::Released { usage, duration: since.elapsed() });
    Ok(())
}
