// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! End-to-end tests of server and client over real TCP + TLS, with mock
//! backends standing in for the kernel USB/IP drivers.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::Result;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use usbnexus_core::backend::{
    into_tokio, loopback_pair, Allowed, BoxFuture, ExportBackend, ImportBackend, LocalDevice, Offered,
};
use usbnexus_core::client::{self, AttachEvent, ClientConfig, ClientError, Target};
use usbnexus_core::control::{ErrorCode, RemoteError};
use usbnexus_core::device_id::DeviceId;
use usbnexus_core::identity::Identity;
use usbnexus_core::server::{Server, ServerConfig};
use usbnexus_core::trust::TrustStore;
use usbnexus_proto::{DeviceInfo, Speed};

fn device() -> DeviceInfo {
    DeviceInfo {
        path: "/sys/devices/fake/1-1".into(),
        busid: "1-1".into(),
        busnum: 1,
        devnum: 2,
        speed: Speed::High,
        id_vendor: 0x1234,
        id_product: 0x5678,
        bcd_device: 0x0100,
        device_class: 0,
        device_subclass: 0,
        device_protocol: 0,
        configuration_value: 1,
        num_configurations: 1,
        interfaces: vec![],
    }
}

const DEVICE_ID: &str = "1234:5678:SER1";

/// Export side "kernel": echoes URB bytes; closes the socket on "bye".
/// Offers one device, which can be unplugged and restricted.
struct MockExport {
    plugged: AtomicBool,
    allowed: Mutex<Allowed>,
}

impl Default for MockExport {
    fn default() -> Self {
        MockExport { plugged: AtomicBool::new(true), allowed: Mutex::new(Allowed::Everyone) }
    }
}

impl ExportBackend for MockExport {
    fn list(&self) -> Result<Vec<Offered>> {
        let local = LocalDevice {
            info: device(),
            product: Some("Mock".into()),
            manufacturer: None,
            serial: Some("SER1".into()),
            driver: None,
        };
        Ok(vec![Offered {
            id: DeviceId::of(&local),
            device: self.plugged.load(Ordering::SeqCst).then_some(local),
            product: Some("Mock".into()),
            manufacturer: None,
            allowed: self.allowed.lock().unwrap().clone(),
        }])
    }

    fn export<'a>(&'a self, _busid: &'a str) -> BoxFuture<'a, Result<TcpStream>> {
        Box::pin(async move {
            let (ours, kernel) = loopback_pair()?;
            let mut kernel = into_tokio(kernel)?;
            tokio::spawn(async move {
                let mut buf = [0u8; 1024];
                while let Ok(n) = kernel.read(&mut buf).await {
                    if n == 0 || &buf[..n] == b"bye" {
                        break;
                    }
                    if kernel.write_all(&buf[..n]).await.is_err() {
                        break;
                    }
                }
            });
            Ok(into_tokio(ours)?)
        })
    }
}

/// Import side "kernel": hands its socket end to the test.
#[derive(Default)]
struct MockImport {
    kernel_ends: Mutex<Vec<TcpStream>>,
    next_port: Mutex<u32>,
}

impl ImportBackend for MockImport {
    fn attach<'a>(&'a self, _device: &'a DeviceInfo) -> BoxFuture<'a, Result<(u32, TcpStream)>> {
        Box::pin(async move {
            let (ours, kernel) = loopback_pair()?;
            self.kernel_ends.lock().unwrap().push(into_tokio(kernel)?);
            let mut p = self.next_port.lock().unwrap();
            *p += 1;
            Ok((*p, into_tokio(ours)?))
        })
    }

    fn detach(&self, _port: u32) -> Result<()> {
        Ok(())
    }
}

struct Fixture {
    server: Server,
    export: Arc<MockExport>,
    server_trust: TrustStore,
    addr: String,
    client: ClientConfig,
}

async fn start() -> Fixture {
    let server_trust = TrustStore::in_memory();
    let export = Arc::new(MockExport::default());
    let server = Server::new(ServerConfig {
        name: "test-server".into(),
        identity: Identity::generate("test-server").unwrap(),
        trust: server_trust.clone(),
        backend: export.clone(),
    })
    .unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    let s = server.clone();
    tokio::spawn(async move { s.serve(listener).await });
    let client = ClientConfig {
        name: "test-client".into(),
        identity: Identity::generate("test-client").unwrap(),
        trust: TrustStore::in_memory(),
    };
    Fixture { server, export, server_trust, addr, client }
}

fn remote_code(e: &anyhow::Error) -> Option<ErrorCode> {
    e.downcast_ref::<RemoteError>().map(|r| r.code)
}

