// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! Sharing Windows USB devices through the VBoxUSB drivers.
//!
//! VirtualBox's USB drivers (GPL-3.0, signed by Oracle and Microsoft) let a
//! userspace program take over a physical device:
//!
//! * `VBoxUSBMon` is a monitor with capture filters: a device that matches a
//!   filter gets the `VBoxUSB` driver instead of its normal one when it is
//!   (re)plugged. Filters belong to the monitor handle that added them.
//! * `VBoxUSB` exposes the captured device; transfers are submitted with
//!   `SUPUSB_IOCTL_SEND_URB`.
//!
//! Sharing a device: remove it from the system, add a filter, cycle its hub
//! port so it re-enumerates under VBoxUSB, then claim it. Releasing reverses
//! this. The URB phase is served by [`crate::usb_server`].
//!
//! The IOCTL codes and structure layouts below follow the driver ABI declared
//! in VirtualBox's `include/VBox/usblib-win.h` and `include/VBox/usbfilter.h`
//! (structures packed to 4 bytes); they are built byte by byte here so every
//! offset is explicit.

use std::ffi::c_void;
use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::{anyhow, bail, Context, Result};
use usbnexus_proto::{DeviceInfo, InterfaceInfo, Speed};
use windows_sys::core::GUID;
use windows_sys::Win32::Devices::DeviceAndDriverInstallation as di;
use windows_sys::Win32::Devices::Usb as usb;
use windows_sys::Win32::Foundation::{
    CloseHandle, GetLastError, ERROR_IO_PENDING, GENERIC_READ, GENERIC_WRITE, HANDLE, INVALID_HANDLE_VALUE,
};
use windows_sys::Win32::Storage::FileSystem::{
    CreateFileW, FILE_FLAG_OVERLAPPED, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
};
use windows_sys::Win32::System::Threading::CreateEventW;
use windows_sys::Win32::System::IO::{DeviceIoControl, GetOverlappedResult, OVERLAPPED};

use crate::api::ApiError;
use crate::backend::{into_tokio, loopback_pair, BoxFuture, DeviceHost, LocalDevice};
use crate::usb_server::{errno, Completion, IsoResult, Transfer, TransferKind, UsbDevice};

// ------------------------------------------------------------------ driver ABI

/// `CTL_CODE(FILE_DEVICE_UNKNOWN, function, METHOD_BUFFERED, FILE_WRITE_ACCESS)`.
const fn ctl(function: u32) -> u32 {
    (0x22 << 16) | (0x2 << 14) | (function << 2)
}

const SUPUSB_IOCTL_SEND_URB: u32 = ctl(0x607);
const SUPUSB_IOCTL_USB_SELECT_INTERFACE: u32 = ctl(0x609);
const SUPUSB_IOCTL_USB_SET_CONFIG: u32 = ctl(0x60a);
const SUPUSB_IOCTL_USB_CLAIM_DEVICE: u32 = ctl(0x60b);
const SUPUSB_IOCTL_USB_RELEASE_DEVICE: u32 = ctl(0x60c);
const SUPUSB_IOCTL_USB_CLEAR_ENDPOINT: u32 = ctl(0x60e);
const SUPUSB_IOCTL_GET_VERSION: u32 = ctl(0x60f);
const SUPUSB_IOCTL_USB_ABORT_ENDPOINT: u32 = ctl(0x610);

const SUPUSBFLT_IOCTL_GET_VERSION: u32 = ctl(0x610);
const SUPUSBFLT_IOCTL_ADD_FILTER: u32 = ctl(0x611);
const SUPUSBFLT_IOCTL_REMOVE_FILTER: u32 = ctl(0x612);

const USBDRV_MAJOR_VERSION: u32 = 5;
const USBMON_MAJOR_VERSION: u32 = 5;
const USBMON_DEVICE_NAME: &str = r"\\.\VBoxUSBMon";

/// Device interface class of VBoxUSB-captured devices.
const GUID_CLASS_VBOXUSB: GUID = GUID::from_u128(0x00873fdf_cafe_80ee_aa5e_00c04fb1720b);

// USBSUP_TRANSFER_TYPE
const TYPE_ISOC: u32 = 1;
const TYPE_BULK: u32 = 2;
const TYPE_INTR: u32 = 3;
const TYPE_MSG: u32 = 4;
// USBSUP_DIRECTION
const DIR_IN: u32 = 1;
const DIR_OUT: u32 = 2;
// USBSUP_XFER_FLAG
const FLAG_SHORT_OK: u32 = 1;

/// Size of `USBSUP_URB` on 64-bit Windows.
const URB_SIZE: usize = 104;
/// Isochronous packets per `USBSUP_URB`.
const URB_ISO_PACKETS: usize = 8;

// USBSUP_URB field offsets (pack 4).
const URB_TYPE: usize = 0;
const URB_EP: usize = 4;
const URB_DIR: usize = 8;
const URB_FLAGS: usize = 12;
const URB_ERROR: usize = 16;
const URB_LEN: usize = 20;
const URB_BUF: usize = 28;
const URB_NUM_ISO: usize = 36;
const URB_ISO: usize = 40;

/// Maps `USBSUP_ERROR` to a negated Linux errno.
fn urb_status(error: u32) -> i32 {
    -match error {
        0 => return 0,
        1 => errno::EPIPE,     // stall
        2 => errno::ETIME,     // device not responding
        3 => errno::EILSEQ,    // CRC
        4 => errno::EPROTO,    // NAK
        5 => errno::EREMOTEIO, // underrun
        6 => errno::EOVERFLOW, // overrun
        _ => errno::EPROTO,
    }
}

