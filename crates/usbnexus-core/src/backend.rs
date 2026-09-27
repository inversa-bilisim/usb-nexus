// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! Platform backends.
//!
//! A backend connects USB Nexus to the operating system's USB/IP implementation.
//! It never parses URBs itself: it hands one end of a loopback TCP socket to
//! the OS stack and returns the other end, which USB Nexus relays over TLS.

use std::collections::{BTreeSet, HashMap};
use std::io;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use usbnexus_proto::DeviceInfo;

use crate::access::{DeviceAccess, Policy};
use crate::device_id::DeviceId;
use crate::handover::{DeviceKind, Handover};

/// A local USB device that can be exported.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalDevice {
    pub info: DeviceInfo,
    pub product: Option<String>,
    pub manufacturer: Option<String>,
    /// Serial number string, if the device has one.
    pub serial: Option<String>,
    /// Kernel driver currently bound to the device, if any.
    pub driver: Option<String>,
}

/// A boxed, sendable future (object-safe async trait methods).
pub type BoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;

/// Which paired computers may use an offered device.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Allowed {
    Everyone,
    /// Only the computers with these certificate fingerprints.
    Only(BTreeSet<String>),
}

impl Allowed {
    pub fn allows(&self, fingerprint: &str) -> bool {
        match self {
            Allowed::Everyone => true,
            Allowed::Only(set) => set.contains(fingerprint),
        }
    }
}

/// A device a server offers. Shared devices that are unplugged are still
/// offered, without `device`, so clients can wait for them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Offered {
    pub id: DeviceId,
    /// The device, while it is plugged in.
    pub device: Option<LocalDevice>,
    /// Names, also remembered while the device is unplugged.
    pub product: Option<String>,
    pub manufacturer: Option<String>,
    pub allowed: Allowed,
    /// Idle time after which the device goes to the next waiting computer
    /// (see [`crate::handover`]); `None`: it stays with its user.
    pub handover: Option<std::time::Duration>,
}

impl Offered {
    /// Whether a client asking for `wanted` (an identity, or a bus id from
    /// an older client) means this device.
    pub fn is(&self, wanted: &DeviceId) -> bool {
        if self.id == *wanted {
            return true;
        }
        match wanted {
            DeviceId::Legacy { busid } => self.device.as_ref().is_some_and(|d| d.info.busid == *busid),
            _ => false,
        }
    }
}

/// Server side: shares local devices.
pub trait ExportBackend: Send + Sync + 'static {
    /// Devices this server offers.
    fn list(&self) -> Result<Vec<Offered>>;

    /// Hands `busid` to the USB/IP stack and returns the socket that carries
    /// its URB traffic. Dropping the socket ends the export.
    fn export<'a>(&'a self, busid: &'a str) -> BoxFuture<'a, Result<tokio::net::TcpStream>>;
}

/// Access to all USB devices of this machine (the OS side of exporting).
pub trait DeviceHost: Send + Sync + 'static {
    /// Every USB device currently connected.
    fn list_all(&self) -> Result<Vec<LocalDevice>>;

    /// Hands `busid` to the USB/IP stack; see [`ExportBackend::export`].
    fn export<'a>(&'a self, busid: &'a str) -> BoxFuture<'a, Result<tokio::net::TcpStream>>;

    /// Returns `busid` to its normal driver after sharing ends.
    fn release(&self, busid: &str) -> Result<()>;
}

/// A device the user chose to share, as saved in the configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "SharedRepr")]
pub struct SharedDevice {
    pub id: DeviceId,
    /// Last known names, shown while the device is unplugged.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[serde(default)]
    pub access: DeviceAccess,
    #[serde(default, skip_serializing_if = "Handover::is_default")]
    pub handover: Handover,
}

impl SharedDevice {
    pub fn new(id: DeviceId) -> Self {
        SharedDevice {
            id,
            product: None,
            manufacturer: None,
            access: DeviceAccess::default(),
            handover: Handover::default(),
        }
    }
}

