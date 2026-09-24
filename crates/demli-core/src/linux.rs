// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Demli contributors

//! Linux backends driving the in-kernel USB/IP drivers through sysfs.
//!
//! * Export: `usbip-host` (stub driver) — bind the device, then hand the
//!   kernel a socket via `usbip_sockfd`.
//! * Import: `vhci-hcd` (virtual host controller) — write
//!   `"<port> <sockfd> <devid> <speed>"` to its `attach` attribute.
//!
//! The sysfs root is configurable so the logic can be tested on a fake tree.

use std::fs;
use std::io::Write;
use std::os::fd::AsRawFd;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use demli_proto::{DeviceInfo, InterfaceInfo, Speed};

use crate::backend::{into_tokio, loopback_pair, ExportBackend, ImportBackend, LocalDevice};

const STUB_DRIVER: &str = "usbip-host";
/// `SDEV_ST_AVAILABLE` in the stub driver.
const STUB_AVAILABLE: &str = "1";
/// `VDEV_ST_NULL` in the vhci driver: port is free.
const VHCI_FREE: u32 = 4;

fn read_attr(dir: &Path, name: &str) -> Option<String> {
    fs::read_to_string(dir.join(name)).ok().map(|s| s.trim().to_string())
}

fn hex_attr<T: TryFrom<u32>>(dir: &Path, name: &str) -> Result<T> {
    let s = read_attr(dir, name).ok_or_else(|| anyhow!("missing {name}"))?;
    let v = u32::from_str_radix(&s, 16).with_context(|| format!("parsing {name}={s}"))?;
    T::try_from(v).map_err(|_| anyhow!("{name} out of range"))
}

fn dec_attr<T: std::str::FromStr + Default>(dir: &Path, name: &str) -> T {
    read_attr(dir, name).and_then(|s| s.parse().ok()).unwrap_or_default()
}

fn sysfs_write(path: &Path, value: &str) -> Result<()> {
    let mut f = fs::OpenOptions::new().write(true).open(path).with_context(|| format!("opening {}", path.display()))?;
    f.write_all(value.as_bytes()).with_context(|| format!("writing '{value}' to {}", path.display()))
}

/// Whether `name` looks like a USB device (not an interface or root hub).
fn is_device_busid(name: &str) -> bool {
    !name.contains(':') && !name.starts_with("usb") && name.contains('-')
}

/// Reads a device description from sysfs.
pub fn read_device(sys: &Path, busid: &str) -> Result<LocalDevice> {
    let dir = sys.join("bus/usb/devices").join(busid);
    if !dir.exists() {
        bail!("no USB device {busid}");
    }
    let configuration_value: u8 = dec_attr(&dir, "bConfigurationValue");
    let num_interfaces: u8 = dec_attr(&dir, "bNumInterfaces");
    let mut interfaces = Vec::new();
    for i in 0..num_interfaces {
        let idir = dir.join(format!("{busid}:{configuration_value}.{i}"));
        interfaces.push(InterfaceInfo {
            class: hex_attr(&idir, "bInterfaceClass").unwrap_or(0),
            subclass: hex_attr(&idir, "bInterfaceSubClass").unwrap_or(0),
            protocol: hex_attr(&idir, "bInterfaceProtocol").unwrap_or(0),
        });
    }
    let path = fs::canonicalize(&dir).unwrap_or(dir.clone()).to_string_lossy().into_owned();
    let driver =
        fs::read_link(dir.join("driver")).ok().and_then(|p| p.file_name().map(|n| n.to_string_lossy().into_owned()));
    Ok(LocalDevice {
        info: DeviceInfo {
            path,
            busid: busid.to_string(),
            busnum: dec_attr(&dir, "busnum"),
            devnum: dec_attr(&dir, "devnum"),
            speed: Speed::from_sysfs(&read_attr(&dir, "speed").unwrap_or_default()),
            id_vendor: hex_attr(&dir, "idVendor")?,
            id_product: hex_attr(&dir, "idProduct")?,
            bcd_device: hex_attr(&dir, "bcdDevice").unwrap_or(0),
            device_class: hex_attr(&dir, "bDeviceClass").unwrap_or(0),
            device_subclass: hex_attr(&dir, "bDeviceSubClass").unwrap_or(0),
            device_protocol: hex_attr(&dir, "bDeviceProtocol").unwrap_or(0),
            configuration_value,
            num_configurations: dec_attr(&dir, "bNumConfigurations"),
            interfaces,
        },
        product: read_attr(&dir, "product"),
        manufacturer: read_attr(&dir, "manufacturer"),
        driver,
    })
}

/// Lists all USB devices on this machine.
pub fn list_local(sys: &Path) -> Result<Vec<LocalDevice>> {
    let dir = sys.join("bus/usb/devices");
    let mut out = Vec::new();
    for entry in fs::read_dir(&dir).with_context(|| format!("reading {}", dir.display()))? {
        let name = entry?.file_name().to_string_lossy().into_owned();
        if is_device_busid(&name) {
            if let Ok(d) = read_device(sys, &name) {
                out.push(d);
            }
        }
    }
    out.sort_by(|a, b| a.info.busid.cmp(&b.info.busid));
    Ok(out)
}