fn put_u32(b: &mut [u8], off: usize, v: u32) {
    b[off..off + 4].copy_from_slice(&v.to_le_bytes());
}

fn get_u32(b: &[u8], off: usize) -> u32 {
    u32::from_le_bytes(b[off..off + 4].try_into().unwrap())
}

fn get_u16(b: &[u8], off: usize) -> u16 {
    u16::from_le_bytes(b[off..off + 2].try_into().unwrap())
}

// USBFILTER (usbfilter.h), pack 4: magic, type, 11 fields, string table end, strings.
const USBFILTER_MAGIC: u32 = 0x1967_0408;
const USBFILTERTYPE_CAPTURE: u32 = 4;
const USBFILTERMATCH_IGNORE: u16 = 1;
const USBFILTERMATCH_NUM_EXACT: u16 = 3;
const USBFILTER_FIELDS: usize = 11;
const USBFILTER_SIZE: usize = 4 + 4 + USBFILTER_FIELDS * 4 + 4 + 256;
// USBFILTERIDX
const FLT_VENDOR_ID: usize = 0;
const FLT_PRODUCT_ID: usize = 1;
const FLT_DEVICE_REV: usize = 2;
const FLT_DEVICE_CLASS: usize = 3;
const FLT_DEVICE_SUB_CLASS: usize = 4;
const FLT_DEVICE_PROTOCOL: usize = 5;
const FLT_PORT: usize = 7;

fn capture_filter(d: &DeviceInfo, port: u16) -> Vec<u8> {
    let mut f = vec![0u8; USBFILTER_SIZE];
    put_u32(&mut f, 0, USBFILTER_MAGIC);
    put_u32(&mut f, 4, USBFILTERTYPE_CAPTURE);
    let mut set = |idx: usize, m: u16, v: u16| {
        let off = 8 + idx * 4;
        f[off..off + 2].copy_from_slice(&m.to_le_bytes());
        f[off + 2..off + 4].copy_from_slice(&v.to_le_bytes());
    };
    for i in 0..USBFILTER_FIELDS {
        set(i, USBFILTERMATCH_IGNORE, 0);
    }
    set(FLT_VENDOR_ID, USBFILTERMATCH_NUM_EXACT, d.id_vendor);
    set(FLT_PRODUCT_ID, USBFILTERMATCH_NUM_EXACT, d.id_product);
    set(FLT_DEVICE_REV, USBFILTERMATCH_NUM_EXACT, d.bcd_device);
    set(FLT_DEVICE_CLASS, USBFILTERMATCH_NUM_EXACT, d.device_class as u16);
    set(FLT_DEVICE_SUB_CLASS, USBFILTERMATCH_NUM_EXACT, d.device_subclass as u16);
    set(FLT_DEVICE_PROTOCOL, USBFILTERMATCH_NUM_EXACT, d.device_protocol as u16);
    set(FLT_PORT, USBFILTERMATCH_NUM_EXACT, port);
    f
}

// ------------------------------------------------------------------ handles and I/O

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}

fn from_wide(buf: &[u16]) -> String {
    let end = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    String::from_utf16_lossy(&buf[..end])
}

/// An owned Win32 handle.
struct Handle(HANDLE);

// SAFETY: Win32 file handles may be used from any thread; overlapped I/O on
// one handle from several threads is allowed.
unsafe impl Send for Handle {}
unsafe impl Sync for Handle {}

impl Drop for Handle {
    fn drop(&mut self) {
        // SAFETY: the handle is owned and closed exactly once.
        unsafe { CloseHandle(self.0) };
    }
}

impl Handle {
    fn open(path: &str, overlapped: bool) -> std::io::Result<Handle> {
        let p = wide(path);
        // SAFETY: `p` is NUL-terminated; other arguments are plain values.
        let h = unsafe {
            CreateFileW(
                p.as_ptr(),
                GENERIC_READ | GENERIC_WRITE,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                std::ptr::null(),
                OPEN_EXISTING,
                if overlapped { FILE_FLAG_OVERLAPPED } else { 0 },
                std::ptr::null_mut(),
            )
        };
        if h == INVALID_HANDLE_VALUE {
            return Err(std::io::Error::last_os_error());
        }
        Ok(Handle(h))
    }

    /// Synchronous IOCTL (handle opened without FILE_FLAG_OVERLAPPED).
    fn ioctl(&self, code: u32, input: &[u8], output: &mut [u8]) -> std::io::Result<u32> {
        let mut returned = 0u32;
        // SAFETY: the buffers are valid for the given sizes for the whole call.
        let ok = unsafe {
            DeviceIoControl(
                self.0,
                code,
                input.as_ptr().cast(),
                input.len() as u32,
                output.as_mut_ptr().cast(),
                output.len() as u32,
                &mut returned,
                std::ptr::null_mut(),
            )
        };
        if ok == 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(returned)
    }
}

/// An overlapped IOCTL in progress. The caller keeps the buffers it passed
/// alive and unmoved until [`Pending::wait`] returns.
struct Pending<'h> {
    handle: &'h Handle,
    overlapped: Box<OVERLAPPED>,
    /// Signalled on completion; must stay open until then.
    _event: Handle,
}

