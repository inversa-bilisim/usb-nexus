// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! The USB Nexus service: one long-running process that shares local devices,
//! keeps remote devices attached, and answers the local API.
//!
//! Its settings (shared devices, access policy, attachments) are saved in
//! `config.json` in the state directory and restored on start-up, so shares
//! and attachments survive reboots. Configurations of older versions, which
//! refer to devices by bus id, are migrated as the devices are seen.
//!
//! Shared devices are looked up every few seconds, so a device that is
//! unplugged stays shared and is offered again when it comes back.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio::task::JoinHandle;
use tracing::{debug, warn};

use crate::access::{DeviceAccess, Policy};
use crate::api::{
    ApiError, AttachState, AttachmentView, DiscoveredView, LocalDeviceView, PairingView, PeersView, RemoteDeviceView,
    Request, Response, Roles, StatusView, UsageView, WebStatusView,
};
use crate::backend::{DeviceHost, ImportBackend, SharedDevice, SharedExport};
use crate::client::{self, AttachEvent, ClientConfig, Target};
use crate::device_id::DeviceId;
use crate::discovery;
use crate::identity::Identity;
use crate::server::{DeviceUse, Server, ServerConfig, ServerEvent};
use crate::trust::TrustStore;
use crate::usage::{UsageEntry, UsageKind, UsageLog, DEFAULT_RETENTION_DAYS};
use crate::web::{self, WebServer, WebSettings};

/// How often connected devices are looked up.
const POLL_INTERVAL: Duration = Duration::from_secs(3);
/// How often old usage log entries are dropped.
const PRUNE_INTERVAL: Duration = Duration::from_secs(3600);
/// Repeated refusals of the same device to the same computer are recorded
/// at most this often.
const DENIED_LOG_INTERVAL: Duration = Duration::from_secs(3600);

pub struct DaemonOptions {
    pub state_dir: PathBuf,
    pub name: String,
    pub listen: String,
    pub mdns: bool,
    pub host: Arc<dyn DeviceHost>,
    pub import: Arc<dyn ImportBackend>,
    /// Optional observer of server activity (pairings, exports).
    pub events: Option<Arc<dyn Fn(ServerEvent) + Send + Sync>>,
}

