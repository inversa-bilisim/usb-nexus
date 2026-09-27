// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! The Windows named pipe the desktop app talks to.

#![cfg(windows)]

use std::sync::Arc;
use std::time::Duration;

use tokio::net::windows::named_pipe::ClientOptions;
use usbnexus_core::api::{self, Request, StatusView};
use usbnexus_core::backend::demo::{DemoHost, DemoImport};
use usbnexus_core::daemon::{Daemon, DaemonOptions};

/// A client that closes before the service accepts it makes
/// `ConnectNamedPipe` fail with ERROR_NO_DATA. That used to make the accept
/// loop spin on the broken instance forever, so the app saw the service as
/// "not running" and the busy loop starved USB traffic.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_client_that_leaves_early_does_not_break_the_pipe() {
    let dir = tempfile::tempdir().unwrap();
    let daemon = Daemon::start(DaemonOptions {
        state_dir: dir.path().to_path_buf(),
        name: "pipe-test".into(),
        listen: "127.0.0.1:0".into(),
        mdns: false,
        host: Arc::new(DemoHost),
        import: Arc::new(DemoImport::default()),
        events: None,
    })
    .await
    .unwrap();
    let name = std::path::PathBuf::from(format!(r"\\.\pipe\usbnexus-test-{}", std::process::id()));
    let serve = api::serve_pipe(&name, daemon, false).unwrap();

    // Connect and leave before the accept loop runs.
    drop(ClientOptions::new().open(name.as_os_str()).unwrap());
    tokio::spawn(serve);

    for _ in 0..3 {
        let status = tokio::time::timeout(Duration::from_secs(5), api::call::<StatusView>(&name, &Request::Status))
            .await
            .expect("the pipe answers again")
            .unwrap();
        assert_eq!(status.name, "pipe-test");
    }
}

/// The installer's web port check reads the listener tables.
#[test]
fn listening_ports_are_seen() {
    for addr in ["127.0.0.1:0", "[::1]:0"] {
        let Ok(listener) = std::net::TcpListener::bind(addr) else { continue };
        let port = listener.local_addr().unwrap().port();
        assert!(usbnexus_core::windows::tcp_port_listening(port), "{addr}");
        drop(listener);
        assert!(!usbnexus_core::windows::tcp_port_listening(port), "{addr} closed");
    }
}