/// Saved form: older configurations list bus ids only.
#[derive(Deserialize)]
#[serde(untagged)]
enum SharedRepr {
    Busid(String),
    Full {
        id: DeviceId,
        #[serde(default)]
        product: Option<String>,
        #[serde(default)]
        manufacturer: Option<String>,
        #[serde(default)]
        access: DeviceAccess,
        #[serde(default)]
        handover: Handover,
    },
}

impl From<SharedRepr> for SharedDevice {
    fn from(r: SharedRepr) -> Self {
        match r {
            SharedRepr::Busid(b) => SharedDevice::new(DeviceId::parse(&b)),
            SharedRepr::Full { id, product, manufacturer, access, handover } => {
                SharedDevice { id, product, manufacturer, access, handover }
            }
        }
    }
}

/// A shared device matched against the devices plugged in right now.
#[derive(Debug, Clone)]
pub struct Resolved {
    pub shared: SharedDevice,
    pub device: Option<LocalDevice>,
}

/// Matches shared devices with connected ones. Each connected device
/// belongs to at most one entry (the first that matches).
pub fn resolve(shared: &[SharedDevice], connected: &[LocalDevice]) -> Vec<Resolved> {
    let mut taken = vec![false; connected.len()];
    shared
        .iter()
        .map(|s| {
            let hit = connected.iter().enumerate().position(|(i, d)| !taken[i] && s.id.matches(d));
            if let Some(i) = hit {
                taken[i] = true;
            }
            Resolved { shared: s.clone(), device: hit.map(|i| connected[i].clone()) }
        })
        .collect()
}

/// Exports the subset of a [`DeviceHost`]'s devices the user chose to share.
/// Devices are identified by [`DeviceId`], so a device keeps being shared
/// when it is unplugged and plugged in again, on any port if it has a
/// serial number. The set can change while the server runs.
pub struct SharedExport {
    /// Replaced when the computer's roles change.
    host: std::sync::RwLock<Arc<dyn DeviceHost>>,
    state: Mutex<ShareState>,
    /// Devices seen by the last listing. Listing can block for a long time
    /// (on Windows it asks every hub for descriptors), so only
    /// [`SharedExport::refresh`], which the service runs on a blocking
    /// thread, lists devices; everything else, often called from async
    /// code, uses this copy.
    connected: Mutex<Option<Vec<LocalDevice>>>,
}

struct ShareState {
    shared: Vec<SharedDevice>,
    policy: Policy,
    /// Where each shared device was plugged in at the last refresh.
    ports: HashMap<DeviceId, String>,
}

fn not_shared(what: &str) -> anyhow::Error {
    crate::api::ApiError::new("no_such_device", format!("{what} is not shared")).into()
}

impl SharedExport {
    pub fn new(host: Arc<dyn DeviceHost>, shared: impl IntoIterator<Item = SharedDevice>, policy: Policy) -> Self {
        let state = ShareState { shared: shared.into_iter().collect(), policy, ports: HashMap::new() };
        SharedExport { host: std::sync::RwLock::new(host), state: Mutex::new(state), connected: Mutex::new(None) }
    }

    /// Devices of the last listing; lists them now if there was none yet.
    fn connected(&self) -> Result<Vec<LocalDevice>> {
        if let Some(list) = &*self.connected.lock().unwrap() {
            return Ok(list.clone());
        }
        self.list_now()
    }

    fn list_now(&self) -> Result<Vec<LocalDevice>> {
        let list = self.host().list_all()?;
        *self.connected.lock().unwrap() = Some(list.clone());
        Ok(list)
    }

    pub fn host(&self) -> Arc<dyn DeviceHost> {
        self.host.read().unwrap().clone()
    }

    /// Switches to another host (roles changed). Until the next
    /// [`SharedExport::refresh`] no devices are listed.
    pub fn set_host(&self, host: Arc<dyn DeviceHost>) {
        *self.host.write().unwrap() = host;
        *self.connected.lock().unwrap() = Some(vec![]);
    }

    /// The shared devices, in the order they were shared.
    pub fn shared(&self) -> Vec<SharedDevice> {
        self.state.lock().unwrap().shared.clone()
    }

