// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! Sharing USB devices through libusb (used on macOS).
//!
//! The device is opened from userspace and the URB phase is served by
//! [`crate::usb_server`]. Transfers use libusb's synchronous API on blocking
//! threads, in short time slices so that endpoint aborts take effect quickly;
//! the raw calls are used because they report the bytes moved even when a
//! slice times out, so no data is lost between slices.
//!
//! Limitations:
//! * Isochronous transfers are not supported yet.
//! * On macOS a device driven by a system driver (mass storage, keyboards,
//!   ...) can only be taken over by a process with Apple's
//!   `com.apple.vm.device-access` entitlement; without it such devices report
//!   `device_in_use_by_os`. Devices without a macOS driver work.

use std::collections::HashMap;
use std::ffi::c_int;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::{anyhow, Context as _, Result};
use rusb::{ffi, Context, Device, DeviceHandle, UsbContext};
use usbnexus_proto::{DeviceInfo, InterfaceInfo, Speed};

use crate::api::ApiError;
use crate::backend::{into_tokio, loopback_pair, BoxFuture, DeviceHost, LocalDevice};
use crate::usb_server::{errno, Completion, Transfer, TransferKind, UsbDevice};

/// Length of one blocking slice of a bulk/interrupt transfer.
const SLICE: Duration = Duration::from_millis(250);
const CONTROL_TIMEOUT: Duration = Duration::from_secs(5);

const LIBUSB_ERROR_ACCESS: c_int = -3;
const LIBUSB_ERROR_NO_DEVICE: c_int = -4;
const LIBUSB_ERROR_BUSY: c_int = -6;
const LIBUSB_ERROR_TIMEOUT: c_int = -7;
const LIBUSB_ERROR_OVERFLOW: c_int = -8;
const LIBUSB_ERROR_PIPE: c_int = -9;

/// Maps a libusb error code to a negated Linux errno.
fn status(rc: c_int) -> i32 {
    -match rc {
        LIBUSB_ERROR_PIPE => errno::EPIPE,
        LIBUSB_ERROR_TIMEOUT => errno::ETIME,
        LIBUSB_ERROR_OVERFLOW => errno::EOVERFLOW,
        LIBUSB_ERROR_NO_DEVICE => errno::ESHUTDOWN,
        _ => errno::EPROTO,
    }
}

fn speed(s: rusb::Speed) -> Speed {
    match s {
        rusb::Speed::Low => Speed::Low,
        rusb::Speed::Full => Speed::Full,
        rusb::Speed::High => Speed::High,
        rusb::Speed::Super => Speed::Super,
        rusb::Speed::SuperPlus => Speed::SuperPlus,
        _ => Speed::Unknown,
    }
}

fn bcd(v: rusb::Version) -> u16 {
    let (major, minor, sub) = (v.major() as u16, v.minor() as u16, v.sub_minor() as u16);
    ((major / 10) << 12) | ((major % 10) << 8) | (minor << 4) | sub
}

/// Bus id in the Linux style: `<bus>-<port>[.<port>...]`.
fn busid(d: &Device<Context>) -> Option<String> {
    let ports = d.port_numbers().ok()?;
    if ports.is_empty() {
        return None; // root hub
    }
    let path: Vec<String> = ports.iter().map(u8::to_string).collect();
    Some(format!("{}-{}", d.bus_number(), path.join(".")))
}

fn describe(d: &Device<Context>) -> Option<LocalDevice> {
    let busid = busid(d)?;
    let desc = d.device_descriptor().ok()?;
    if desc.class_code() == 9 {
        return None; // hub
    }
    let config = d.active_config_descriptor().ok();
    let interfaces = config
        .as_ref()
        .map(|c| {
            c.interfaces()
                .filter_map(|i| i.descriptors().next())
                .map(|a| InterfaceInfo {
                    class: a.class_code(),
                    subclass: a.sub_class_code(),
                    protocol: a.protocol_code(),
                })
                .collect()
        })
        .unwrap_or_default();
    // Strings need an open handle; devices owned by the OS may refuse.
    let (manufacturer, product) = match d.open() {
        Ok(h) => (h.read_manufacturer_string_ascii(&desc).ok(), h.read_product_string_ascii(&desc).ok()),
        Err(_) => (None, None),
    };
    Some(LocalDevice {
        info: DeviceInfo {
            path: format!("libusb:{busid}"),
            busid,
            busnum: d.bus_number() as u32,
            devnum: d.address() as u32,
            speed: speed(d.speed()),
            id_vendor: desc.vendor_id(),
            id_product: desc.product_id(),
            bcd_device: bcd(desc.device_version()),
            device_class: desc.class_code(),
            device_subclass: desc.sub_class_code(),
            device_protocol: desc.protocol_code(),
            configuration_value: config.as_ref().map(|c| c.number()).unwrap_or(0),
            num_configurations: desc.num_configurations(),
            interfaces,
        },
        product,
        manufacturer,
        driver: None,
    })
}