/// Exports an allow-list of local devices through `usbip-host`.
pub struct LinuxExport {
    sys: PathBuf,
    busids: Vec<String>,
}

impl LinuxExport {
    pub fn new(sys: impl Into<PathBuf>, busids: Vec<String>) -> Self {
        LinuxExport { sys: sys.into(), busids }
    }

    fn stub_dir(&self) -> PathBuf {
        self.sys.join("bus/usb/drivers").join(STUB_DRIVER)
    }

    /// Binds a device to `usbip-host`, unbinding its current driver.
    pub fn bind(&self, busid: &str) -> Result<()> {
        let dev = self.sys.join("bus/usb/devices").join(busid);
        let current = read_device(&self.sys, busid)?.driver;
        if current.as_deref() == Some(STUB_DRIVER) {
            return Ok(());
        }
        let stub = self.stub_dir();
        if !stub.exists() {
            bail!("the usbip-host kernel module is not loaded (run: modprobe usbip-host)");
        }
        if current.is_some() {
            sysfs_write(&dev.join("driver/unbind"), busid)?;
        }
        sysfs_write(&stub.join("match_busid"), &format!("add {busid}"))?;
        sysfs_write(&stub.join("bind"), busid)
    }

    /// Returns a device to its normal driver.
    pub fn release(&self, busid: &str) -> Result<()> {
        let current = read_device(&self.sys, busid)?.driver;
        if current.as_deref() != Some(STUB_DRIVER) {
            return Ok(());
        }
        let stub = self.stub_dir();
        sysfs_write(&stub.join("unbind"), busid)?;
        sysfs_write(&stub.join("match_busid"), &format!("del {busid}"))?;
        sysfs_write(&self.sys.join("bus/usb/drivers_probe"), busid)
    }
}

impl ExportBackend for LinuxExport {
    fn list(&self) -> Result<Vec<LocalDevice>> {
        Ok(self.busids.iter().filter_map(|b| read_device(&self.sys, b).ok()).collect())
    }

    fn export(&self, busid: &str) -> Result<tokio::net::TcpStream> {
        if !self.busids.iter().any(|b| b == busid) {
            bail!("{busid} is not exported");
        }
        self.bind(busid)?;
        let dev = self.sys.join("bus/usb/devices").join(busid);
        let status = read_attr(&dev, "usbip_status").unwrap_or_default();
        if status != STUB_AVAILABLE {
            bail!("{busid} is not available (usbip_status={status})");
        }
        let (ours, kernel) = loopback_pair()?;
        sysfs_write(&dev.join("usbip_sockfd"), &kernel.as_raw_fd().to_string())?;
        // The kernel now holds its own reference to the socket.
        drop(kernel);
        Ok(into_tokio(ours)?)
    }
}

/// One port of the virtual host controller.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VhciPort {
    pub superspeed: bool,
    pub port: u32,
    pub status: u32,
}

/// Parses the `status*` files of vhci_hcd.
pub fn parse_vhci_status(text: &str) -> Vec<VhciPort> {
    text.lines()
        .filter_map(|line| {
            let mut f = line.split_whitespace();
            let hub = f.next()?;
            let superspeed = match hub {
                "hs" => false,
                "ss" => true,
                _ => return None,
            };
            let port = f.next()?.parse().ok()?;
            let status = f.next()?.parse().ok()?;
            Some(VhciPort { superspeed, port, status })
        })
        .collect()
}

/// Attaches remote devices to `vhci-hcd`.
pub struct LinuxImport {
    sys: PathBuf,
}

impl LinuxImport {
    pub fn new(sys: impl Into<PathBuf>) -> Self {
        LinuxImport { sys: sys.into() }
    }

    fn vhci_dir(&self) -> PathBuf {
        self.sys.join("devices/platform/vhci_hcd.0")
    }

    pub fn ports(&self) -> Result<Vec<VhciPort>> {
        let dir = self.vhci_dir();
        if !dir.exists() {
            bail!("the vhci-hcd kernel module is not loaded (run: modprobe vhci-hcd)");
        }
        let mut ports = Vec::new();
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            if entry.file_name().to_string_lossy().starts_with("status") {
                ports.extend(parse_vhci_status(&fs::read_to_string(entry.path())?));
            }
        }
        ports.sort_by_key(|p| p.port);
        Ok(ports)
    }
}

impl ImportBackend for LinuxImport {
    fn attach(&self, device: &DeviceInfo) -> Result<(u32, tokio::net::TcpStream)> {
        let ss = device.speed.is_superspeed();
        let mut last_err = None;
        // Another process may grab the same free port; retry on the next one.
        for _ in 0..3 {
            let port = self
                .ports()?
                .into_iter()
                .find(|p| p.superspeed == ss && p.status == VHCI_FREE)
                .ok_or_else(|| anyhow!("no free virtual USB port"))?;
            let (ours, kernel) = loopback_pair()?;
            let line = format!("{} {} {} {}", port.port, kernel.as_raw_fd(), device.devid(), device.speed as u32);
            match sysfs_write(&self.vhci_dir().join("attach"), &line) {
                Ok(()) => {
                    drop(kernel);
                    return Ok((port.port, into_tokio(ours)?));
                }
                Err(e) => last_err = Some(e),
            }
        }
        Err(last_err.unwrap_or_else(|| anyhow!("attach failed")))
    }

