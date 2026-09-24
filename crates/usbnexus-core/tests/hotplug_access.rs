// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! Two services with devices that come and go: identity based sharing,
//! migration of older configurations, waiting attachments, access control
//! and the usage log, driven through the local API.

#![cfg(unix)]

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::{bail, Result};
use serde::de::DeserializeOwned;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::task::JoinHandle;
use usbnexus_core::access::Policy;
use usbnexus_core::api::{
    self, AttachState, AttachmentView, LocalDeviceView, PairingView, PeersView, RemoteDeviceView, Request, StatusView,
    UsageView,
};
use usbnexus_core::backend::demo::{DemoHost, DemoImport};
use usbnexus_core::backend::{into_tokio, loopback_pair, BoxFuture, DeviceHost, LocalDevice};
use usbnexus_core::daemon::{Daemon, DaemonOptions};
use usbnexus_core::identity::Identity;
use usbnexus_core::usage::UsageKind;

const STICK: &str = "0781:5567:4C530001231120115142";
const RECEIVER: &str = "046d:c52b@1-2";

/// The demo devices, which can be unplugged and plugged into other ports.
/// Unplugging ends a running export, like pulling out a real device.
#[derive(Default)]
struct PlugHost {
    /// busid -> where it is plugged in now (`None`: unplugged).
    moved: Mutex<HashMap<String, Option<String>>>,
    exports: Mutex<HashMap<String, JoinHandle<()>>>,
}

impl PlugHost {
    /// Unplugs the device originally at `original`.
    fn unplug(&self, original: &str) {
        let port = self.moved.lock().unwrap().insert(original.into(), None);
        let port = port.unwrap_or_else(|| Some(original.into()));
        if let Some(task) = port.and_then(|p| self.exports.lock().unwrap().remove(&p)) {
            task.abort();
        }
    }

    /// Plugs the device originally at `original` into `port`.
    fn plug(&self, original: &str, port: &str) {
        self.moved.lock().unwrap().insert(original.into(), Some(port.into()));
    }
}

impl DeviceHost for PlugHost {
    fn list_all(&self) -> Result<Vec<LocalDevice>> {
        let moved = self.moved.lock().unwrap();
        Ok(DemoHost
            .list_all()?
            .into_iter()
            .filter_map(|mut d| match moved.get(&d.info.busid) {
                None => Some(d),
                Some(None) => None,
                Some(Some(port)) => {
                    d.info.busid.clone_from(port);
                    Some(d)
                }
            })
            .collect())
    }

    fn export<'a>(&'a self, busid: &'a str) -> BoxFuture<'a, Result<tokio::net::TcpStream>> {
        Box::pin(async move {
            if !self.list_all()?.iter().any(|d| d.info.busid == busid) {
                bail!("no USB device {busid}");
            }
            let (ours, kernel) = loopback_pair()?;
            let mut kernel = into_tokio(kernel)?;
            let task = tokio::spawn(async move {
                let mut buf = [0u8; 4096];
                while let Ok(n) = kernel.read(&mut buf).await {
                    if n == 0 || kernel.write_all(&buf[..n]).await.is_err() {
                        break;
                    }
                }
            });
            self.exports.lock().unwrap().insert(busid.to_string(), task);
            Ok(into_tokio(ours)?)
        })
    }

    fn release(&self, _busid: &str) -> Result<()> {
        Ok(())
    }
}

struct Node {
    daemon: Daemon,
    socket: PathBuf,
    listen: String,
}

async fn node(dir: &Path, name: &str, host: Arc<dyn DeviceHost>) -> Node {
    let daemon = Daemon::start(DaemonOptions {
        state_dir: dir.to_path_buf(),
        name: name.into(),
        listen: "127.0.0.1:0".into(),
        mdns: false,
        host,
        import: Arc::new(DemoImport::default()),
        events: None,
    })
    .await
    .unwrap();
    let socket = dir.join("api.sock");
    let _ = std::fs::remove_file(&socket);
    let listener = tokio::net::UnixListener::bind(&socket).unwrap();
    tokio::spawn(api::serve(listener, daemon.clone()));
    let status: StatusView = api::call(&socket, &Request::Status).await.unwrap();
    Node { daemon, socket, listen: status.listen }
}

async fn call<T: DeserializeOwned>(n: &Node, req: Request) -> T {
    api::call(&n.socket, &req).await.unwrap_or_else(|e| panic!("{req:?}: {e:#}"))
}

/// Polls the attachments of `n` until `what` holds (up to 40 s).
async fn wait_for(n: &Node, what: impl Fn(&AttachmentView) -> bool) -> AttachmentView {
    for _ in 0..400 {
        let a: Vec<AttachmentView> = call(n, Request::Attachments).await;
        if a.len() == 1 && what(&a[0]) {
            return a[0].clone();
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    panic!("condition not reached: {:?}", call::<Vec<AttachmentView>>(n, Request::Attachments).await);
}

fn attached(a: &AttachmentView) -> bool {
    matches!(a.state, AttachState::Attached { .. })
}

fn waiting(code: &'static str) -> impl Fn(&AttachmentView) -> bool {
    move |a| matches!(&a.state, AttachState::Waiting { error, .. } if error.code == code)
}

fn local<'a>(list: &'a [LocalDeviceView], id: &str) -> &'a LocalDeviceView {
    list.iter().find(|d| d.id == id).unwrap_or_else(|| panic!("{id} not listed: {list:?}"))
}