    pub fn policy(&self) -> Policy {
        self.state.lock().unwrap().policy
    }

    pub fn set_policy(&self, policy: Policy) {
        self.state.lock().unwrap().policy = policy;
    }

    /// Every connected device, and the shared devices matched against them.
    pub fn snapshot(&self) -> Result<(Vec<LocalDevice>, Vec<Resolved>)> {
        let connected = self.connected()?;
        let resolved = resolve(&self.shared(), &connected);
        Ok((connected, resolved))
    }

    /// Updates remembered names and replaces bus ids from older
    /// configurations with device identities. Ports that shared devices
    /// left (unplugged, or moved elsewhere) are released, so the OS does not
    /// keep reserving them. Returns whether anything changed (and should be
    /// saved).
    pub fn refresh(&self) -> Result<bool> {
        let connected = self.list_now()?;
        let mut st = self.state.lock().unwrap();
        let resolved = resolve(&st.shared, &connected);
        let mut changed = false;
        let mut out: Vec<SharedDevice> = Vec::with_capacity(resolved.len());
        let mut ports = HashMap::new();
        for r in resolved {
            let mut s = r.shared;
            if let Some(d) = &r.device {
                ports.insert(DeviceId::of(d), d.info.busid.clone());
                if s.id.is_legacy() {
                    s.id = DeviceId::of(d);
                    changed = true;
                }
                for (have, now) in [(&mut s.product, &d.product), (&mut s.manufacturer, &d.manufacturer)] {
                    if now.is_some() && have != now {
                        have.clone_from(now);
                        changed = true;
                    }
                }
            }
            if out.iter().any(|o| o.id == s.id) {
                changed = true; // the same device listed twice after migration
                continue;
            }
            out.push(s);
        }
        st.shared = out;
        let left: Vec<String> = st
            .ports
            .iter()
            .filter(|(id, old)| ports.get(*id) != Some(*old) && !ports.values().any(|b| b == *old))
            .map(|(_, old)| old.clone())
            .collect();
        st.ports = ports;
        drop(st);
        for busid in left {
            if let Err(e) = self.host().release(&busid) {
                tracing::debug!(busid, "could not release a port: {e:#}");
            }
        }
        Ok(changed)
    }

    /// Index of the shared entry meant by `wanted`: an identity, or a bus
    /// id of a connected shared device.
    fn find(shared: &[SharedDevice], connected: &[LocalDevice], wanted: &DeviceId) -> Option<usize> {
        if let Some(i) = shared.iter().position(|s| s.id == *wanted) {
            return Some(i);
        }
        let DeviceId::Legacy { busid } = wanted else { return None };
        resolve(shared, connected).iter().position(|r| r.device.as_ref().is_some_and(|d| d.info.busid == *busid))
    }

    /// Shares or stops sharing `device` (an identity, or the bus id of a
    /// connected device); returns whether anything changed. A device that is
    /// no longer shared is handed back to its normal driver.
    pub fn set_shared(&self, device: &str, shared: bool) -> Result<bool> {
        let wanted = DeviceId::parse(device);
        let connected = self.connected()?;
        let mut st = self.state.lock().unwrap();
        let index = Self::find(&st.shared, &connected, &wanted);
        if shared {
            if index.is_some() {
                return Ok(false);
            }
            let present = connected.iter().find(|d| wanted.matches(d));
            let entry = match present {
                Some(d) => SharedDevice {
                    product: d.product.clone(),
                    manufacturer: d.manufacturer.clone(),
                    ..SharedDevice::new(DeviceId::of(d))
                },
                // Not plugged in: shared as soon as it appears.
                None => SharedDevice::new(wanted),
            };
            if st.shared.iter().any(|s| s.id == entry.id) {
                return Ok(false);
            }
            st.shared.push(entry);
            return Ok(true);
        }
        let Some(index) = index else { return Ok(false) };
        let before = resolve(&st.shared, &connected);
        st.shared.remove(index);
        drop(st);
        if let Some(d) = &before[index].device {
            self.host().release(&d.info.busid)?;
        }
        Ok(true)
    }