#[tokio::test]
async fn pairing_flow() {
    let f = start().await;

    // Unpaired and no PIN: refused.
    let e = client::connect(&f.client, &f.addr, None).await.err().unwrap();
    assert!(matches!(e.downcast_ref::<ClientError>(), Some(ClientError::PairingRequired { .. })));

    // PIN given but pairing not open on the server.
    let e = client::connect(&f.client, &f.addr, Some("000000")).await.err().unwrap();
    assert_eq!(remote_code(&e), Some(ErrorCode::PairingClosed));

    // Wrong PIN.
    f.server.open_pairing_with_pin("123456", Duration::from_secs(60));
    let e = client::connect(&f.client, &f.addr, Some("654321")).await.err().unwrap();
    assert_eq!(remote_code(&e), Some(ErrorCode::PairingFailed));
    assert!(f.client.trust.peers().is_empty());
    assert!(f.server_trust.peers().is_empty());

    // Correct PIN pairs both directions and closes the window.
    let mut s = client::connect(&f.client, &f.addr, Some("123-456")).await.unwrap();
    assert_eq!(s.server_name, "test-server");
    assert!(f.client.trust.is_trusted(f.server.fingerprint()));
    assert!(f.server_trust.is_trusted(&f.client.identity.fingerprint()));
    assert!(!f.server.pairing_open());
    let devices = s.list().await.unwrap();
    assert_eq!(devices.len(), 1);
    assert_eq!(devices[0].product.as_deref(), Some("Mock"));
    assert_eq!(devices[0].id, DEVICE_ID);
    assert!(devices[0].present && devices[0].allowed);

    // Later connections need no PIN.
    let mut s = client::connect(&f.client, &f.addr, None).await.unwrap();
    assert_eq!(s.list().await.unwrap().len(), 1);

    // If the server forgets the client, pairing is required again.
    f.server_trust.remove(&f.client.identity.fingerprint()).unwrap();
    let e = client::connect(&f.client, &f.addr, None).await.err().unwrap();
    assert!(matches!(e.downcast_ref::<ClientError>(), Some(ClientError::PairingRequired { .. })));
}

#[tokio::test]
async fn pairing_window_closes_after_repeated_failures() {
    let f = start().await;
    f.server.open_pairing_with_pin("111111", Duration::from_secs(60));
    for _ in 0..usbnexus_core::pairing::MAX_ATTEMPTS {
        let e = client::connect(&f.client, &f.addr, Some("999999")).await.err().unwrap();
        assert_eq!(remote_code(&e), Some(ErrorCode::PairingFailed));
    }
    // Even the right PIN is now refused.
    let e = client::connect(&f.client, &f.addr, Some("111111")).await.err().unwrap();
    assert_eq!(remote_code(&e), Some(ErrorCode::PairingClosed));
}

#[tokio::test]
async fn busy_and_unknown_devices() {
    let f = start().await;
    f.server.open_pairing_with_pin("222222", Duration::from_secs(60));
    client::connect(&f.client, &f.addr, Some("222222")).await.unwrap();

    let s = client::connect(&f.client, &f.addr, None).await.unwrap();
    let e = s.import("9-9").await.err().unwrap();
    assert_eq!(remote_code(&e), Some(ErrorCode::NoSuchDevice));

    // Asking by bus id (older clients) reports the identity to use instead.
    let s = client::connect(&f.client, &f.addr, None).await.unwrap();
    let (_dev, id, _held) = s.import("1-1").await.unwrap();
    assert_eq!(id, DEVICE_ID);
    let s = client::connect(&f.client, &f.addr, None).await.unwrap();
    let e = s.import(DEVICE_ID).await.err().unwrap();
    assert_eq!(remote_code(&e), Some(ErrorCode::DeviceBusy));
}

async fn expect_attached(rx: &mut mpsc::UnboundedReceiver<AttachEvent>) -> u32 {
    loop {
        let ev = tokio::time::timeout(Duration::from_secs(20), rx.recv()).await.expect("attach timed out").unwrap();
        if let AttachEvent::Attached { port, .. } = ev {
            return port;
        }
    }
}

async fn echo(kernel: &mut TcpStream, msg: &[u8]) {
    kernel.write_all(msg).await.unwrap();
    let mut buf = vec![0u8; msg.len()];
    tokio::time::timeout(Duration::from_secs(5), kernel.read_exact(&mut buf)).await.unwrap().unwrap();
    assert_eq!(buf, msg);
}

#[tokio::test]
async fn attach_relays_and_reconnects() {
    let f = start().await;
    f.server.open_pairing_with_pin("333333", Duration::from_secs(60));
    client::connect(&f.client, &f.addr, Some("333333")).await.unwrap();

    let import = Arc::new(MockImport::default());
    let (tx, mut rx) = mpsc::unbounded_channel();
    let cfg = f.client.clone();
    let target = Target::Addr(f.addr.clone());
    let backend: Arc<dyn ImportBackend> = import.clone();
    let task = tokio::spawn(async move {
        client::attach_forever(&cfg, &target, "1-1", backend, move |e| {
            let _ = tx.send(e);
        })
        .await
    });

    // URB bytes flow kernel -> client -> TLS -> server -> kernel and back.
    assert_eq!(expect_attached(&mut rx).await, 1);
    let mut k1 = import.kernel_ends.lock().unwrap().pop().unwrap();
    echo(&mut k1, b"CMD_SUBMIT").await;

    // The server side goes away (simulated network/device loss); the client
    // must reconnect and re-attach on its own.
    k1.write_all(b"bye").await.unwrap();
    assert_eq!(expect_attached(&mut rx).await, 2);
    let mut k2 = import.kernel_ends.lock().unwrap().pop().unwrap();
    echo(&mut k2, b"after-reconnect").await;

    // Detaching locally (kernel closes its socket) ends the loop cleanly.
    drop(k2);
    let res = tokio::time::timeout(Duration::from_secs(10), task).await.unwrap().unwrap();
    assert!(res.is_ok(), "{res:?}");
}