impl<'h> Pending<'h> {
    /// Starts an IOCTL on an overlapped handle.
    ///
    /// # Safety
    /// `input` and `output` (and anything they point to, such as a URB data
    /// buffer) must stay valid until `wait` returns.
    unsafe fn start(
        handle: &'h Handle,
        code: u32,
        input: *const u8,
        in_len: usize,
        output: *mut u8,
        out_len: usize,
    ) -> std::io::Result<Self> {
        // SAFETY: plain event creation.
        let ev = unsafe { CreateEventW(std::ptr::null(), 1, 0, std::ptr::null()) };
        if ev.is_null() {
            return Err(std::io::Error::last_os_error());
        }
        let event = Handle(ev);
        // SAFETY: OVERLAPPED is plain data; zero is its initial state.
        let mut overlapped: Box<OVERLAPPED> = Box::new(unsafe { std::mem::zeroed() });
        overlapped.hEvent = event.0;
        let mut returned = 0u32;
        // SAFETY: buffers are valid per the caller's contract; `overlapped`
        // is boxed so its address stays fixed until completion.
        let ok = unsafe {
            DeviceIoControl(
                handle.0,
                code,
                input as *const c_void,
                in_len as u32,
                output as *mut c_void,
                out_len as u32,
                &mut returned,
                &mut *overlapped,
            )
        };
        // SAFETY: reading the thread's last error.
        if ok == 0 && unsafe { GetLastError() } != ERROR_IO_PENDING {
            return Err(std::io::Error::last_os_error());
        }
        Ok(Pending { handle, overlapped, _event: event })
    }

    /// Blocks until the IOCTL completes; returns the bytes written to output.
    fn wait(self) -> std::io::Result<u32> {
        let mut n = 0u32;
        // SAFETY: `overlapped` belongs to an IOCTL started on this handle.
        let ok = unsafe { GetOverlappedResult(self.handle.0, &*self.overlapped, &mut n, 1) };
        if ok == 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(n)
    }
}

/// Overlapped IOCTL with owned buffers, completed on a blocking thread.
async fn ioctl_async(handle: Arc<Handle>, code: u32, input: Vec<u8>, out_len: usize) -> std::io::Result<Vec<u8>> {
    tokio::task::spawn_blocking(move || {
        let mut output = vec![0u8; out_len];
        // SAFETY: `input` and `output` live in this closure until `wait` returns.
        let pending =
            unsafe { Pending::start(&handle, code, input.as_ptr(), input.len(), output.as_mut_ptr(), output.len())? };
        let n = pending.wait()?;
        output.truncate(n as usize);
        Ok(output)
    })
    .await
    .map_err(std::io::Error::other)?
}

// ------------------------------------------------------------------ enumeration

/// A USB device as seen by Windows PnP.
#[derive(Debug, Clone)]
struct PnpDevice {
    instance_id: String,
    devnode: u32,
    hub: u16,
    port: u16,
    service: String,
    description: String,
}

impl PnpDevice {
    fn busid(&self) -> String {
        format!("{}-{}", self.hub, self.port)
    }
}

/// Parses `Port_#0003.Hub_#0001` into `(hub, port)`.
fn parse_location(s: &str) -> Option<(u16, u16)> {
    let (port, hub) = s.split_once('.')?;
    let port = port.strip_prefix("Port_#")?.parse().ok()?;
    let hub = hub.strip_prefix("Hub_#")?.parse().ok()?;
    Some((hub, port))
}

/// Parses a busid (`hub-port`).
fn parse_busid(busid: &str) -> Option<(u16, u16)> {
    let (hub, port) = busid.split_once('-')?;
    Some((hub.parse().ok()?, port.parse().ok()?))
}

struct DevInfoSet(di::HDEVINFO);

impl Drop for DevInfoSet {
    fn drop(&mut self) {
        // SAFETY: the set was returned by SetupDiGetClassDevsW.
        unsafe { di::SetupDiDestroyDeviceInfoList(self.0) };
    }
}

impl DevInfoSet {
    fn interfaces(class: &GUID, enumerator: Option<&str>) -> Result<Self> {
        let e = enumerator.map(wide);
        // SAFETY: `class` is valid; `e` is NUL-terminated when present.
        let h = unsafe {
            di::SetupDiGetClassDevsW(
                class,
                e.as_ref().map_or(std::ptr::null(), |v| v.as_ptr()),
                std::ptr::null_mut(),
                di::DIGCF_PRESENT | di::DIGCF_DEVICEINTERFACE,
            )
        };
        if h == INVALID_HANDLE_VALUE as isize {
            return Err(std::io::Error::last_os_error()).context("SetupDiGetClassDevsW");
        }
        Ok(DevInfoSet(h))
    }

    /// Device interface paths with their device info records.
    fn entries(&self, class: &GUID) -> Vec<(String, di::SP_DEVINFO_DATA)> {
        let mut out = Vec::new();
        for index in 0.. {
            // SAFETY: plain data initialised with its size field.
            let mut iface: di::SP_DEVICE_INTERFACE_DATA = unsafe { std::mem::zeroed() };
            iface.cbSize = std::mem::size_of::<di::SP_DEVICE_INTERFACE_DATA>() as u32;
            // SAFETY: valid set and output structure.
            if unsafe { di::SetupDiEnumDeviceInterfaces(self.0, std::ptr::null(), class, index, &mut iface) } == 0 {
                break;
            }
            let mut needed = 0u32;
            // SAFETY: size query with no output buffer.
            unsafe {
                di::SetupDiGetDeviceInterfaceDetailW(
                    self.0,
                    &iface,
                    std::ptr::null_mut(),
                    0,
                    &mut needed,
                    std::ptr::null_mut(),
                )
            };
            if needed == 0 {
                continue;
            }
            // u64 storage keeps the detail structure aligned.
            let mut buf = vec![0u64; (needed as usize).div_ceil(8)];
            let detail = buf.as_mut_ptr() as *mut di::SP_DEVICE_INTERFACE_DETAIL_DATA_W;
            // SAFETY: `buf` holds `needed` bytes; cbSize is the fixed header size.
            unsafe { (*detail).cbSize = std::mem::size_of::<di::SP_DEVICE_INTERFACE_DETAIL_DATA_W>() as u32 };
            // SAFETY: plain data initialised with its size field.
            let mut info: di::SP_DEVINFO_DATA = unsafe { std::mem::zeroed() };
            info.cbSize = std::mem::size_of::<di::SP_DEVINFO_DATA>() as u32;
            // SAFETY: buffers valid as described above.
            let ok =
                unsafe { di::SetupDiGetDeviceInterfaceDetailW(self.0, &iface, detail, needed, &mut needed, &mut info) };
            if ok == 0 {
                continue;
            }
            // SAFETY: DevicePath is a NUL-terminated string inside `buf`.
            let path_ptr = unsafe { std::ptr::addr_of!((*detail).DevicePath) as *const u16 };
            let chars = (needed as usize - std::mem::size_of::<u32>()) / 2;
            // SAFETY: the path lies within the `needed` bytes written.
            let path = from_wide(unsafe { std::slice::from_raw_parts(path_ptr, chars) });
            out.push((path, info));
        }
        out
    }

