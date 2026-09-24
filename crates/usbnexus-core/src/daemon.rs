// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! The USB Nexus service: one long-running process that shares local devices,
//! keeps remote devices attached, and answers the local API.
//!
//! Its settings (shared devices, attachments) are saved in `config.json` in
//! the state directory and restored on start-up, so shares and attachments
//! survive reboots.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio::task::JoinHandle;
use tracing::warn;

use crate::api::{
    ApiError, AttachState, AttachmentView, DiscoveredView, LocalDeviceView, PairingView, PeersView, RemoteDeviceView,
    Request, Response, StatusView, WebStatusView,
};
use crate::backend::{DeviceHost, ImportBackend, SharedExport};
use crate::client::{self, AttachEvent, ClientConfig, Target};
use crate::discovery;
use crate::identity::Identity;
use crate::server::{Server, ServerConfig, ServerEvent};
use crate::trust::TrustStore;
use crate::web::{self, WebServer, WebSettings};

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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct Config {
    #[serde(default)]
    shared: Vec<String>,
    #[serde(default)]
    attachments: Vec<SavedAttachment>,
    #[serde(default)]
    web: WebSettings,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct SavedAttachment {
    server: String,
    busid: String,
}

type Key = (String, String);

struct Slot {
    state: Arc<Mutex<AttachState>>,
    ids: Arc<Mutex<Option<(u16, u16)>>>,
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

impl Daemon {
    /// Loads state, starts the server and restores saved attachments.
    pub async fn start(opts: DaemonOptions) -> Result<Daemon> {
        std::fs::create_dir_all(&opts.state_dir).with_context(|| format!("creating {}", opts.state_dir.display()))?;
        let identity = Identity::load_or_create(&opts.state_dir, &opts.name)?;
        let clients = TrustStore::load(&opts.state_dir.join("trusted-clients.json"))?;
        let servers = TrustStore::load(&opts.state_dir.join("trusted-servers.json"))?;
        let config_path = opts.state_dir.join("config.json");
        let config: Config = match std::fs::read(&config_path) {
            Ok(data) => serde_json::from_slice(&data).with_context(|| format!("parsing {}", config_path.display()))?,
            Err(_) => Config::default(),
        };

        let export = Arc::new(SharedExport::new(opts.host.clone(), config.shared.clone()));
        let server = Server::with_events(
            ServerConfig {
                name: opts.name.clone(),
                identity: identity.clone(),
                trust: clients.clone(),
                backend: export.clone(),
            },
            opts.events.clone().unwrap_or_else(|| Arc::new(|_| {})),
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
                _mdns: mdns,
                state_dir: opts.state_dir.clone(),
                web: tokio::sync::Mutex::new(Ok(None)),
            }),
        };
        for a in saved {
            daemon.spawn_attachment(&a.server, &a.busid);
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
        for (_, slot) in self.inner.attachments.lock().unwrap().drain() {
            slot.task.abort();
        }
        for busid in self.inner.export.shared() {
            if let Err(e) = self.inner.export.host().release(&busid) {
                warn!(busid, "could not release device: {e:#}");
            }
        }
    }

    fn save(&self) -> Result<()> {
        let data = serde_json::to_vec_pretty(&*self.inner.config.lock().unwrap())?;
        let tmp = self.inner.config_path.with_extension("tmp");
        // Holds the web password hash: owner-only.
        crate::identity::write_private(&tmp, &data)?;
        std::fs::rename(&tmp, &self.inner.config_path)?;
        Ok(())
    }

    fn spawn_attachment(&self, server: &str, busid: &str) {
        let key = (server.to_string(), busid.to_string());
        let mut map = self.inner.attachments.lock().unwrap();
        if map.get(&key).is_some_and(|s| !s.task.is_finished()) {
            return;
        }
        let state = Arc::new(Mutex::new(AttachState::Connecting));
        let ids = Arc::new(Mutex::new(None));
        let (cfg, import) = (self.inner.client.clone(), self.inner.import.clone());
        let (st, id2) = (state.clone(), ids.clone());
        let (fp, bus) = key.clone();
        let task = tokio::spawn(async move {
            let target = Target::Peer { fingerprint: fp };
            let events = |ev: AttachEvent| {
                let next = match ev {
                    AttachEvent::Connecting { .. } => AttachState::Connecting,
                    AttachEvent::Attached { port, device } => {
                        *id2.lock().unwrap() = Some((device.id_vendor, device.id_product));
                        AttachState::Attached { port }
                    }
                    AttachEvent::Retrying { delay } => {
                        let error = match &*st.lock().unwrap() {
                            AttachState::Retrying { error, .. } => error.clone(),
                            _ => ApiError::new("connection_lost", ""),
                        };
                        AttachState::Retrying { seconds: delay.as_secs_f64().ceil() as u64, error }
                    }
                    AttachEvent::Disconnected { error } => AttachState::Retrying { seconds: 0, error },
                    AttachEvent::Detached => AttachState::Stopped,
                };
                *st.lock().unwrap() = next;
            };
            let result = client::attach_forever(&cfg, &target, &bus, import, events).await;
            *st.lock().unwrap() = match result {
                Ok(()) => AttachState::Stopped,
                Err(e) => AttachState::Failed { error: ApiError::from_anyhow(&e) },
            };
        });
        map.insert(key, Slot { state, ids, task });
    }

    fn remove_attachment(&self, server: &str, busid: &str) -> Result<()> {
        if let Some(slot) = self.inner.attachments.lock().unwrap().remove(&(server.to_string(), busid.to_string())) {
            // Dropping the relay closes the socket; the OS detaches the device.
            slot.task.abort();
        }
        self.inner.config.lock().unwrap().attachments.retain(|a| !(a.server == server && a.busid == busid));
        self.save()
    }

    fn attached_here(&self, server: &str) -> Vec<String> {
        self.inner
            .attachments
            .lock()
            .unwrap()
            .iter()
            .filter(|((s, _), slot)| s == server && !matches!(*slot.state.lock().unwrap(), AttachState::Stopped))
            .map(|((_, b), _)| b.clone())
            .collect()
    }

    /// (Re)starts or stops the web interface according to the settings.
    async fn restart_web(&self) {
        let settings = self.inner.config.lock().unwrap().web.clone();
        let mut slot = self.inner.web.lock().await;
        *slot = Ok(None); // stop the old one (and drop its sessions) first
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
                fingerprint: inner.fingerprint.clone(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                listen: inner.listen.clone(),
                pairing: inner.server.pairing().map(|(pin, left)| PairingView { pin, remaining_secs: left.as_secs() }),
            }),
            Request::LocalDevices => {
                let in_use = inner.server.in_use();
                let devices: Vec<LocalDeviceView> = inner
                    .export
                    .host()
                    .list_all()?
                    .into_iter()
                    .map(|d| LocalDeviceView {
                        shared: inner.export.is_shared(&d.info.busid),
                        used_by: in_use.get(&d.info.busid).cloned(),
                        busid: d.info.busid,
                        vendor_id: d.info.id_vendor,
                        product_id: d.info.id_product,
                        product: d.product,
                        manufacturer: d.manufacturer,
                        speed: d.info.speed.label().to_string(),
                    })
                    .collect();
                ok(devices)
            }
            Request::SetShared { busid, shared } => {
                inner.export.set_shared(&busid, shared)?;
                inner.config.lock().unwrap().shared = inner.export.shared().into_iter().collect();
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
                        attached_here: here.contains(&d.info.busid),
                        in_use: d.in_use,
                        busid: d.info.busid,
                        vendor_id: d.info.id_vendor,
                        product_id: d.info.id_product,
                        product: d.product,
                        manufacturer: d.manufacturer,
                        speed: d.info.speed.label().to_string(),
                    })
                    .collect();
                ok(devices)
            }
            Request::Attach { server, busid } => {
                if !inner.client.trust.is_trusted(&server) {
                    return Err(ApiError::new("not_trusted", "server is not paired").into());
                }
                {
                    let mut cfg = inner.config.lock().unwrap();
                    let entry = SavedAttachment { server: server.clone(), busid: busid.clone() };
                    if !cfg.attachments.contains(&entry) {
                        cfg.attachments.push(entry);
                    }
                }
                self.save()?;
                self.spawn_attachment(&server, &busid);
                ok(())
            }
            Request::Detach { server, busid } => {
                self.remove_attachment(&server, &busid)?;
                ok(())
            }
            Request::Attachments => {
                let map = inner.attachments.lock().unwrap();
                let mut views: Vec<AttachmentView> = map
                    .iter()
                    .map(|((server, busid), slot)| {
                        let ids = *slot.ids.lock().unwrap();
                        AttachmentView {
                            server_name: inner.client.trust.get(server).map(|p| p.name).unwrap_or_default(),
                            server: server.clone(),
                            busid: busid.clone(),
                            vendor_id: ids.map(|i| i.0),
                            product_id: ids.map(|i| i.1),
                            state: slot.state.lock().unwrap().clone(),
                        }
                    })
                    .collect();
                views.sort_by(|a, b| (&a.server_name, &a.busid).cmp(&(&b.server_name, &b.busid)));
                ok(views)
            }
            Request::Peers => ok(PeersView { servers: inner.client.trust.peers(), clients: inner.clients.peers() }),
            Request::Forget { fingerprint } => {
                let busids: Vec<String> = inner
                    .config
                    .lock()
                    .unwrap()
                    .attachments
                    .iter()
                    .filter(|a| a.server == fingerprint)
                    .map(|a| a.busid.clone())
                    .collect();
                for b in busids {
                    self.remove_attachment(&fingerprint, &b)?;
                }
                inner.client.trust.remove(&fingerprint)?;
                inner.clients.remove(&fingerprint)?;
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