    /// Changes the automatic handover setting of a shared device.
    pub fn set_handover(&self, device: &str, handover: Handover) -> Result<()> {
        let wanted = DeviceId::parse(device);
        let connected = if wanted.is_legacy() { self.connected()? } else { vec![] };
        let mut st = self.state.lock().unwrap();
        let i = Self::find(&st.shared, &connected, &wanted).ok_or_else(|| not_shared(device))?;
        st.shared[i].handover = handover;
        Ok(())
    }

    /// Changes who may use a shared device.
    pub fn set_access(&self, device: &str, access: DeviceAccess) -> Result<()> {
        let wanted = DeviceId::parse(device);
        let connected = if wanted.is_legacy() { self.connected()? } else { vec![] };
        let mut st = self.state.lock().unwrap();
        let i = Self::find(&st.shared, &connected, &wanted).ok_or_else(|| not_shared(device))?;
        st.shared[i].access = access;
        Ok(())
    }

    /// Lets the computer `fingerprint` use exactly the devices in `devices`
    /// (among those whose access is limited to a list).
    pub fn set_client_devices(&self, fingerprint: &str, devices: &[String]) -> Result<()> {
        let connected = self.connected()?;
        let mut st = self.state.lock().unwrap();
        let mut chosen = BTreeSet::new();
        for d in devices {
            chosen.insert(Self::find(&st.shared, &connected, &DeviceId::parse(d)).ok_or_else(|| not_shared(d))?);
        }
        for (i, s) in st.shared.iter_mut().enumerate() {
            if chosen.contains(&i) {
                s.access.allowed.insert(fingerprint.to_string());
            } else {
                s.access.allowed.remove(fingerprint);
            }
        }
        Ok(())
    }

    /// Removes a computer from every device's list (e.g. when it is forgotten).
    pub fn forget_client(&self, fingerprint: &str) -> bool {
        let mut st = self.state.lock().unwrap();
        let mut changed = false;
        for s in &mut st.shared {
            changed |= s.access.allowed.remove(fingerprint);
        }
        changed
    }

    fn offered(&self, resolved: Vec<Resolved>) -> Vec<Offered> {
        let policy = self.policy();
        resolved
            .into_iter()
            .map(|r| Offered {
                allowed: if r.shared.access.is_open(policy) {
                    Allowed::Everyone
                } else {
                    Allowed::Only(r.shared.access.allowed.clone())
                },
                handover: r.device.as_ref().and_then(|d| r.shared.handover.idle_time(DeviceKind::of(d))),
                product: r.device.as_ref().and_then(|d| d.product.clone()).or(r.shared.product),
                manufacturer: r.device.as_ref().and_then(|d| d.manufacturer.clone()).or(r.shared.manufacturer),
                id: r.shared.id,
                device: r.device,
            })
            .collect()
    }
}

impl ExportBackend for SharedExport {
    fn list(&self) -> Result<Vec<Offered>> {
        let (_, resolved) = self.snapshot()?;
        Ok(self.offered(resolved))
    }

    fn export<'a>(&'a self, busid: &'a str) -> BoxFuture<'a, Result<tokio::net::TcpStream>> {
        let shared =
            self.snapshot().map(|(_, r)| r.iter().any(|r| r.device.as_ref().is_some_and(|d| d.info.busid == busid)));
        match shared {
            Ok(true) => {
                let host = self.host();
                Box::pin(async move { host.export(busid).await })
            }
            Ok(false) => Box::pin(async move { Err(not_shared(busid)) }),
            Err(e) => Box::pin(async move { Err(e) }),
        }
    }
}

/// Placeholder for platforms where sharing local devices is not available
/// yet; every call fails with the `unsupported` API error.
pub struct UnsupportedHost;

impl DeviceHost for UnsupportedHost {
    fn list_all(&self) -> Result<Vec<LocalDevice>> {
        Err(crate::api::ApiError::new("unsupported", "sharing devices is not supported on this platform yet").into())
    }