    fn property(&self, info: &di::SP_DEVINFO_DATA, prop: u32) -> Option<String> {
        let mut buf = [0u16; 512];
        let mut needed = 0u32;
        // SAFETY: buffer valid for its byte size.
        let ok = unsafe {
            di::SetupDiGetDeviceRegistryPropertyW(
                self.0,
                info,
                prop,
                std::ptr::null_mut(),
                buf.as_mut_ptr().cast(),
                (buf.len() * 2) as u32,
                &mut needed,
            )
        };
        (ok != 0).then(|| from_wide(&buf))
    }

    fn instance_id(&self, info: &di::SP_DEVINFO_DATA) -> Option<String> {
        let mut buf = [0u16; 512];
        // SAFETY: buffer valid for its length in characters.
        let ok = unsafe {
            di::SetupDiGetDeviceInstanceIdW(self.0, info, buf.as_mut_ptr(), buf.len() as u32, std::ptr::null_mut())
        };
        (ok != 0).then(|| from_wide(&buf))
    }
}

fn pnp_devices(class: &GUID) -> Result<Vec<(String, PnpDevice)>> {
    let set = DevInfoSet::interfaces(class, None)?;
    let mut out = Vec::new();
    for (path, info) in set.entries(class) {
        let Some((hub, port)) = set.property(&info, di::SPDRP_LOCATION_INFORMATION).as_deref().and_then(parse_location)
        else {
            continue;
        };
        let Some(instance_id) = set.instance_id(&info) else { continue };
        out.push((
            path,
            PnpDevice {
                instance_id,
                devnode: info.DevInst,
                hub,
                port,
                service: set.property(&info, di::SPDRP_SERVICE).unwrap_or_default(),
                description: set.property(&info, di::SPDRP_DEVICEDESC).unwrap_or_default(),
            },
        ));
    }
    Ok(out)
}

/// Service of usbip-win2's virtual host controller (`ROOT\USBIP_WIN2\UDE`).
const USBIP_WIN2_SERVICE: &str = "usbip2_ude";

/// Whether `devnode` hangs off usbip-win2's virtual host controller, i.e.
/// is a device of another computer attached through USB Nexus. Such devices
/// are not listed: sharing them makes no sense, and asking their hub for
/// descriptors sends requests through this very service, which can wait
/// on itself.
fn is_remote_device(devnode: u32) -> bool {
    let mut node = devnode;
    // Device -> (hubs) -> root hub -> host controller; a few levels suffice.
    for _ in 0..8 {
        let mut parent = 0u32;
        // SAFETY: plain out-parameter.
        if unsafe { di::CM_Get_Parent(&mut parent, node, 0) } != di::CR_SUCCESS {
            return false;
        }
        node = parent;
        let mut buf = [0u16; 128];
        let mut len = (buf.len() * 2) as u32;
        // SAFETY: buffer valid for `len` bytes.
        let rc = unsafe {
            di::CM_Get_DevNode_Registry_PropertyW(
                node,
                di::CM_DRP_SERVICE,
                std::ptr::null_mut(),
                buf.as_mut_ptr().cast(),
                &mut len,
                0,
            )
        };
        if rc == di::CR_SUCCESS && from_wide(&buf).eq_ignore_ascii_case(USBIP_WIN2_SERVICE) {
            return true;
        }
    }
    false
}

/// Device interface path of the hub a device is plugged into.
fn hub_path(devnode: u32) -> Result<String> {
    let mut parent = 0u32;
    // SAFETY: plain out-parameter.
    if unsafe { di::CM_Get_Parent(&mut parent, devnode, 0) } != di::CR_SUCCESS {
        bail!("CM_Get_Parent failed");
    }
    let mut id = [0u16; 512];
    // SAFETY: buffer valid for its length.
    if unsafe { di::CM_Get_Device_IDW(parent, id.as_mut_ptr(), id.len() as u32, 0) } != di::CR_SUCCESS {
        bail!("CM_Get_Device_IDW failed");
    }
    let parent_id = from_wide(&id);
    let set = DevInfoSet::interfaces(&usb::GUID_DEVINTERFACE_USB_HUB, Some(&parent_id))?;
    set.entries(&usb::GUID_DEVINTERFACE_USB_HUB)
        .into_iter()
        .next()
        .map(|(p, _)| p)
        .ok_or_else(|| anyhow!("no hub interface for {parent_id}"))
}

/// Descriptor-level information read from the parent hub.
struct HubInfo {
    descriptor: [u8; 18],
    speed: Speed,
    configuration: u8,
    is_hub: bool,
}