    fn detach(&self, port: u32) -> Result<()> {
        sysfs_write(&self.vhci_dir().join("detach"), &port.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fake_device(sys: &Path, busid: &str) {
        let d = sys.join("bus/usb/devices").join(busid);
        fs::create_dir_all(&d).unwrap();
        for (k, v) in [
            ("busnum", "1"),
            ("devnum", "4"),
            ("speed", "480"),
            ("idVendor", "0781"),
            ("idProduct", "5567"),
            ("bcdDevice", "0100"),
            ("bDeviceClass", "00"),
            ("bDeviceSubClass", "00"),
            ("bDeviceProtocol", "00"),
            ("bConfigurationValue", "1"),
            ("bNumConfigurations", "1"),
            ("bNumInterfaces", " 1"),
            ("product", "Cruzer Blade"),
            ("manufacturer", "SanDisk"),
            ("usbip_status", "1"),
        ] {
            fs::write(d.join(k), format!("{v}\n")).unwrap();
        }
        let i = d.join(format!("{busid}:1.0"));
        fs::create_dir_all(&i).unwrap();
        fs::write(i.join("bInterfaceClass"), "08\n").unwrap();
        fs::write(i.join("bInterfaceSubClass"), "06\n").unwrap();
        fs::write(i.join("bInterfaceProtocol"), "50\n").unwrap();
    }

    #[test]
    fn reads_devices() {
        let tmp = tempfile::tempdir().unwrap();
        fake_device(tmp.path(), "1-2");
        fs::create_dir_all(tmp.path().join("bus/usb/devices/usb1")).unwrap();
        fs::create_dir_all(tmp.path().join("bus/usb/devices/1-2:1.0")).unwrap();

        let all = list_local(tmp.path()).unwrap();
        assert_eq!(all.len(), 1);
        let d = &all[0];
        assert_eq!(d.info.busid, "1-2");
        assert_eq!(d.info.devid(), (1 << 16) | 4);
        assert_eq!(d.info.speed, Speed::High);
        assert_eq!((d.info.id_vendor, d.info.id_product), (0x0781, 0x5567));
        assert_eq!(d.info.interfaces, vec![InterfaceInfo { class: 8, subclass: 6, protocol: 0x50 }]);
        assert_eq!(d.product.as_deref(), Some("Cruzer Blade"));

        let export = LinuxExport::new(tmp.path(), vec!["1-2".into(), "9-9".into()]);
        assert_eq!(export.list().unwrap().len(), 1, "missing devices are skipped");
    }

    #[test]
    fn vhci_status_parsing() {
        let text = "hub port sta spd dev      sockfd local_busid\n\
                    hs  0000 006 003 00010004 000003 1-2\n\
                    hs  0001 004 000 00000000 000000 0-0\n\
                    ss  0008 004 000 00000000 000000 0-0\n";
        let ports = parse_vhci_status(text);
        assert_eq!(ports.len(), 3);
        assert_eq!(ports[0], VhciPort { superspeed: false, port: 0, status: 6 });
        assert_eq!(ports[2], VhciPort { superspeed: true, port: 8, status: VHCI_FREE });
    }

    #[tokio::test]
    async fn attach_writes_expected_line() {
        let tmp = tempfile::tempdir().unwrap();
        let vhci = tmp.path().join("devices/platform/vhci_hcd.0");
        fs::create_dir_all(&vhci).unwrap();
        fs::write(
            vhci.join("status"),
            "hub port sta spd dev sockfd local_busid\nhs  0000 006 003 0 0 1-1\nhs  0001 004 000 0 0 0-0\n",
        )
        .unwrap();
        fs::write(vhci.join("attach"), "").unwrap();

        let dev = read_device_fixture(tmp.path());
        let (port, _sock) = LinuxImport::new(tmp.path()).attach(&dev).unwrap();
        assert_eq!(port, 1);
        let line = fs::read_to_string(vhci.join("attach")).unwrap();
        let f: Vec<&str> = line.split(' ').collect();
        assert_eq!(f[0], "1");
        assert_eq!(f[2], ((1u32 << 16) | 4).to_string());
        assert_eq!(f[3], "3");
    }

    fn read_device_fixture(sys: &Path) -> DeviceInfo {
        fake_device(sys, "1-2");
        read_device(sys, "1-2").unwrap().info
    }

    #[test]
    fn missing_modules_are_explained() {
        let tmp = tempfile::tempdir().unwrap();
        let err = LinuxImport::new(tmp.path()).ports().unwrap_err().to_string();
        assert!(err.contains("modprobe vhci-hcd"));
        fake_device(tmp.path(), "1-2");
        let err = LinuxExport::new(tmp.path(), vec!["1-2".into()]).bind("1-2").unwrap_err().to_string();
        assert!(err.contains("modprobe usbip-host"));
    }
}