/// An opened device with its claimed interfaces.
struct Opened {
    handle: DeviceHandle<Context>,
    claimed: Mutex<Vec<u8>>,
    /// Bumped by `abort_endpoint`; transfers check it between slices.
    generations: Mutex<HashMap<u8, Arc<AtomicU64>>>,
}

impl Opened {
    fn raw(&self) -> *mut ffi::libusb_device_handle {
        self.handle.as_raw()
    }

    fn generation(&self, endpoint: u8) -> Arc<AtomicU64> {
        self.generations.lock().unwrap().entry(endpoint).or_default().clone()
    }

    /// Claims every interface of the active configuration.
    fn claim_all(&self) -> Result<()> {
        let config = self.handle.device().active_config_descriptor().context("reading configuration")?;
        let mut claimed = self.claimed.lock().unwrap();
        for iface in config.interfaces() {
            let n = iface.number();
            // SAFETY: valid open handle.
            let rc = unsafe { ffi::libusb_claim_interface(self.raw(), n as c_int) };
            match rc {
                0 => claimed.push(n),
                LIBUSB_ERROR_BUSY | LIBUSB_ERROR_ACCESS => {
                    return Err(ApiError::new(
                        "device_in_use_by_os",
                        format!("interface {n} is held by an operating system driver"),
                    )
                    .into())
                }
                other => return Err(anyhow!("claiming interface {n} failed ({other})")),
            }
        }
        Ok(())
    }

    fn release_all(&self) {
        for n in self.claimed.lock().unwrap().drain(..) {
            // SAFETY: valid open handle; interface was claimed by us.
            unsafe { ffi::libusb_release_interface(self.raw(), n as c_int) };
        }
    }
}

fn open(busid: &str) -> Result<Opened> {
    let ctx = Context::new().context("initialising libusb")?;
    let device = ctx
        .devices()?
        .iter()
        .find(|d| self::busid(d).as_deref() == Some(busid))
        .ok_or_else(|| anyhow::Error::from(ApiError::new("no_such_device", format!("no USB device {busid}"))))?;
    let handle = device.open().map_err(|e| match e {
        rusb::Error::Access | rusb::Error::Busy => {
            anyhow::Error::from(ApiError::new("device_in_use_by_os", format!("cannot open {busid}: {e}")))
        }
        other => anyhow!("opening {busid}: {other}"),
    })?;
    // Where supported, let libusb move system drivers out of the way.
    // SAFETY: valid open handle.
    unsafe { ffi::libusb_set_auto_detach_kernel_driver(handle.as_raw(), 1) };
    let opened = Opened { handle, claimed: Mutex::default(), generations: Mutex::default() };
    opened.claim_all()?;
    Ok(opened)
}

/// Bulk or interrupt transfer in time slices (blocking).
fn run_stream(dev: &Opened, t: Transfer) -> Completion {
    let address = t.address();
    let generation = dev.generation(address);
    let started = generation.load(Ordering::SeqCst);
    let mut buf = t.buffer;
    let mut done = 0usize;
    loop {
        let mut moved: c_int = 0;
        let remaining = buf.len() - done;
        // SAFETY: `buf[done..]` is valid for `remaining` bytes during the call.
        let rc = unsafe {
            let data = buf.as_mut_ptr().add(done);
            let f = if t.kind == TransferKind::Interrupt {
                ffi::libusb_interrupt_transfer
            } else {
                ffi::libusb_bulk_transfer
            };
            f(dev.raw(), address, data, remaining as c_int, &mut moved, SLICE.as_millis() as u32)
        };
        done += moved.max(0) as usize;
        match rc {
            0 => break,
            LIBUSB_ERROR_TIMEOUT => {
                // IN: any data ends the transfer (USB semantics: a short
                // packet); OUT: keep sending the rest.
                if t.dir_in && done > 0 {
                    break;
                }
                if !t.dir_in && done == buf.len() {
                    break;
                }
                if generation.load(Ordering::SeqCst) != started {
                    return Completion {
                        status: -errno::ECONNRESET,
                        buffer: buf,
                        actual_length: done as u32,
                        iso: vec![],
                    };
                }
            }
            other => return Completion { status: status(other), buffer: buf, actual_length: done as u32, iso: vec![] },
        }
    }
    Completion { status: 0, buffer: buf, actual_length: done as u32, iso: vec![] }
}

/// Control transfer on endpoint 0 (blocking).
fn run_control(dev: &Opened, t: Transfer) -> Completion {
    let s = t.setup;
    let (request_type, request) = (s[0], s[1]);
    let value = u16::from_le_bytes([s[2], s[3]]);
    let index = u16::from_le_bytes([s[4], s[5]]);
    let mut buf = t.buffer;
    let len = buf.len().min(u16::MAX as usize) as u16;
    // SAFETY: `buf` is valid for `len` bytes during the call.
    let rc = unsafe {
        ffi::libusb_control_transfer(
            dev.raw(),
            request_type,
            request,
            value,
            index,
            buf.as_mut_ptr(),
            len,
            CONTROL_TIMEOUT.as_millis() as u32,
        )
    };
    if rc < 0 {
        return Completion { status: status(rc), buffer: buf, actual_length: 0, iso: vec![] };
    }
    Completion { status: 0, buffer: buf, actual_length: rc as u32, iso: vec![] }
}