fn hub_connection_info(hub: &Handle, port: u16) -> Result<HubInfo> {
    // USB_NODE_CONNECTION_INFORMATION_EX (packed): index, 18-byte device
    // descriptor, current configuration, speed, is-hub, address, pipes...
    let mut buf = vec![0u8; 512];
    put_u32(&mut buf, 0, port as u32);
    let input = buf.clone();
    hub.ioctl(usb::IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX, &input, &mut buf)
        .context("IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX")?;
    let mut descriptor = [0u8; 18];
    descriptor.copy_from_slice(&buf[4..22]);
    let mut speed = match buf[23] {
        0 => Speed::Low,
        1 => Speed::Full,
        2 => Speed::High,
        3 => Speed::Super,
        _ => Speed::Unknown,
    };

    // The V2 query tells whether the device really runs at SuperSpeed(+).
    let mut v2 = vec![0u8; 16];
    put_u32(&mut v2, 0, port as u32);
    put_u32(&mut v2, 4, 16);
    put_u32(&mut v2, 8, 0b111); // USB 1.1, 2.0 and 3.0 supported by the caller
    let input = v2.clone();
    if hub.ioctl(usb::IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX_V2, &input, &mut v2).is_ok() {
        let flags = get_u32(&v2, 12);
        if flags & 0b100 != 0 {
            speed = Speed::SuperPlus;
        } else if flags & 0b1 != 0 {
            speed = Speed::Super;
        }
    }
    Ok(HubInfo { descriptor, speed, configuration: buf[22], is_hub: buf[24] != 0 })
}

/// Reads a descriptor through the hub (works whichever driver owns the device).
fn hub_descriptor(hub: &Handle, port: u16, value: u16, index: u16, len: u16) -> Result<Vec<u8>> {
    // USB_DESCRIPTOR_REQUEST (packed): index, setup packet, data.
    const HEADER: usize = 4 + 8;
    let mut buf = vec![0u8; HEADER + len as usize];
    put_u32(&mut buf, 0, port as u32);
    buf[4] = 0x80;
    buf[5] = 0x06;
    buf[6..8].copy_from_slice(&value.to_le_bytes());
    buf[8..10].copy_from_slice(&index.to_le_bytes());
    buf[10..12].copy_from_slice(&len.to_le_bytes());
    let input = buf.clone();
    let n = hub.ioctl(usb::IOCTL_USB_GET_DESCRIPTOR_FROM_NODE_CONNECTION, &input, &mut buf)? as usize;
    Ok(buf[HEADER..n.max(HEADER)].to_vec())
}

fn string_descriptor(hub: &Handle, port: u16, index: u8) -> Option<String> {
    if index == 0 {
        return None;
    }
    let d = hub_descriptor(hub, port, (3 << 8) | index as u16, 0x0409, 255).ok()?;
    if d.len() < 2 || d[1] != 3 {
        return None;
    }
    let len = (d[0] as usize).min(d.len());
    let units: Vec<u16> = d[2..len].chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
    let s = String::from_utf16_lossy(&units).trim().to_string();
    (!s.is_empty()).then_some(s)
}

/// Interface classes from the active configuration descriptor.
fn interfaces(hub: &Handle, port: u16) -> Vec<InterfaceInfo> {
    let Ok(cfg) = hub_descriptor(hub, port, 2 << 8, 0, 1024) else { return vec![] };
    let mut out = vec![];
    let mut i = 0;
    while i + 2 <= cfg.len() {
        let len = cfg[i] as usize;
        if len < 2 || i + len > cfg.len() {
            break;
        }
        // Interface descriptor, alternate setting 0.
        if cfg[i + 1] == 4 && len >= 9 && cfg[i + 3] == 0 {
            out.push(InterfaceInfo { class: cfg[i + 5], subclass: cfg[i + 6], protocol: cfg[i + 7] });
        }
        i += len;
    }
    out
}

fn local_device(dev: &PnpDevice) -> Result<Option<LocalDevice>> {
    let hub = Handle::open(&hub_path(dev.devnode)?, false).context("opening hub")?;
    let h = hub_connection_info(&hub, dev.port)?;
    if h.is_hub {
        return Ok(None);
    }
    let d = &h.descriptor;
    let info = DeviceInfo {
        path: dev.instance_id.clone(),
        busid: dev.busid(),
        busnum: dev.hub as u32,
        devnum: dev.port as u32,
        speed: h.speed,
        id_vendor: get_u16(d, 8),
        id_product: get_u16(d, 10),
        bcd_device: get_u16(d, 12),
        device_class: d[4],
        device_subclass: d[5],
        device_protocol: d[6],
        configuration_value: h.configuration,
        num_configurations: d[17],
        interfaces: interfaces(&hub, dev.port),
    };
    Ok(Some(LocalDevice {
        manufacturer: string_descriptor(&hub, dev.port, d[14]),
        product: string_descriptor(&hub, dev.port, d[15]).or_else(|| Some(dev.description.clone())),
        serial: string_descriptor(&hub, dev.port, d[16]),
        driver: Some(dev.service.clone()),
        info,
    }))
}

fn find(busid: &str) -> Result<PnpDevice> {
    pnp_devices(&usb::GUID_DEVINTERFACE_USB_DEVICE)?
        .into_iter()
        .map(|(_, d)| d)
        .find(|d| d.busid() == busid && !is_remote_device(d.devnode))
        .ok_or_else(|| ApiError::new("no_such_device", format!("no USB device {busid}")).into())
}

// ------------------------------------------------------------------ capture