    fn export<'a>(&'a self, _busid: &'a str) -> BoxFuture<'a, Result<tokio::net::TcpStream>> {
        Box::pin(async move { self.list_all().map(|_| unreachable!()) })
    }

    fn release(&self, _busid: &str) -> Result<()> {
        Ok(())
    }
}

/// Host for a computer not set up to share its devices: lists none and
/// never touches USB devices.
pub struct NoHost;

impl DeviceHost for NoHost {
    fn list_all(&self) -> Result<Vec<LocalDevice>> {
        Ok(vec![])
    }

    fn export<'a>(&'a self, busid: &'a str) -> BoxFuture<'a, Result<tokio::net::TcpStream>> {
        Box::pin(async move { Err(not_shared(busid)) })
    }

    fn release(&self, _busid: &str) -> Result<()> {
        Ok(())
    }
}

/// Placeholder for platforms that cannot attach remote devices (macOS has
/// no virtual USB host controller available to third parties).
pub struct UnsupportedImport;

impl ImportBackend for UnsupportedImport {
    fn attach<'a>(&'a self, _device: &'a DeviceInfo) -> BoxFuture<'a, Result<(u32, tokio::net::TcpStream)>> {
        Box::pin(async {
            Err(crate::api::ApiError::new("unsupported", "using remote devices is not supported on this platform")
                .into())
        })
    }

    fn detach(&self, _port: u32) -> Result<()> {
        Ok(())
    }
}

/// Client side: attaches remote devices to a virtual host controller.
pub trait ImportBackend: Send + Sync + 'static {
    /// Attaches `device`; returns the virtual port and the socket that
    /// carries its URB traffic. Dropping the socket detaches the device.
    fn attach<'a>(&'a self, device: &'a DeviceInfo) -> BoxFuture<'a, Result<(u32, tokio::net::TcpStream)>>;

    /// Explicitly detaches a virtual port.
    fn detach(&self, port: u32) -> Result<()>;
}

/// Creates a connected pair of loopback TCP sockets.
///
/// The accepted connection is checked against the connecting socket's
/// address, so another local process cannot slip in between.
pub fn loopback_pair() -> io::Result<(TcpStream, TcpStream)> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let ours = TcpStream::connect(listener.local_addr()?)?;
    let expected = ours.local_addr()?;
    loop {
        let (theirs, from) = listener.accept()?;
        if from == expected {
            ours.set_nodelay(true)?;
            theirs.set_nodelay(true)?;
            return Ok((ours, theirs));
        }
    }
}