struct LibusbDevice {
    dev: Arc<Opened>,
}

impl UsbDevice for LibusbDevice {
    fn submit(&self, t: Transfer) -> BoxFuture<'static, Completion> {
        let dev = self.dev.clone();
        Box::pin(async move {
            tokio::task::spawn_blocking(move || match t.kind {
                TransferKind::Control => run_control(&dev, t),
                TransferKind::Bulk | TransferKind::Interrupt => run_stream(&dev, t),
                TransferKind::Isochronous => Completion::error(-errno::EINVAL),
            })
            .await
            .unwrap_or_else(|_| Completion::error(-errno::ESHUTDOWN))
        })
    }

    fn set_configuration(&self, value: u8) -> BoxFuture<'_, Result<()>> {
        let dev = self.dev.clone();
        Box::pin(async move {
            tokio::task::spawn_blocking(move || {
                dev.release_all();
                // SAFETY: valid open handle.
                let rc = unsafe { ffi::libusb_set_configuration(dev.raw(), value as c_int) };
                if rc != 0 {
                    return Err(anyhow!("set configuration {value} failed ({rc})"));
                }
                dev.claim_all()
            })
            .await?
        })
    }

    fn set_interface(&self, interface: u8, alternate: u8) -> BoxFuture<'_, Result<()>> {
        let dev = self.dev.clone();
        Box::pin(async move {
            tokio::task::spawn_blocking(move || {
                // SAFETY: valid open handle.
                let rc =
                    unsafe { ffi::libusb_set_interface_alt_setting(dev.raw(), interface as c_int, alternate as c_int) };
                if rc != 0 {
                    return Err(anyhow!("set interface {interface}/{alternate} failed ({rc})"));
                }
                Ok(())
            })
            .await?
        })
    }

    fn clear_halt(&self, endpoint: u8) -> BoxFuture<'_, Result<()>> {
        let dev = self.dev.clone();
        Box::pin(async move {
            tokio::task::spawn_blocking(move || {
                // SAFETY: valid open handle.
                let rc = unsafe { ffi::libusb_clear_halt(dev.raw(), endpoint) };
                if rc != 0 {
                    return Err(anyhow!("clear halt {endpoint:#04x} failed ({rc})"));
                }
                Ok(())
            })
            .await?
        })
    }

    fn abort_endpoint(&self, endpoint: u8) -> BoxFuture<'_, Result<()>> {
        self.dev.generation(endpoint).fetch_add(1, Ordering::SeqCst);
        Box::pin(async { Ok(()) })
    }
}

/// Shares local USB devices through libusb.
#[derive(Default)]
pub struct LibusbHost;

impl DeviceHost for LibusbHost {
    fn list_all(&self) -> Result<Vec<LocalDevice>> {
        let ctx = Context::new().context("initialising libusb")?;
        let mut out: Vec<LocalDevice> = ctx.devices()?.iter().filter_map(|d| describe(&d)).collect();
        out.sort_by(|a, b| a.info.busid.cmp(&b.info.busid));
        Ok(out)
    }

    fn export<'a>(&'a self, busid: &'a str) -> BoxFuture<'a, Result<tokio::net::TcpStream>> {
        Box::pin(async move {
            let id = busid.to_string();
            let opened = Arc::new(tokio::task::spawn_blocking(move || open(&id)).await??);
            let (ours, theirs) = loopback_pair()?;
            let ours = into_tokio(ours)?;
            let device: Arc<dyn UsbDevice> = Arc::new(LibusbDevice { dev: opened.clone() });
            let busid = busid.to_string();
            tokio::spawn(async move {
                if let Err(e) = crate::usb_server::serve(ours, device).await {
                    tracing::warn!(busid, "sharing session ended: {e:#}");
                }
                // Closing the handle re-attaches system drivers detached by libusb.
                let _ = tokio::task::spawn_blocking(move || opened.release_all()).await;
            });
            Ok(into_tokio(theirs)?)
        })
    }

    fn release(&self, _busid: &str) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bcd_encoding() {
        assert_eq!(bcd(rusb::Version(2, 1, 0)), 0x0210);
        assert_eq!(bcd(rusb::Version(1, 0, 0)), 0x0100);
        assert_eq!(bcd(rusb::Version(12, 3, 4)), 0x1234);
    }

    #[test]
    fn error_mapping() {
        assert_eq!(status(LIBUSB_ERROR_PIPE), -errno::EPIPE);
        assert_eq!(status(-99), -errno::EPROTO);
    }

    #[test]
    fn listing_does_not_fail_without_devices() {
        // CI machines and containers usually have no USB devices; libusb may
        // also be unable to enumerate at all there, which must not panic.
        let _ = LibusbHost.list_all();
    }
}