/// Removes a device node from the running system without restarting it.
fn remove_devnode(devnode: u32) -> Result<()> {
    let mut veto = 0;
    let mut name = [0u16; 260];
    // SAFETY: out-parameters valid for their sizes.
    let cr = unsafe {
        di::CM_Query_And_Remove_SubTreeW(
            devnode,
            &mut veto,
            name.as_mut_ptr(),
            name.len() as u32,
            di::CM_REMOVE_NO_RESTART | di::CM_REMOVE_UI_NOT_OK,
        )
    };
    match cr {
        di::CR_SUCCESS => Ok(()),
        di::CR_REMOVE_VETOED => {
            Err(ApiError::new("device_busy", format!("Windows is using the device ({})", from_wide(&name))).into())
        }
        other => bail!("CM_Query_And_Remove_SubTree failed ({other})"),
    }
}

/// Re-plugs a device electrically and lets PnP start it again.
fn replug(devnode: u32, port: u16) {
    // Drivers expect a freshly plugged device; give it a moment first.
    std::thread::sleep(Duration::from_millis(100));
    if let Ok(path) = hub_path(devnode) {
        if let Ok(hub) = Handle::open(&path, false) {
            let mut p = vec![0u8; 8];
            put_u32(&mut p, 0, port as u32);
            let input = p.clone();
            let _ = hub.ioctl(usb::IOCTL_USB_HUB_CYCLE_PORT, &input, &mut p);
        }
    }
    // SAFETY: plain call on a device node handle.
    unsafe { di::CM_Setup_DevNode(devnode, di::CM_SETUP_DEVNODE_READY) };
}

struct Monitor {
    handle: Handle,
}

impl Monitor {
    fn open() -> Result<Monitor> {
        let handle = Handle::open(USBMON_DEVICE_NAME, false).map_err(|e| {
            anyhow::Error::from(ApiError::new("vboxusb_missing", format!("VBoxUSBMon is not available: {e}")))
        })?;
        let mut v = [0u8; 8];
        handle.ioctl(SUPUSBFLT_IOCTL_GET_VERSION, &[], &mut v).context("VBoxUSBMon version")?;
        if get_u32(&v, 0) != USBMON_MAJOR_VERSION {
            bail!("unsupported VBoxUSBMon version {}.{}", get_u32(&v, 0), get_u32(&v, 4));
        }
        Ok(Monitor { handle })
    }

    fn add_filter(&self, filter: &[u8]) -> Result<u64> {
        // USBSUP_FLTADDOUT: u64 id, i32 rc.
        let mut out = [0u8; 12];
        self.handle.ioctl(SUPUSBFLT_IOCTL_ADD_FILTER, filter, &mut out).context("adding capture filter")?;
        let rc = i32::from_le_bytes(out[8..12].try_into().unwrap());
        if rc != 0 {
            bail!("VBoxUSBMon rejected the filter ({rc})");
        }
        Ok(u64::from_le_bytes(out[..8].try_into().unwrap()))
    }

    fn remove_filter(&self, id: u64) {
        let mut rc = [0u8; 4];
        let _ = self.handle.ioctl(SUPUSBFLT_IOCTL_REMOVE_FILTER, &id.to_le_bytes(), &mut rc);
    }
}

/// A device taken over by VBoxUSB for the duration of a sharing session.
struct Captured {
    device: Arc<Handle>,
    /// Keeps the capture filter alive; `None` if the device already had VBoxUSB.
    monitor: Option<(Monitor, u64)>,
    busid: String,
    port: u16,
}