fn default_retention() -> u32 {
    DEFAULT_RETENTION_DAYS
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    /// Shared devices (older versions saved bus ids).
    #[serde(default)]
    shared: Vec<SharedDevice>,
    /// Server-wide access policy; `None` until chosen (then: open).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    policy: Option<Policy>,
    #[serde(default)]
    attachments: Vec<SavedAttachment>,
    #[serde(default)]
    web: WebSettings,
    #[serde(default = "default_retention")]
    usage_retention_days: u32,
    /// Set by the installer; `None` (older versions): both.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    roles: Option<Roles>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            shared: vec![],
            policy: None,
            attachments: vec![],
            web: WebSettings::default(),
            usage_retention_days: DEFAULT_RETENTION_DAYS,
            roles: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct SavedAttachment {
    server: String,
    /// Device identity on the server (older versions saved a bus id).
    #[serde(alias = "busid")]
    device: String,
    /// Device name as listed by the server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

/// (server fingerprint, device identity)
type Key = (String, String);

struct Slot {
    state: Arc<Mutex<AttachState>>,
    /// Vendor id, product id and bus id when last attached.
    seen: Arc<Mutex<Option<(u16, u16, String)>>>,
    task: JoinHandle<()>,
}

struct Inner {
    name: String,
    fingerprint: String,
    listen: String,
    config_path: PathBuf,
    config: Mutex<Config>,
    server: Server,
    export: Arc<SharedExport>,
    client: ClientConfig,
    clients: TrustStore,
    import: Arc<dyn ImportBackend>,
    attachments: Mutex<HashMap<Key, Slot>>,
    usage: Arc<UsageLog>,
    poller: Mutex<Option<JoinHandle<()>>>,
    _mdns: Option<discovery::Advertiser>,
    state_dir: PathBuf,
    /// Running web interface, or why it could not start.
    web: tokio::sync::Mutex<Result<Option<WebServer>, String>>,
}

#[derive(Clone)]
pub struct Daemon {
    inner: Arc<Inner>,
}

fn ok<T: Serialize>(v: T) -> Response {
    match serde_json::to_value(v) {
        Ok(data) => Response::Ok { data },
        Err(e) => Response::Error { error: ApiError::new("internal", e.to_string()) },
    }
}

fn err(e: anyhow::Error) -> Response {
    Response::Error { error: ApiError::from_anyhow(&e) }
}

/// Usage log entry for a server event.
fn usage_entry(ev: &ServerEvent) -> UsageEntry {
    let with_use = |kind, u: &DeviceUse| UsageEntry {
        computer: Some(u.client.clone()),
        fingerprint: Some(u.fingerprint.clone()),
        address: Some(u.addr.ip().to_string()),
        device: Some(u.device.clone()),
        device_name: u.device_name.clone(),
        ..UsageEntry::now(kind)
    };
    match ev {
        ServerEvent::Paired { name, fingerprint, addr } => UsageEntry {
            computer: Some(name.clone()),
            fingerprint: Some(fingerprint.clone()),
            address: Some(addr.ip().to_string()),
            ..UsageEntry::now(UsageKind::Paired)
        },
        ServerEvent::PairingFailed { addr, name, fingerprint } => UsageEntry {
            computer: Some(name.clone()),
            fingerprint: Some(fingerprint.clone()),
            address: Some(addr.ip().to_string()),
            ..UsageEntry::now(UsageKind::PairingFailed)
        },
        ServerEvent::Exported(u) => with_use(UsageKind::Attached, u),
        ServerEvent::Released { usage, duration } => {
            UsageEntry { duration_secs: Some(duration.as_secs()), ..with_use(UsageKind::Detached, usage) }
        }
        ServerEvent::Denied(u) => with_use(UsageKind::Denied, u),
    }
}

/// Whether a saved attachment key refers to a device listed by a server.
fn refers_to(saved: &str, id: &str, busid: Option<&str>) -> bool {
    saved == id || (DeviceId::parse(saved).is_legacy() && Some(saved) == busid)
}

fn read_config(path: &Path) -> Result<Config> {
    match std::fs::read(path) {
        Ok(data) => serde_json::from_slice(&data).with_context(|| format!("parsing {}", path.display())),
        Err(_) => Ok(Config::default()),
    }
}

fn write_config(path: &Path, config: &Config) -> Result<()> {
    let data = serde_json::to_vec_pretty(config)?;
    let tmp = path.with_extension("tmp");
    // Holds the web password hash: owner-only.
    crate::identity::write_private(&tmp, &data)?;
    std::fs::rename(&tmp, path)?;
    Ok(())
}

/// Settings an installer applies while the service is stopped; `None`
/// fields keep their current value.
#[derive(Debug, Default)]
pub struct Setup {
    pub roles: Option<Roles>,
    /// Turns the web interface on or off.
    pub web_enabled: Option<bool>,
    pub web_lan: Option<bool>,
    pub web_port: Option<u16>,
    /// New web password (at least 8 characters).
    pub web_password: Option<String>,
}

/// Roles saved in `state_dir`; both if none were chosen.
pub fn saved_roles(state_dir: &Path) -> Roles {
    read_config(&state_dir.join("config.json")).ok().and_then(|c| c.roles).unwrap_or_default()
}

/// Web settings saved in `state_dir`.
pub fn saved_web(state_dir: &Path) -> WebSettings {
    read_config(&state_dir.join("config.json")).map(|c| c.web).unwrap_or_default()
}

/// Writes `setup` into the configuration in `state_dir`, for the next
/// start of the service. Enabling the web interface needs a password,
/// given now or set before.
pub fn apply_setup(state_dir: &Path, setup: Setup) -> Result<()> {
    std::fs::create_dir_all(state_dir).with_context(|| format!("creating {}", state_dir.display()))?;
    let path = state_dir.join("config.json");
    let mut config = read_config(&path)?;
    if let Some(roles) = setup.roles {
        config.roles = Some(roles);
    }
    let web = &mut config.web;
    if let Some(password) = setup.web_password {
        web.password_hash = Some(web::hash_password(&password)?);
    }
    if let Some(lan) = setup.web_lan {
        web.lan = lan;
    }
    if let Some(port) = setup.web_port {
        web.port = Some(port);
    }
    if let Some(enabled) = setup.web_enabled {
        if enabled && web.password_hash.is_none() {
            return Err(ApiError::new("password_required", "set a web password first").into());
        }
        web.enabled = enabled;
    }
    write_config(&path, &config)
}

impl Daemon {
    /// Loads state, starts the server and restores saved attachments.
    pub async fn start(opts: DaemonOptions) -> Result<Daemon> {
        std::fs::create_dir_all(&opts.state_dir).with_context(|| format!("creating {}", opts.state_dir.display()))?;
        let identity = Identity::load_or_create(&opts.state_dir, &opts.name)?;
        let clients = TrustStore::load(&opts.state_dir.join("trusted-clients.json"))?;
        let servers = TrustStore::load(&opts.state_dir.join("trusted-servers.json"))?;
        let config_path = opts.state_dir.join("config.json");
        let config = read_config(&config_path)?;
        let usage = Arc::new(UsageLog::open(&opts.state_dir.join("usage.log"), config.usage_retention_days));

        let export =
            Arc::new(SharedExport::new(opts.host.clone(), config.shared.clone(), config.policy.unwrap_or_default()));
        let events = {
            let (log, forward) = (usage.clone(), opts.events.clone());
            // A client waiting for permission asks again and again; its
            // refusals are recorded once per DENIED_LOG_INTERVAL.
            let denied: Mutex<HashMap<(String, String), Instant>> = Mutex::default();
            Arc::new(move |ev: ServerEvent| {
                let record = match &ev {
                    ServerEvent::Denied(u) => {
                        let mut denied = denied.lock().unwrap();
                        denied.retain(|_, t| t.elapsed() < DENIED_LOG_INTERVAL);
                        denied.insert((u.fingerprint.clone(), u.device.clone()), Instant::now()).is_none()
                    }
                    _ => true,
                };
                if record {
                    log.record(usage_entry(&ev));
                }
                if let Some(f) = &forward {
                    f(ev);
                }
            })
        };
        let server = Server::with_events(
            ServerConfig {
                name: opts.name.clone(),
                identity: identity.clone(),
                trust: clients.clone(),
                backend: export.clone(),
            },
            events,
        )?;
        let listener =
            TcpListener::bind(&opts.listen).await.with_context(|| format!("listening on {}", opts.listen))?;
        let listen = listener.local_addr()?.to_string();
        let port = listener.local_addr()?.port();
        {
            let server = server.clone();
            tokio::spawn(async move {
                if let Err(e) = server.serve(listener).await {
                    warn!("server stopped: {e:#}");
                }
            });
        }
        let mdns = if opts.mdns {
            discovery::advertise(&opts.name, port, server.fingerprint())
                .map_err(|e| warn!("mDNS unavailable: {e:#}"))
                .ok()
        } else {
            None
        };

        let saved = config.attachments.clone();
        let daemon = Daemon {
            inner: Arc::new(Inner {
                name: opts.name.clone(),
                fingerprint: identity.fingerprint(),
                listen,
                config_path,
                config: Mutex::new(config),
                server,
                export,
                client: ClientConfig { name: opts.name, identity, trust: servers },
                clients,
                import: opts.import,
                attachments: Mutex::new(HashMap::new()),
                usage,
                poller: Mutex::new(None),
                _mdns: mdns,
                state_dir: opts.state_dir.clone(),
                web: tokio::sync::Mutex::new(Ok(None)),
            }),
        };
        {
            let d = daemon.clone();
            tokio::task::spawn_blocking(move || d.poll_devices()).await?;
        }
        *daemon.inner.poller.lock().unwrap() = Some(daemon.spawn_poller());
        for a in saved {
            daemon.spawn_attachment(&a.server, &a.device);
        }
        daemon.restart_web().await;
        Ok(daemon)
    }

    pub fn server(&self) -> &Server {
        &self.inner.server
    }

    /// Stops attachments and returns shared devices to their drivers.
    pub fn shutdown(&self) {
        if let Ok(mut web) = self.inner.web.try_lock() {
            *web = Ok(None);
        }
        if let Some(p) = self.inner.poller.lock().unwrap().take() {
            p.abort();
        }
        for (_, slot) in self.inner.attachments.lock().unwrap().drain() {
            slot.task.abort();
        }
        let Ok((_, resolved)) = self.inner.export.snapshot() else { return };
        for busid in resolved.into_iter().filter_map(|r| r.device).map(|d| d.info.busid) {
            if let Err(e) = self.inner.export.host().release(&busid) {
                warn!(busid, "could not release device: {e:#}");
            }
        }
    }

    fn save(&self) -> Result<()> {
        // The lock is held until the file is replaced, so saves from the
        // device poller and API requests do not interleave.
        let config = self.inner.config.lock().unwrap();
        write_config(&self.inner.config_path, &config)
    }

    /// Saves the shared devices after a change and ends sessions the new
    /// settings no longer allow.
    fn shares_changed(&self) -> Result<()> {
        self.inner.config.lock().unwrap().shared = self.inner.export.shared();
        self.save()?;
        if let Err(e) = self.inner.server.enforce() {
            warn!("could not check active sessions: {e:#}");
        }
        Ok(())
    }

    /// Looks at the connected devices: remembers names of shared devices and
    /// migrates bus ids of older configurations.
    fn poll_devices(&self) {
        match self.inner.export.refresh() {
            Ok(true) => {
                if let Err(e) = self.shares_changed() {
                    warn!("could not save the configuration: {e:#}");
                }
            }
            Ok(false) => {}
            Err(e) => debug!("listing devices failed: {e:#}"),
        }
    }

    fn spawn_poller(&self) -> JoinHandle<()> {
        let weak = Arc::downgrade(&self.inner);
        tokio::spawn(async move {
            let mut last_prune = tokio::time::Instant::now();
            loop {
                tokio::time::sleep(POLL_INTERVAL).await;
                let Some(inner) = weak.upgrade() else { return };
                let daemon = Daemon { inner };
                // Listing devices may block (e.g. opening hubs on Windows).
                let d = daemon.clone();
                let _ = tokio::task::spawn_blocking(move || d.poll_devices()).await;
                if last_prune.elapsed() >= PRUNE_INTERVAL {
                    last_prune = tokio::time::Instant::now();
                    let usage = daemon.inner.usage.clone();
                    let _ = tokio::task::spawn_blocking(move || usage.prune()).await;
                }
            }
        })
    }

    fn spawn_attachment(&self, server: &str, device: &str) {
        let key = (server.to_string(), device.to_string());
        let mut map = self.inner.attachments.lock().unwrap();
        if map.get(&key).is_some_and(|s| !s.task.is_finished()) {
            return;
        }
        let state = Arc::new(Mutex::new(AttachState::Connecting));
        let seen = Arc::new(Mutex::new(None));
        let (cfg, import) = (self.inner.client.clone(), self.inner.import.clone());
        let (st, seen2) = (state.clone(), seen.clone());
        let weak = Arc::downgrade(&self.inner);
        let (fp, dev) = key.clone();
        let task = tokio::spawn(async move {
            let target = Target::Peer { fingerprint: fp.clone() };
            // The key changes when an attachment saved by bus id learns the
            // device identity.
            let current = Mutex::new(dev.clone());
            let events = |ev: AttachEvent| {
                let next = match ev {
                    AttachEvent::Connecting { .. } => AttachState::Connecting,
                    AttachEvent::Attached { port, device, id } => {
                        *seen2.lock().unwrap() = Some((device.id_vendor, device.id_product, device.busid.clone()));
                        let mut cur = current.lock().unwrap();
                        if *cur != id {
                            if let Some(inner) = weak.upgrade() {
                                Daemon { inner }.rekey(&fp, &cur, &id);
                            }
                            cur.clone_from(&id);
                        }
                        AttachState::Attached { port }
                    }
                    AttachEvent::Retrying { delay } => {
                        let seconds = delay.as_secs_f64().ceil() as u64;
                        match &*st.lock().unwrap() {
                            AttachState::Waiting { error, .. } => {
                                AttachState::Waiting { seconds, error: error.clone() }
                            }
                            AttachState::Retrying { error, .. } => {
                                AttachState::Retrying { seconds, error: error.clone() }
                            }
                            _ => AttachState::Retrying { seconds, error: ApiError::new("connection_lost", "") },
                        }
                    }
                    AttachEvent::Disconnected { error } => match error.code.as_str() {
                        "no_such_device" | "access_denied" => AttachState::Waiting { seconds: 0, error },
                        _ => AttachState::Retrying { seconds: 0, error },
                    },
                    AttachEvent::Detached => AttachState::Stopped,
                };
                *st.lock().unwrap() = next;
            };
            let result = client::attach_forever(&cfg, &target, &dev, import, events).await;
            *st.lock().unwrap() = match result {
                Ok(()) => AttachState::Stopped,
                Err(e) => AttachState::Failed { error: ApiError::from_anyhow(&e) },
            };
        });
        map.insert(key, Slot { state, seen, task });
    }

    /// Renames an attachment saved by bus id once the device identity is known.
    fn rekey(&self, server: &str, old: &str, new: &str) {
        {
            let mut map = self.inner.attachments.lock().unwrap();
            let new_key = (server.to_string(), new.to_string());
            if map.contains_key(&new_key) {
                return;
            }
            if let Some(slot) = map.remove(&(server.to_string(), old.to_string())) {
                map.insert(new_key, slot);
            }
        }
        {
            let mut cfg = self.inner.config.lock().unwrap();
            for a in cfg.attachments.iter_mut().filter(|a| a.server == server && a.device == old) {
                a.device = new.to_string();
            }
        }
        if let Err(e) = self.save() {
            warn!("could not save the configuration: {e:#}");
        }
    }

    fn remove_attachment(&self, server: &str, device: &str) -> Result<()> {
        let busid = DeviceId::parse(device).busid().map(str::to_string);
        let matches = |s: &str, d: &str| s == server && refers_to(d, device, busid.as_deref());
        self.inner.attachments.lock().unwrap().retain(|(s, d), slot| {
            let remove = matches(s, d);
            if remove {
                // Dropping the relay closes the socket; the OS detaches the device.
                slot.task.abort();
            }
            !remove
        });
        self.inner.config.lock().unwrap().attachments.retain(|a| !matches(&a.server, &a.device));
        self.save()
    }

    /// Saves the names of attached devices listed by `server`, so they can
    /// be shown while the server is unreachable.
    fn remember_names(&self, server: &str, devices: &[RemoteDeviceView]) {
        let mut changed = false;
        {
            let mut cfg = self.inner.config.lock().unwrap();
            for a in cfg.attachments.iter_mut().filter(|a| a.server == server) {
                let Some(d) = devices.iter().find(|d| refers_to(&a.device, &d.id, d.busid.as_deref())) else {
                    continue;
                };
                let name =
                    [d.manufacturer.clone(), d.product.clone()].into_iter().flatten().reduce(|a, b| a + " " + &b);
                if name.is_some() && a.name != name {
                    a.name = name;
                    changed = true;
                }
            }
        }
        if changed {
            if let Err(e) = self.save() {
                warn!("could not save the configuration: {e:#}");
            }
        }
    }

    /// Devices of `server` attached (or being attached) here.
    fn attached_here(&self, server: &str) -> Vec<String> {
        self.inner
            .attachments
            .lock()
            .unwrap()
            .iter()
            .filter(|((s, _), slot)| s == server && !matches!(*slot.state.lock().unwrap(), AttachState::Stopped))
            .map(|((_, d), _)| d.clone())
            .collect()
    }

    /// (Re)starts or stops the web interface according to the settings.
    async fn restart_web(&self) {
        let settings = self.inner.config.lock().unwrap().web.clone();
        let mut slot = self.inner.web.lock().await;
        // Stop the old one (and drop its sessions) first.
        if let Ok(Some(old)) = std::mem::replace(&mut *slot, Ok(None)) {
            old.stop().await;
        }
        if !settings.enabled {
            return;
        }
        let started = async {
            let id = web::web_identity(&self.inner.state_dir, &self.inner.name)?;
            web::start(self.clone(), &self.inner.name, &id, &settings).await
        }
        .await;
        *slot = match started {
            Ok(server) => Ok(Some(server)),
            Err(e) => {
                warn!("web interface not started: {e:#}");
                Err(format!("{e:#}"))
            }
        };
    }

    async fn web_status(&self) -> WebStatusView {
        let settings = self.inner.config.lock().unwrap().web.clone();
        let slot = self.inner.web.lock().await;
        let (running, error) = match &*slot {
            Ok(Some(s)) => (Some((s.fingerprint.clone(), s.addr.port())), None),
            Ok(None) => (None, None),
            Err(e) => (None, Some(e.clone())),
        };
        let port = running.as_ref().map(|r| r.1).unwrap_or(settings.port());
        WebStatusView {
            enabled: settings.enabled,
            lan: settings.lan,
            port,
            password_set: settings.password_hash.is_some(),
            urls: if running.is_some() { web::urls(&settings, port, &self.inner.name) } else { vec![] },
            fingerprint: running.map(|r| r.0),
            error,
        }
    }

    /// Devices of this computer: every connected device, then shared
    /// devices that are not plugged in.
    fn local_devices(&self) -> Result<Vec<LocalDeviceView>> {
        let inner = &self.inner;
        let (connected, resolved) = inner.export.snapshot()?;
        let in_use = inner.server.in_use();
        let policy = inner.export.policy();
        let shared_view = |id: &DeviceId, access: Option<&DeviceAccess>| {
            (
                access.cloned(),
                access.is_some_and(|a| a.is_open(policy)),
                in_use.get(&id.to_string()).map(|u| u.0.clone()),
            )
        };
        let mut out = vec![];
        for d in &connected {
            let entry = resolved.iter().find(|r| r.device.as_ref() == Some(d)).map(|r| &r.shared);
            let id = entry.map(|e| e.id.clone()).unwrap_or_else(|| DeviceId::of(d));
            let (access, open_to_all, used_by) = shared_view(&id, entry.map(|e| &e.access));
            out.push(LocalDeviceView {
                id: id.to_string(),
                busid: Some(d.info.busid.clone()),
                vendor_id: d.info.id_vendor,
                product_id: d.info.id_product,
                product: d.product.clone(),
                manufacturer: d.manufacturer.clone(),
                speed: d.info.speed.label().to_string(),
                present: true,
                by_port: id.by_port(),
                shared: entry.is_some(),
                access,
                open_to_all,
                used_by,
            });
        }
        for r in resolved.iter().filter(|r| r.device.is_none()) {
            let s = &r.shared;
            let (vendor_id, product_id) = s.id.ids().unwrap_or_default();
            let (access, open_to_all, used_by) = shared_view(&s.id, Some(&s.access));
            out.push(LocalDeviceView {
                id: s.id.to_string(),
                busid: None,
                vendor_id,
                product_id,
                product: s.product.clone(),
                manufacturer: s.manufacturer.clone(),
                speed: String::new(),
                present: false,
                by_port: s.id.by_port(),
                shared: true,
                access,
                open_to_all,
                used_by,
            });
        }
        Ok(out)
    }

    pub async fn handle(&self, req: Request) -> Response {
        match self.dispatch(req).await {
            Ok(r) => r,
            Err(e) => err(e),
        }
    }

    async fn dispatch(&self, req: Request) -> Result<Response> {
        let inner = &self.inner;
        Ok(match req {
            Request::Status => ok(StatusView {
                name: inner.name.clone(),
                roles: inner.config.lock().unwrap().roles.unwrap_or_default(),
                fingerprint: inner.fingerprint.clone(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                listen: inner.listen.clone(),
                pairing: inner.server.pairing().map(|(pin, left)| PairingView { pin, remaining_secs: left.as_secs() }),
                policy: inner.export.policy(),
                policy_chosen: inner.config.lock().unwrap().policy.is_some(),
            }),
            Request::LocalDevices => ok(self.local_devices()?),
            Request::SetShared { device, shared } => {
                if inner.export.set_shared(&device, shared)? {
                    self.shares_changed()?;
                }
                ok(())
            }
            Request::SetPolicy { policy } => {
                inner.export.set_policy(policy);
                inner.config.lock().unwrap().policy = Some(policy);
                self.shares_changed()?;
                ok(())
            }
            Request::SetDeviceAccess { device, mode, allowed } => {
                let current = inner.export.shared().into_iter().find(|s| s.id.to_string() == device);
                let mut access = current.map(|s| s.access).unwrap_or_default();
                access.mode = mode;
                if let Some(list) = allowed {
                    access.allowed = list.into_iter().collect();
                }
                inner.export.set_access(&device, access)?;
                self.shares_changed()?;
                ok(())
            }
            Request::SetClientAccess { fingerprint, devices } => {
                if !inner.clients.is_trusted(&fingerprint) {
                    return Err(ApiError::new("not_found", "no such paired computer").into());
                }
                inner.export.set_client_devices(&fingerprint, &devices)?;
                self.shares_changed()?;
                ok(())
            }
            Request::Usage { limit } => {
                let usage = inner.usage.clone();
                let mut entries = tokio::task::spawn_blocking(move || usage.entries()).await?;
                entries.reverse();
                entries.truncate(limit.unwrap_or(usize::MAX));
                ok(UsageView { entries, retention_days: inner.usage.retention_days() })
            }
            Request::SetUsageRetention { days } => {
                let days = days.clamp(1, 3650);
                let usage = inner.usage.clone();
                tokio::task::spawn_blocking(move || usage.set_retention_days(days)).await?;
                inner.config.lock().unwrap().usage_retention_days = days;
                self.save()?;
                ok(())
            }
            Request::OpenPairing { seconds } => {
                let seconds = seconds.clamp(30, 3600);
                let pin = inner.server.open_pairing(Duration::from_secs(seconds));
                ok(PairingView { pin, remaining_secs: seconds })
            }
            Request::ClosePairing => {
                inner.server.close_pairing();
                ok(())
            }
            Request::Discover { seconds } => {
                let found = discovery::browse(Duration::from_secs(seconds.clamp(1, 15))).await?;
                let views: Vec<DiscoveredView> = found
                    .into_iter()
                    .filter(|d| d.fingerprint != inner.fingerprint)
                    .map(|d| DiscoveredView {
                        paired: inner.client.trust.is_trusted(&d.fingerprint),
                        address: d.addrs.first().map(|a| a.to_string()),
                        name: d.name,
                        fingerprint: d.fingerprint,
                    })
                    .collect();
                ok(views)
            }
            Request::Pair { address, pin } => {
                let s = client::connect(&inner.client, &address, Some(&pin)).await?;
                ok(inner.client.trust.get(&s.server_fingerprint))
            }
            Request::RemoteDevices { server } => {
                let target = Target::Peer { fingerprint: server.clone() };
                let mut s = target.connect(&inner.client, &|_| {}).await?;
                let here = self.attached_here(&server);
                let devices: Vec<RemoteDeviceView> = s
                    .list()
                    .await?
                    .into_iter()
                    .map(|d| RemoteDeviceView {
                        attached_here: here
                            .iter()
                            .any(|h| refers_to(h, &d.id, d.present.then_some(d.info.busid.as_str()))),
                        by_port: DeviceId::parse(&d.id).by_port(),
                        in_use: d.in_use,
                        present: d.present,
                        allowed: d.allowed,
                        // Older servers do not report identities.
                        id: if d.id.is_empty() { d.info.busid.clone() } else { d.id },
                        busid: d.present.then_some(d.info.busid),
                        vendor_id: d.info.id_vendor,
                        product_id: d.info.id_product,
                        product: d.product,
                        manufacturer: d.manufacturer,
                        speed: if d.present { d.info.speed.label().to_string() } else { String::new() },
                    })
                    .collect();
                self.remember_names(&server, &devices);
                ok(devices)
            }
            Request::Attach { server, device } => {
                if !inner.client.trust.is_trusted(&server) {
                    return Err(ApiError::new("not_trusted", "server is not paired").into());
                }
                {
                    let mut cfg = inner.config.lock().unwrap();
                    if !cfg.attachments.iter().any(|a| a.server == server && a.device == device) {
                        cfg.attachments.push(SavedAttachment {
                            server: server.clone(),
                            device: device.clone(),
                            name: None,
                        });
                    }
                }
                self.save()?;
                self.spawn_attachment(&server, &device);
                ok(())
            }
            Request::Detach { server, device } => {
                self.remove_attachment(&server, &device)?;
                ok(())
            }
            Request::Attachments => {
                let map = inner.attachments.lock().unwrap();
                let mut views: Vec<AttachmentView> = map
                    .iter()
                    .map(|((server, device), slot)| {
                        let seen = slot.seen.lock().unwrap().clone();
                        let ids = seen.as_ref().map(|s| (s.0, s.1)).or_else(|| DeviceId::parse(device).ids());
                        AttachmentView {
                            server_name: inner.client.trust.get(server).map(|p| p.name).unwrap_or_default(),
                            server: server.clone(),
                            device: device.clone(),
                            busid: seen.map(|s| s.2),
                            name: None,
                            vendor_id: ids.map(|i| i.0),
                            product_id: ids.map(|i| i.1),
                            state: slot.state.lock().unwrap().clone(),
                        }
                    })
                    .collect();
                let names: HashMap<(String, String), String> = inner
                    .config
                    .lock()
                    .unwrap()
                    .attachments
                    .iter()
                    .filter_map(|a| Some(((a.server.clone(), a.device.clone()), a.name.clone()?)))
                    .collect();
                for v in &mut views {
                    v.name = names.get(&(v.server.clone(), v.device.clone())).cloned();
                }
                views.sort_by(|a, b| (&a.server_name, &a.device).cmp(&(&b.server_name, &b.device)));
                ok(views)
            }
            Request::Peers => ok(PeersView { servers: inner.client.trust.peers(), clients: inner.clients.peers() }),
            Request::Forget { fingerprint } => {
                let devices: Vec<String> = inner
                    .config
                    .lock()
                    .unwrap()
                    .attachments
                    .iter()
                    .filter(|a| a.server == fingerprint)
                    .map(|a| a.device.clone())
                    .collect();
                for d in devices {
                    self.remove_attachment(&fingerprint, &d)?;
                }
                inner.client.trust.remove(&fingerprint)?;
                inner.clients.remove(&fingerprint)?;
                inner.export.forget_client(&fingerprint);
                // Also ends the sessions of the forgotten computer.
                self.shares_changed()?;
                ok(())
            }
            Request::WebStatus => ok(self.web_status().await),
            Request::WebConfigure { enabled, lan, port, password } => {
                let hash = match password {
                    Some(p) => Some(tokio::task::spawn_blocking(move || web::hash_password(&p)).await??),
                    None => None,
                };
                {
                    let mut cfg = inner.config.lock().unwrap();
                    let w = &mut cfg.web;
                    if let Some(h) = hash {
                        w.password_hash = Some(h);
                    }
                    if let Some(l) = lan {
                        w.lan = l;
                    }
                    if let Some(p) = port {
                        w.port = Some(p);
                    }
                    if let Some(e) = enabled {
                        if e && w.password_hash.is_none() {
                            return Err(ApiError::new("password_required", "set a web password first").into());
                        }
                        w.enabled = e;
                    }
                }
                self.save()?;
                self.restart_web().await;
                ok(self.web_status().await)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn installer_setup() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.json");
        assert_eq!(saved_roles(dir.path()), Roles { server: true, client: true }, "both by default");

        // Other settings survive.
        let config = Config { usage_retention_days: 30, ..Config::default() };
        write_config(&path, &config).unwrap();

        let client_only = Roles { server: false, client: true };
        apply_setup(dir.path(), Setup { roles: Some(client_only), ..Setup::default() }).unwrap();
        assert_eq!(saved_roles(dir.path()), client_only);
        assert_eq!(read_config(&path).unwrap().usage_retention_days, 30);

        // The web interface needs a password.
        let enable = || Setup { web_enabled: Some(true), web_lan: Some(false), ..Setup::default() };
        let e = apply_setup(dir.path(), enable()).unwrap_err();
        assert_eq!(ApiError::from_anyhow(&e).code, "password_required");
        let short = Setup { web_password: Some("short".into()), ..enable() };
        assert_eq!(ApiError::from_anyhow(&apply_setup(dir.path(), short).unwrap_err()).code, "weak_password");
        let first = Setup { web_password: Some("correct horse".into()), web_port: Some(4000), ..enable() };
        apply_setup(dir.path(), first).unwrap();
        let web = saved_web(dir.path());
        assert!(web.enabled && !web.lan);
        assert_eq!(web.port(), 4000);
        let hash = web.password_hash.clone().unwrap();

        // An upgrade without a new password keeps the old one.
        apply_setup(dir.path(), Setup { web_lan: Some(true), ..enable() }).unwrap();
        let web = saved_web(dir.path());
        assert!(web.enabled && web.lan);
        assert_eq!(web.password_hash, Some(hash));

        apply_setup(dir.path(), Setup { web_enabled: Some(false), ..Setup::default() }).unwrap();
        assert!(!saved_web(dir.path()).enabled);
        assert_eq!(saved_roles(dir.path()), client_only, "roles untouched");
    }
}
