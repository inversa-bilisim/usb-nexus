// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! Two services talking to each other, driven through the local API socket
//! exactly like the desktop app does.

#![cfg(unix)]

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use serde::de::DeserializeOwned;
use usbnexus_core::api::{
    self, AttachState, AttachmentView, LocalDeviceView, PairingView, PeersView, RemoteDeviceView, Request, StatusView,
};
use usbnexus_core::backend::demo::{DemoHost, DemoImport};
use usbnexus_core::daemon::{Daemon, DaemonOptions};

struct Node {
    daemon: Daemon,
    socket: PathBuf,
    listen: String,
}

async fn node(dir: &Path, name: &str) -> Node {
    let daemon = Daemon::start(DaemonOptions {
        state_dir: dir.to_path_buf(),
        name: name.into(),
        listen: "127.0.0.1:0".into(),
        mdns: false,
        host: Arc::new(DemoHost),
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

async fn wait_for(n: &Node, what: impl Fn(&[AttachmentView]) -> bool) -> Vec<AttachmentView> {
    for _ in 0..100 {
        let a: Vec<AttachmentView> = call(n, Request::Attachments).await;
        if what(&a) {
            return a;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    panic!("condition not reached: {:?}", call::<Vec<AttachmentView>>(n, Request::Attachments).await);
}

fn attached(a: &[AttachmentView]) -> bool {
    a.len() == 1 && matches!(a[0].state, AttachState::Attached { .. })
}

#[tokio::test]
async fn share_pair_attach_restore_detach() {
    let (dir_a, dir_b) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
    let a = node(dir_a.path(), "office").await;
    let b = node(dir_b.path(), "laptop").await;

    // A shares one device.
    let _: () = call(&a, Request::SetShared { device: "1-1".into(), shared: true }).await;
    let local: Vec<LocalDeviceView> = call(&a, Request::LocalDevices).await;
    let shared: Vec<&LocalDeviceView> = local.iter().filter(|d| d.shared).collect();
    assert_eq!(shared.len(), 1);
    assert_eq!(shared[0].busid.as_deref(), Some("1-1"));
    let id = shared[0].id.clone();
    assert_eq!(id, "0781:5567:4C530001231120115142", "shared by identity");

    // B cannot pair with a wrong PIN, then pairs with the right one.
    let pin: PairingView = call(&a, Request::OpenPairing { seconds: 60 }).await;
    let wrong = if pin.pin == "000000" { "111111" } else { "000000" };
    let e = api::call::<serde_json::Value>(&b.socket, &Request::Pair { address: a.listen.clone(), pin: wrong.into() })
        .await
        .unwrap_err();
    assert_eq!(e.downcast_ref::<api::ApiError>().unwrap().code, "pairing_failed");
    let _: serde_json::Value = call(&b, Request::Pair { address: a.listen.clone(), pin: pin.pin.clone() }).await;
    let peers: PeersView = call(&b, Request::Peers).await;
    let server_fp = peers.servers[0].fingerprint.clone();
    assert_eq!(peers.servers[0].name, "office");
    let peers_a: PeersView = call(&a, Request::Peers).await;
    assert_eq!(peers_a.clients[0].name, "laptop");

    // B lists A's shared devices and attaches one.
    let remote: Vec<RemoteDeviceView> = call(&b, Request::RemoteDevices { server: server_fp.clone() }).await;
    assert_eq!(remote.len(), 1);
    assert!(!remote[0].in_use && !remote[0].attached_here);
    let _: () = call(&b, Request::Attach { server: server_fp.clone(), device: id.clone() }).await;
    let list = wait_for(&b, attached).await;
    assert_eq!(list[0].server_name, "office");
    assert_eq!(list[0].vendor_id, Some(0x0781));

    // A sees who uses the device; B sees it as attached here.
    let local: Vec<LocalDeviceView> = call(&a, Request::LocalDevices).await;
    assert_eq!(local.iter().find(|d| d.id == id).unwrap().used_by.as_deref(), Some("laptop"));
    let remote: Vec<RemoteDeviceView> = call(&b, Request::RemoteDevices { server: server_fp.clone() }).await;
    assert!(remote[0].in_use && remote[0].attached_here);

    // Restarting B restores the attachment from its saved configuration.
    b.daemon.shutdown();
    drop(b);
    let b = node(dir_b.path(), "laptop").await;
    wait_for(&b, attached).await;

    // Detaching removes it and frees the device on A.
    let _: () = call(&b, Request::Detach { server: server_fp.clone(), device: id.clone() }).await;
    wait_for(&b, |a| a.is_empty()).await;
    for _ in 0..50 {
        let local: Vec<LocalDeviceView> = call(&a, Request::LocalDevices).await;
        if local.iter().all(|d| d.used_by.is_none()) {
            break;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    let local: Vec<LocalDeviceView> = call(&a, Request::LocalDevices).await;
    assert!(local.iter().all(|d| d.used_by.is_none()));

    // Forgetting the server removes it from B's paired list.
    let _: () = call(&b, Request::Forget { fingerprint: server_fp }).await;
    let peers: PeersView = call(&b, Request::Peers).await;
    assert!(peers.servers.is_empty());
}

#[tokio::test]
async fn attach_requires_pairing_and_bad_requests_are_rejected() {
    let dir = tempfile::tempdir().unwrap();
    let n = node(dir.path(), "solo").await;
    let e = api::call::<()>(&n.socket, &Request::Attach { server: "ab".repeat(32), device: "1-1".into() })
        .await
        .unwrap_err();
    assert_eq!(e.downcast_ref::<api::ApiError>().unwrap().code, "not_trusted");

    // A malformed line gets an "invalid" error rather than a dropped connection.
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    let mut s = tokio::net::UnixStream::connect(&n.socket).await.unwrap();
    s.write_all(b"{\"cmd\":\"nope\"}\n").await.unwrap();
    let mut line = String::new();
    BufReader::new(s).read_line(&mut line).await.unwrap();
    assert!(line.contains("\"invalid\""), "{line}");
}