fn open_vbox_device(busid: &str, timeout: Duration) -> Result<(Handle, u32)> {
    let deadline = Instant::now() + timeout;
    loop {
        if let Some((path, dev)) = pnp_devices(&GUID_CLASS_VBOXUSB)?.into_iter().find(|(_, d)| d.busid() == busid) {
            let handle = Handle::open(&path, true).context("opening VBoxUSB device")?;
            return Ok((handle, dev.devnode));
        }
        if Instant::now() >= deadline {
            bail!("{busid} did not come back with the VBoxUSB driver");
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}

fn sync_on_overlapped(handle: &Handle, code: u32, input: &[u8], out_len: usize) -> std::io::Result<Vec<u8>> {
    let mut output = vec![0u8; out_len];
    // SAFETY: buffers live until `wait` returns within this function.
    let pending =
        unsafe { Pending::start(handle, code, input.as_ptr(), input.len(), output.as_mut_ptr(), output.len())? };
    let n = pending.wait()?;
    output.truncate(n as usize);
    Ok(output)
}

/// Takes over `busid` (blocking; may take a few seconds).
fn capture(busid: &str) -> Result<(Captured, LocalDevice)> {
    let dev = find(busid)?;
    let local = local_device(&dev)?.ok_or_else(|| anyhow!("{busid} is a hub"))?;
    let mut monitor = None;
    if !dev.service.eq_ignore_ascii_case("VBoxUSB") {
        let mon = Monitor::open()?;
        remove_devnode(dev.devnode)?;
        let id = match mon.add_filter(&capture_filter(&local.info, dev.port)) {
            Ok(id) => id,
            Err(e) => {
                replug(dev.devnode, dev.port);
                return Err(e);
            }
        };
        replug(dev.devnode, dev.port);
        monitor = Some((mon, id));
    }
    let (handle, _) = match open_vbox_device(busid, Duration::from_secs(10)) {
        Ok(v) => v,
        Err(e) => {
            if let Some((mon, id)) = &monitor {
                mon.remove_filter(*id);
            }
            return Err(e);
        }
    };
    let v = sync_on_overlapped(&handle, SUPUSB_IOCTL_GET_VERSION, &[], 8)?;
    if v.len() < 8 || get_u32(&v, 0) != USBDRV_MAJOR_VERSION {
        bail!("unsupported VBoxUSB driver version");
    }
    // USBSUP_CLAIMDEV: interface number (unused), claimed flag.
    let claim = sync_on_overlapped(&handle, SUPUSB_IOCTL_USB_CLAIM_DEVICE, &[0, 0], 2)?;
    if claim.get(1).copied().unwrap_or(0) == 0 {
        bail!("VBoxUSB could not claim {busid}");
    }
    Ok((Captured { device: Arc::new(handle), monitor, busid: busid.to_string(), port: dev.port }, local))
}

/// Gives the device back to Windows (blocking).
fn release(c: Captured) {
    let _ = sync_on_overlapped(&c.device, SUPUSB_IOCTL_USB_RELEASE_DEVICE, &[0, 0], 2);
    let vbox_devnode = pnp_devices(&GUID_CLASS_VBOXUSB)
        .ok()
        .and_then(|v| v.into_iter().find(|(_, d)| d.busid() == c.busid).map(|(_, d)| d.devnode));
    drop(c.device);
    if let Some(devnode) = vbox_devnode {
        let _ = remove_devnode(devnode);
        if let Some((mon, id)) = &c.monitor {
            mon.remove_filter(*id);
        }
        replug(devnode, c.port);
    } else if let Some((mon, id)) = &c.monitor {
        mon.remove_filter(*id);
    }
}

// ------------------------------------------------------------------ transfers

struct VBoxDevice {
    handle: Arc<Handle>,
}

fn urb_bytes(kind: u32, ep: u8, dir_in: bool, flags: u32, len: usize, buf: *mut u8) -> Vec<u8> {
    let mut u = vec![0u8; URB_SIZE];
    put_u32(&mut u, URB_TYPE, kind);
    put_u32(&mut u, URB_EP, ep as u32);
    put_u32(&mut u, URB_DIR, if dir_in { DIR_IN } else { DIR_OUT });
    put_u32(&mut u, URB_FLAGS, flags);
    u[URB_LEN..URB_LEN + 8].copy_from_slice(&(len as u64).to_le_bytes());
    u[URB_BUF..URB_BUF + 8].copy_from_slice(&(buf as u64).to_le_bytes());
    u
}

fn urb_len(u: &[u8]) -> usize {
    u64::from_le_bytes(u[URB_LEN..URB_LEN + 8].try_into().unwrap()) as usize
}

/// Runs one non-isochronous transfer (blocking).
fn run_transfer(handle: &Handle, t: Transfer) -> Completion {
    let control = t.kind == TransferKind::Control;
    // Control transfers carry the setup packet in front of the data.
    let mut buf = if control {
        let mut b = t.setup.to_vec();
        b.extend_from_slice(&t.buffer);
        b
    } else {
        t.buffer
    };
    let kind = match t.kind {
        TransferKind::Control => TYPE_MSG,
        TransferKind::Bulk => TYPE_BULK,
        TransferKind::Interrupt => TYPE_INTR,
        TransferKind::Isochronous => TYPE_ISOC,
    };
    let flags = if t.short_ok { FLAG_SHORT_OK } else { 0 };
    let ptr = if buf.is_empty() { std::ptr::null_mut() } else { buf.as_mut_ptr() };
    let mut urb = urb_bytes(kind, t.endpoint, t.dir_in, flags, buf.len(), ptr);
    let urb_ptr = urb.as_mut_ptr();
    // SAFETY: `urb` and `buf` (pointed to by the URB) outlive `wait`.
    let result = unsafe { Pending::start(handle, SUPUSB_IOCTL_SEND_URB, urb_ptr, URB_SIZE, urb_ptr, URB_SIZE) }
        .and_then(|p| p.wait());
    if result.is_err() {
        // Typically the endpoint was aborted (unlink or disconnect).
        return Completion::error(-errno::ECONNRESET);
    }
    let mut len = urb_len(&urb).min(buf.len());
    if control {
        len = len.saturating_sub(8);
        buf.drain(..8);
    }
    Completion { status: urb_status(get_u32(&urb, URB_ERROR)), buffer: buf, actual_length: len as u32, iso: vec![] }
}

/// Runs an isochronous transfer, split into URBs of at most 8 packets whose
/// offsets fit in 16 bits; all URBs are queued before waiting (blocking).
fn run_iso(handle: &Handle, t: Transfer) -> Completion {
    let mut buf = t.buffer;
    let lengths = t.iso_lengths;
    let mut chunks: Vec<(usize, usize, usize)> = vec![]; // (first packet, packet count, buffer offset)
    let mut i = 0;
    let mut offset = 0usize;
    while i < lengths.len() {
        let (first, start) = (i, offset);
        let mut rel = 0usize;
        while i < lengths.len() && i - first < URB_ISO_PACKETS && rel + lengths[i] as usize <= u16::MAX as usize {
            rel += lengths[i] as usize;
            i += 1;
        }
        if i == first {
            return Completion::error(-errno::EINVAL); // a single packet larger than 64 KiB
        }
        chunks.push((first, i - first, start));
        offset += rel;
    }

    let mut urbs: Vec<Vec<u8>> = Vec::with_capacity(chunks.len());
    for &(first, count, start) in &chunks {
        let len: usize = lengths[first..first + count].iter().map(|&l| l as usize).sum();
        // SAFETY: `start` lies within `buf` (lengths sum to its size).
        let ptr = unsafe { buf.as_mut_ptr().add(start) };
        let mut u = urb_bytes(TYPE_ISOC, t.endpoint, t.dir_in, 0, len, ptr);
        put_u32(&mut u, URB_NUM_ISO, count as u32);
        let mut rel = 0u16;
        for (k, &l) in lengths[first..first + count].iter().enumerate() {
            let p = URB_ISO + k * 8;
            u[p..p + 2].copy_from_slice(&(l as u16).to_le_bytes());
            u[p + 2..p + 4].copy_from_slice(&rel.to_le_bytes());
            rel += l as u16;
        }
        urbs.push(u);
    }

    let mut results = vec![IsoResult { actual_length: 0, status: -errno::ECONNRESET }; lengths.len()];
    let mut pending = Vec::with_capacity(urbs.len());
    for u in urbs.iter_mut() {
        let p = u.as_mut_ptr();
        // SAFETY: `urbs` and `buf` stay alive and unmoved until every wait below.
        pending.push(unsafe { Pending::start(handle, SUPUSB_IOCTL_SEND_URB, p, URB_SIZE, p, URB_SIZE) });
    }
    for (n, p) in pending.into_iter().enumerate() {
        let ok = p.and_then(|p| p.wait()).is_ok();
        let (first, count, _) = chunks[n];
        if !ok {
            continue;
        }
        for k in 0..count {
            let q = URB_ISO + k * 8;
            results[first + k] =
                IsoResult { actual_length: get_u16(&urbs[n], q) as u32, status: urb_status(get_u32(&urbs[n], q + 4)) };
        }
    }
    Completion { status: 0, buffer: buf, actual_length: 0, iso: results }
}

impl UsbDevice for VBoxDevice {
    fn submit(&self, t: Transfer) -> BoxFuture<'static, Completion> {
        let handle = self.handle.clone();
        Box::pin(async move {
            tokio::task::spawn_blocking(move || {
                if t.kind == TransferKind::Isochronous {
                    run_iso(&handle, t)
                } else {
                    run_transfer(&handle, t)
                }
            })
            .await
            .unwrap_or_else(|_| Completion::error(-errno::ESHUTDOWN))
        })
    }

    fn set_configuration(&self, value: u8) -> BoxFuture<'_, Result<()>> {
        Box::pin(async move {
            ioctl_async(self.handle.clone(), SUPUSB_IOCTL_USB_SET_CONFIG, vec![value], 0).await?;
            Ok(())
        })
    }

    fn set_interface(&self, interface: u8, alternate: u8) -> BoxFuture<'_, Result<()>> {
        Box::pin(async move {
            ioctl_async(self.handle.clone(), SUPUSB_IOCTL_USB_SELECT_INTERFACE, vec![interface, alternate], 0).await?;
            Ok(())
        })
    }

    fn clear_halt(&self, endpoint: u8) -> BoxFuture<'_, Result<()>> {
        Box::pin(async move {
            ioctl_async(self.handle.clone(), SUPUSB_IOCTL_USB_CLEAR_ENDPOINT, vec![endpoint], 0).await?;
            Ok(())
        })
    }

    fn abort_endpoint(&self, endpoint: u8) -> BoxFuture<'_, Result<()>> {
        Box::pin(async move {
            ioctl_async(self.handle.clone(), SUPUSB_IOCTL_USB_ABORT_ENDPOINT, vec![endpoint], 0).await?;
            Ok(())
        })
    }
}