#[tokio::test]
async fn devices_come_and_go_and_access_is_enforced() {
    let (dir_a, dir_b) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());

    // A configuration written by an older version: devices shared by bus id.
    std::fs::write(dir_a.path().join("config.json"), r#"{"shared":["1-1","1-2"]}"#).unwrap();
    let host = Arc::new(PlugHost::default());
    let a = node(dir_a.path(), "office", host.clone()).await;

    // Migrated to identities: by serial number, or by port without one.
    let list: Vec<LocalDeviceView> = call(&a, Request::LocalDevices).await;
    let stick = local(&list, STICK);
    assert!(stick.shared && stick.present && !stick.by_port);
    let receiver = local(&list, RECEIVER);
    assert!(receiver.shared && receiver.by_port);
    let saved = std::fs::read_to_string(dir_a.path().join("config.json")).unwrap();
    assert!(saved.contains(STICK) && saved.contains(RECEIVER), "{saved}");
    let status: StatusView = call(&a, Request::Status).await;
    assert_eq!(status.policy, Policy::Open, "open until chosen");
    assert!(!status.policy_chosen);

    // B pairs, then is restarted with an attachment saved by bus id.
    let b = node(dir_b.path(), "laptop", Arc::new(DemoHost)).await;
    let pin: PairingView = call(&a, Request::OpenPairing { seconds: 60 }).await;
    let _: serde_json::Value = call(&b, Request::Pair { address: a.listen.clone(), pin: pin.pin }).await;
    let server = Identity::load_or_create(dir_a.path(), "office").unwrap().fingerprint();
    b.daemon.shutdown();
    drop(b);
    std::fs::write(
        dir_b.path().join("config.json"),
        format!(r#"{{"attachments":[{{"server":"{server}","busid":"1-1"}}]}}"#),
    )
    .unwrap();
    let b = node(dir_b.path(), "laptop", Arc::new(DemoHost)).await;
    let att = wait_for(&b, attached).await;
    assert_eq!(att.device, STICK, "the attachment learned the device identity");
    assert_eq!(att.busid.as_deref(), Some("1-1"));
    for _ in 0..50 {
        if std::fs::read_to_string(dir_b.path().join("config.json")).unwrap().contains(STICK) {
            break;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    assert!(std::fs::read_to_string(dir_b.path().join("config.json")).unwrap().contains(STICK));

    // Unplugged: A keeps it listed, B waits for it.
    host.unplug("1-1");
    wait_for(&b, waiting("no_such_device")).await;
    let list: Vec<LocalDeviceView> = call(&a, Request::LocalDevices).await;
    let stick = local(&list, STICK);
    assert!(stick.shared && !stick.present && stick.busid.is_none());
    assert_eq!(stick.product.as_deref(), Some("Cruzer Blade"), "name remembered");
    let remote: Vec<RemoteDeviceView> = call(&b, Request::RemoteDevices { server: server.clone() }).await;
    let r = remote.iter().find(|d| d.id == STICK).unwrap();
    assert!(!r.present && r.attached_here);
    let att: Vec<AttachmentView> = call(&b, Request::Attachments).await;
    assert_eq!(att[0].name.as_deref(), Some("SanDisk Cruzer Blade"), "name kept for the attachment");

    // Plugged into another port: shared again and re-attached by itself.
    host.plug("1-1", "3-2");
    let att = wait_for(&b, attached).await;
    assert_eq!(att.busid.as_deref(), Some("3-2"));

    // Restricted policy: B loses the device at once and sees "no permission".
    let _: () = call(&a, Request::SetPolicy { policy: Policy::Restricted }).await;
    wait_for(&b, waiting("access_denied")).await;
    let remote: Vec<RemoteDeviceView> = call(&b, Request::RemoteDevices { server: server.clone() }).await;
    assert!(remote.iter().all(|d| !d.allowed), "{remote:?}");
    assert_eq!(remote.len(), 2, "restricted devices stay visible");

    // Allowing B the stick lets it come back.
    let peers: PeersView = call(&a, Request::Peers).await;
    let laptop = peers.clients[0].fingerprint.clone();
    let _: () = call(&a, Request::SetClientAccess { fingerprint: laptop.clone(), devices: vec![STICK.into()] }).await;
    wait_for(&b, attached).await;
    let list: Vec<LocalDeviceView> = call(&a, Request::LocalDevices).await;
    assert!(local(&list, STICK).access.as_ref().unwrap().allowed.contains(&laptop));
    assert!(!local(&list, STICK).open_to_all);
    assert_eq!(local(&list, STICK).used_by.as_deref(), Some("laptop"));
    let remote: Vec<RemoteDeviceView> = call(&b, Request::RemoteDevices { server: server.clone() }).await;
    assert!(remote.iter().find(|d| d.id == STICK).unwrap().allowed);
    assert!(!remote.iter().find(|d| d.id == RECEIVER).unwrap().allowed);

    // Everything was recorded on A.
    let _: () = call(&b, Request::Detach { server: server.clone(), device: STICK.into() }).await;
    let mut kinds = vec![];
    for _ in 0..50 {
        let usage: UsageView = call(&a, Request::Usage { limit: None }).await;
        kinds = usage.entries.iter().map(|e| e.kind).collect();
        if kinds.first() == Some(&UsageKind::Detached) {
            break;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    for k in [UsageKind::Paired, UsageKind::Attached, UsageKind::Detached, UsageKind::Denied] {
        assert!(kinds.contains(&k), "{k:?} missing from {kinds:?}");
    }
    let usage: UsageView = call(&a, Request::Usage { limit: Some(1) }).await;
    assert_eq!(usage.retention_days, 90);
    let e = &usage.entries[0];
    assert_eq!(e.kind, UsageKind::Detached, "newest first");
    assert_eq!((e.computer.as_deref(), e.device.as_deref()), (Some("laptop"), Some(STICK)));
    assert!(e.duration_secs.is_some());
}