/// Starts `attach_forever` for `device` and forwards its events.
fn spawn_attach(
    f: &Fixture,
    device: &str,
) -> (Arc<MockImport>, mpsc::UnboundedReceiver<AttachEvent>, tokio::task::JoinHandle<Result<()>>) {
    let import = Arc::new(MockImport::default());
    let (tx, rx) = mpsc::unbounded_channel();
    let (cfg, target, backend) = (f.client.clone(), Target::Addr(f.addr.clone()), import.clone());
    let device = device.to_string();
    let task = tokio::spawn(async move {
        client::attach_forever(&cfg, &target, &device, backend, move |e| {
            let _ = tx.send(e);
        })
        .await
    });
    (import, rx, task)
}

/// Waits for a failed attempt with the given API error code.
async fn expect_error(rx: &mut mpsc::UnboundedReceiver<AttachEvent>, code: &str) {
    loop {
        let ev = tokio::time::timeout(Duration::from_secs(20), rx.recv()).await.expect("no error reported").unwrap();
        if let AttachEvent::Disconnected { error } = ev {
            if error.code == code {
                return;
            }
        }
    }
}

#[tokio::test]
async fn attach_waits_for_unplugged_device() {
    let f = start().await;
    f.server.open_pairing_with_pin("444444", Duration::from_secs(60));
    client::connect(&f.client, &f.addr, Some("444444")).await.unwrap();

    // Unplugged devices stay listed; attaching waits for them.
    f.export.plugged.store(false, Ordering::SeqCst);
    let mut s = client::connect(&f.client, &f.addr, None).await.unwrap();
    let listed = s.list().await.unwrap();
    assert!(!listed[0].present);
    assert_eq!(listed[0].info.id_vendor, 0x1234);

    let (_import, mut rx, task) = spawn_attach(&f, DEVICE_ID);
    expect_error(&mut rx, "no_such_device").await;
    expect_error(&mut rx, "no_such_device").await;
    assert!(!task.is_finished(), "a missing device is not a permanent error");

    // Plugged in again: attached without doing anything.
    f.export.plugged.store(true, Ordering::SeqCst);
    let id = loop {
        let ev = tokio::time::timeout(Duration::from_secs(20), rx.recv()).await.unwrap().unwrap();
        if let AttachEvent::Attached { id, .. } = ev {
            break id;
        }
    };
    assert_eq!(id, DEVICE_ID);
    task.abort();
}

#[tokio::test]
async fn access_is_checked_and_revoking_disconnects() {
    let f = start().await;
    f.server.open_pairing_with_pin("555555", Duration::from_secs(60));
    client::connect(&f.client, &f.addr, Some("555555")).await.unwrap();
    let me = f.client.identity.fingerprint();

    // Restricted: the device is visible but marked, and importing is refused.
    *f.export.allowed.lock().unwrap() = Allowed::Only(Default::default());
    let mut s = client::connect(&f.client, &f.addr, None).await.unwrap();
    assert!(!s.list().await.unwrap()[0].allowed);
    let e = s.import(DEVICE_ID).await.err().unwrap();
    assert_eq!(remote_code(&e), Some(ErrorCode::AccessDenied));

    // Allowed: attaches.
    *f.export.allowed.lock().unwrap() = Allowed::Only([me.clone()].into());
    let (_import, mut rx, task) = spawn_attach(&f, DEVICE_ID);
    expect_attached(&mut rx).await;
    assert_eq!(f.server.in_use().get(DEVICE_ID).map(|u| u.1.clone()), Some(me));

    // Revoked: the session ends at once and the client waits for permission.
    *f.export.allowed.lock().unwrap() = Allowed::Only(Default::default());
    assert_eq!(f.server.enforce().unwrap(), 1);
    expect_error(&mut rx, "access_denied").await;
    assert!(!task.is_finished());
    task.abort();
}

#[tokio::test]
async fn attach_gives_up_on_permanent_errors() {
    let f = start().await;
    f.server.open_pairing_with_pin("666666", Duration::from_secs(60));
    client::connect(&f.client, &f.addr, Some("666666")).await.unwrap();
    // The server forgot this computer: pairing again is needed.
    f.server_trust.remove(&f.client.identity.fingerprint()).unwrap();
    let import: Arc<dyn ImportBackend> = Arc::new(MockImport::default());
    let res = client::attach_forever(&f.client, &Target::Addr(f.addr.clone()), DEVICE_ID, import, |_| {}).await;
    assert!(matches!(res.unwrap_err().downcast_ref::<ClientError>(), Some(ClientError::PairingRequired { .. })));
}