// ------------------------------------------------------------------ host

/// Shares local USB devices through VBoxUSB.
#[derive(Default)]
pub struct WindowsHost;

impl DeviceHost for WindowsHost {
    fn list_all(&self) -> Result<Vec<LocalDevice>> {
        let mut seen = std::collections::BTreeSet::new();
        let mut out = vec![];
        for (_, dev) in pnp_devices(&usb::GUID_DEVINTERFACE_USB_DEVICE)? {
            if is_remote_device(dev.devnode) || !seen.insert(dev.busid()) {
                continue;
            }
            match local_device(&dev) {
                Ok(Some(d)) => out.push(d),
                Ok(None) => {}
                Err(e) => tracing::debug!(busid = dev.busid(), "skipping device: {e:#}"),
            }
        }
        out.sort_by_key(|d| parse_busid(&d.info.busid));
        Ok(out)
    }

    fn export<'a>(&'a self, busid: &'a str) -> BoxFuture<'a, Result<tokio::net::TcpStream>> {
        Box::pin(async move {
            let id = busid.to_string();
            let (captured, _) = tokio::task::spawn_blocking(move || capture(&id)).await??;
            // The port cycle during capture already gave the device a clean start.
            let (ours, theirs) = loopback_pair()?;
            let ours = into_tokio(ours)?;
            let device: Arc<dyn UsbDevice> = Arc::new(VBoxDevice { handle: captured.device.clone() });
            let busid = busid.to_string();
            tokio::spawn(async move {
                if let Err(e) = crate::usb_server::serve(ours, device).await {
                    tracing::warn!(busid, "sharing session ended: {e:#}");
                }
                let _ = tokio::task::spawn_blocking(move || release(captured)).await;
            });
            Ok(into_tokio(theirs)?)
        })
    }

    fn release(&self, _busid: &str) -> Result<()> {
        // Each sharing session gives its device back when it ends.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_constants() {
        assert_eq!(SUPUSB_IOCTL_SEND_URB, 0x0022_981c);
        assert_eq!(SUPUSBFLT_IOCTL_ADD_FILTER, 0x0022_9844);
        assert_eq!(USBFILTER_SIZE, 312);
        assert_eq!(URB_ISO + URB_ISO_PACKETS * 8, URB_SIZE);
    }

    #[test]
    fn locations_and_busids() {
        assert_eq!(parse_location("Port_#0003.Hub_#0001"), Some((1, 3)));
        assert_eq!(parse_location("0000.0014.0000.001.003"), None);
        assert_eq!(parse_busid("1-3"), Some((1, 3)));
    }
}
