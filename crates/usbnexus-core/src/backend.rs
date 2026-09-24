// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! Platform backends.
//!
//! A backend connects USB Nexus to the operating system's USB/IP implementation.
//! It never parses URBs itself: it hands one end of a loopback TCP socket to
//! the OS stack and returns the other end, which USB Nexus relays over TLS.

use std::collections::BTreeSet;
use std::io;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

use anyhow::Result;
use usbnexus_proto::DeviceInfo;

/// A local USB device that can be exported.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalDevice {
    pub info: DeviceInfo,
    pub product: Option<String>,
    pub manufacturer: Option<String>,
    /// Kernel driver currently bound to the device, if any.
    pub driver: Option<String>,
}

/// Server side: shares local devices.
pub trait ExportBackend: Send + Sync + 'static {
    /// Devices this server offers.
    fn list(&self) -> Result<Vec<LocalDevice>>;

    /// Hands `busid` to the USB/IP stack and returns the socket that carries
    /// its URB traffic. Dropping the socket ends the export.
    fn export(&self, busid: &str) -> Result<tokio::net::TcpStream>;
}

/// Access to all USB devices of this machine (the OS side of exporting).
pub trait DeviceHost: Send + Sync + 'static {
    /// Every USB device currently connected.
    fn list_all(&self) -> Result<Vec<LocalDevice>>;

    /// Hands `busid` to the USB/IP stack; see [`ExportBackend::export`].
    fn export(&self, busid: &str) -> Result<tokio::net::TcpStream>;

    /// Returns `busid` to its normal driver after sharing ends.
    fn release(&self, busid: &str) -> Result<()>;
}

/// Exports the subset of a [`DeviceHost`]'s devices the user chose to share.
/// The set can change while the server runs.
pub struct SharedExport {
    host: Arc<dyn DeviceHost>,
    shared: Mutex<BTreeSet<String>>,
}

impl SharedExport {
    pub fn new(host: Arc<dyn DeviceHost>, shared: impl IntoIterator<Item = String>) -> Self {
        SharedExport { host, shared: Mutex::new(shared.into_iter().collect()) }
    }

    pub fn host(&self) -> &Arc<dyn DeviceHost> {
        &self.host
    }

    pub fn shared(&self) -> BTreeSet<String> {
        self.shared.lock().unwrap().clone()
    }

    pub fn is_shared(&self, busid: &str) -> bool {
        self.shared.lock().unwrap().contains(busid)
    }

    /// Adds or removes a device; returns whether anything changed. A removed
    /// device is handed back to its normal driver.
    pub fn set_shared(&self, busid: &str, shared: bool) -> Result<bool> {
        let changed = {
            let mut set = self.shared.lock().unwrap();
            if shared {
                set.insert(busid.to_string())
            } else {
                set.remove(busid)
            }
        };
        if changed && !shared {
            self.host.release(busid)?;
        }
        Ok(changed)
    }
}

impl ExportBackend for SharedExport {
    fn list(&self) -> Result<Vec<LocalDevice>> {
        let shared = self.shared();
        Ok(self.host.list_all()?.into_iter().filter(|d| shared.contains(&d.info.busid)).collect())
    }

    fn export(&self, busid: &str) -> Result<tokio::net::TcpStream> {
        if !self.is_shared(busid) {
            anyhow::bail!("{busid} is not shared");
        }
        self.host.export(busid)
    }
}

/// Client side: attaches remote devices to a virtual host controller.
pub trait ImportBackend: Send + Sync + 'static {
    /// Attaches `device`; returns the virtual port and the socket that
    /// carries its URB traffic. Dropping the socket detaches the device.
    fn attach(&self, device: &DeviceInfo) -> Result<(u32, tokio::net::TcpStream)>;

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

    use super::{into_tokio, loopback_pair, DeviceHost, ImportBackend, LocalDevice};

    fn device(busid: &str, devnum: u32, vid: u16, pid: u16, speed: Speed, mfr: &str, product: &str) -> LocalDevice {
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
                interfaces: vec![InterfaceInfo::default()],
            },
            product: Some(product.into()),
            manufacturer: Some(mfr.into()),
            driver: Some("demo".into()),
        }
    }

    /// A few typical devices; URB traffic is echoed back.
    #[derive(Default)]
    pub struct DemoHost;

    impl DeviceHost for DemoHost {
        fn list_all(&self) -> Result<Vec<LocalDevice>> {
            Ok(vec![
                device("1-1", 2, 0x0781, 0x5567, Speed::High, "SanDisk", "Cruzer Blade"),
                device("1-2", 3, 0x046d, 0xc52b, Speed::Full, "Logitech", "USB Receiver"),
                device("2-1", 4, 0x04e8, 0x61f5, Speed::Super, "Samsung", "Portable SSD T5"),
                device("1-4", 5, 0x0a5c, 0x5832, Speed::Full, "Broadcom", "Smart Card Reader"),
            ])
        }

        fn export(&self, busid: &str) -> Result<tokio::net::TcpStream> {
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
        fn attach(&self, _device: &DeviceInfo) -> Result<(u32, tokio::net::TcpStream)> {
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
        }

        fn detach(&self, _port: u32) -> Result<()> {
            Ok(())
        }
    }
}