/// Converts a std socket to a tokio socket.
pub fn into_tokio(s: TcpStream) -> io::Result<tokio::net::TcpStream> {
    s.set_nonblocking(true)?;
    tokio::net::TcpStream::from_std(s)
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};

    use super::demo::DemoHost;
    use super::*;
    use crate::access::DeviceMode;

    const STICK: &str = "0781:5567:4C530001231120115142";

    #[test]
    fn sharing_by_identity() {
        let export = SharedExport::new(Arc::new(DemoHost), [], Policy::Open);
        // Shared by bus id: stored by identity.
        assert!(export.set_shared("1-1", true).unwrap());
        assert!(!export.set_shared(STICK, true).unwrap(), "already shared");
        assert_eq!(export.shared()[0].id.to_string(), STICK);
        assert_eq!(export.shared()[0].product.as_deref(), Some("Cruzer Blade"));
        // A device that is not plugged in can be shared ahead of time.
        assert!(export.set_shared("1234:5678:ABC", true).unwrap());
        let offered = export.list().unwrap();
        assert_eq!(offered.len(), 2);
        assert!(offered[0].device.is_some() && offered[1].device.is_none());
        assert!(offered[0].is(&DeviceId::parse("1-1")), "older clients ask by bus id");
        // Unsharing by bus id or identity.
        assert!(export.set_shared("1-1", false).unwrap());
        assert!(export.set_shared("1234:5678:ABC", false).unwrap());
        assert!(export.shared().is_empty());
    }

    #[test]
    fn access_lists() {
        let export = SharedExport::new(Arc::new(DemoHost), [], Policy::Restricted);
        export.set_shared("1-1", true).unwrap();
        export.set_shared("1-2", true).unwrap();
        let allowed = |i: usize| export.list().unwrap()[i].allowed.clone();
        assert_eq!(allowed(0), Allowed::Only(BTreeSet::new()));

        export.set_client_devices("pc1", &[STICK.into()]).unwrap();
        assert!(allowed(0).allows("pc1") && !allowed(1).allows("pc1"));
        export.set_client_devices("pc1", &["046d:c52b@1-2".into()]).unwrap();
        assert!(!allowed(0).allows("pc1") && allowed(1).allows("pc1"));
        assert!(export.set_client_devices("pc1", &["nope".into()]).is_err());

        export.set_access(STICK, DeviceAccess { mode: DeviceMode::Open, allowed: BTreeSet::new() }).unwrap();
        assert_eq!(allowed(0), Allowed::Everyone);
        export.set_policy(Policy::Open);
        assert_eq!(allowed(1), Allowed::Everyone);

        assert!(export.forget_client("pc1"));
        assert!(export.shared().iter().all(|s| s.access.allowed.is_empty()));
    }

    #[test]
    fn older_configurations_load() {
        let v: Vec<SharedDevice> = serde_json::from_str(r#"["1-1", {"id": "046d:c52b@1-2"}]"#).unwrap();
        assert_eq!(v[0], SharedDevice::new(DeviceId::parse("1-1")));
        assert_eq!(v[1].id.to_string(), "046d:c52b@1-2");
        let export = SharedExport::new(Arc::new(DemoHost), v, Policy::Open);
        assert!(export.refresh().unwrap());
        assert_eq!(export.shared()[0].id.to_string(), STICK);
        assert!(!export.refresh().unwrap(), "nothing left to migrate");
    }

    /// Counts how often devices are listed.
    #[derive(Default)]
    struct CountingHost(std::sync::atomic::AtomicUsize);

    impl DeviceHost for CountingHost {
        fn list_all(&self) -> Result<Vec<LocalDevice>> {
            self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            DemoHost.list_all()
        }
        fn export<'a>(&'a self, busid: &'a str) -> BoxFuture<'a, Result<tokio::net::TcpStream>> {
            Box::pin(async move { anyhow::bail!("not exporting {busid}") })
        }
        fn release(&self, _busid: &str) -> Result<()> {
            Ok(())
        }
    }

    /// Listing can block (Windows asks every hub, including the virtual one
    /// whose answers travel through this very service), so only `refresh`
    /// lists devices; requests use the last listing.
    #[test]
    fn only_refresh_lists_devices() {
        let host = Arc::new(CountingHost::default());
        let export = SharedExport::new(host.clone(), [], Policy::Open);
        let listings = || host.0.load(std::sync::atomic::Ordering::SeqCst);
        export.refresh().unwrap();
        assert_eq!(listings(), 1);
        assert!(export.set_shared("1-1", true).unwrap());
        export.set_access(STICK, DeviceAccess::default()).unwrap();
        export.set_client_devices("fp", &[STICK.to_string()]).unwrap();
        export.snapshot().unwrap();
        export.list().unwrap();
        assert_eq!(listings(), 1, "requests reuse the last listing");
        export.refresh().unwrap();
        assert_eq!(listings(), 2);
    }

    #[test]
    fn loopback_pair_is_connected() {
        let (mut a, mut b) = super::loopback_pair().unwrap();
        a.write_all(b"ping").unwrap();
        let mut buf = [0u8; 4];
        b.read_exact(&mut buf).unwrap();
        assert_eq!(&buf, b"ping");
    }
}

/// Simulated devices for demos, UI development and tests without hardware.
pub mod demo {
    use std::sync::atomic::{AtomicU32, Ordering};

    use anyhow::{bail, Result};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use usbnexus_proto::{DeviceInfo, InterfaceInfo, Speed};

    use super::{into_tokio, loopback_pair, BoxFuture, DeviceHost, ImportBackend, LocalDevice};

    fn device(
        busid: &str,
        devnum: u32,
        (vid, pid): (u16, u16),
        speed: Speed,
        (mfr, product): (&str, &str),
        serial: Option<&str>,
    ) -> LocalDevice {
        LocalDevice {
            info: DeviceInfo {
                path: format!("/sys/devices/demo/{busid}"),
                busid: busid.into(),
                busnum: 1,
                devnum,
                speed,
                id_vendor: vid,
                id_product: pid,
                bcd_device: 0x0100,
                device_class: 0,
                device_subclass: 0,
                device_protocol: 0,
                configuration_value: 1,
                num_configurations: 1,
                // Class by product: storage, HID (receiver), smart card reader.
                interfaces: vec![InterfaceInfo {
                    class: match product {
                        "USB Receiver" => 0x03,
                        "Smart Card Reader" => 0x0b,
                        _ => 0x08,
                    },
                    ..InterfaceInfo::default()
                }],
            },
            product: Some(product.into()),
            manufacturer: Some(mfr.into()),
            serial: serial.map(str::to_string),
            driver: Some("demo".into()),
        }
    }

    /// A few typical devices; URB traffic is echoed back.
    #[derive(Default)]
    pub struct DemoHost;

    impl DeviceHost for DemoHost {
        fn list_all(&self) -> Result<Vec<LocalDevice>> {
            Ok(vec![
                device(
                    "1-1",
                    2,
                    (0x0781, 0x5567),
                    Speed::High,
                    ("SanDisk", "Cruzer Blade"),
                    Some("4C530001231120115142"),
                ),
                device("1-2", 3, (0x046d, 0xc52b), Speed::Full, ("Logitech", "USB Receiver"), None),
                device(
                    "2-1",
                    4,
                    (0x04e8, 0x61f5),
                    Speed::Super,
                    ("Samsung", "Portable SSD T5"),
                    Some("S3UJNB0K512345"),
                ),
                device(
                    "1-4",
                    5,
                    (0x0a5c, 0x5832),
                    Speed::Full,
                    ("Broadcom", "Smart Card Reader"),
                    Some("0123456789ABCD"),
                ),
            ])
        }

        fn export<'a>(&'a self, busid: &'a str) -> BoxFuture<'a, Result<tokio::net::TcpStream>> {
            Box::pin(async move {
                if !self.list_all()?.iter().any(|d| d.info.busid == busid) {
                    bail!("no USB device {busid}");
                }
                let (ours, kernel) = loopback_pair()?;
                let mut kernel = into_tokio(kernel)?;
                tokio::spawn(async move {
                    let mut buf = [0u8; 4096];
                    while let Ok(n) = kernel.read(&mut buf).await {
                        if n == 0 || kernel.write_all(&buf[..n]).await.is_err() {
                            break;
                        }
                    }
                });
                Ok(into_tokio(ours)?)
            })
        }

        fn release(&self, _busid: &str) -> Result<()> {
            Ok(())
        }
    }

    /// Virtual ports that keep the kernel end open until detached.
    #[derive(Default)]
    pub struct DemoImport {
        next: AtomicU32,
    }

    impl ImportBackend for DemoImport {
        fn attach<'a>(&'a self, _device: &'a DeviceInfo) -> BoxFuture<'a, Result<(u32, tokio::net::TcpStream)>> {
            Box::pin(async move {
                let (ours, kernel) = loopback_pair()?;
                let mut kernel = into_tokio(kernel)?;
                // Hold the "kernel" end open, like an attached device would.
                tokio::spawn(async move {
                    let mut buf = [0u8; 4096];
                    while let Ok(n) = kernel.read(&mut buf).await {
                        if n == 0 {
                            break;
                        }
                    }
                });
                Ok((self.next.fetch_add(1, Ordering::Relaxed), into_tokio(ours)?))
            })
        }

        fn detach(&self, _port: u32) -> Result<()> {
            Ok(())
        }
    }
}
