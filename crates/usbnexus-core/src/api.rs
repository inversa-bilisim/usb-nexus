// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! Local API between the privileged service and user interfaces (GUI, CLI).
//!
//! Transport: newline-delimited JSON over a local socket. Each request line
//! gets exactly one response line. Errors carry a stable `code` that user
//! interfaces translate (message id `err-<code>` with `_` replaced by `-`).

use serde::{Deserialize, Serialize};

use crate::access::{DeviceAccess, DeviceMode, Policy};
use crate::client::ClientError;
use crate::control::RemoteError;
use crate::trust::Peer;
use crate::usage::UsageEntry;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "cmd", rename_all = "snake_case")]
pub enum Request {
    Status,
    /// USB devices of this computer, with their sharing state.
    LocalDevices,
    /// `device` is a device identity (`id` of a [`LocalDeviceView`]) or a
    /// bus id of a connected device.
    SetShared {
        #[serde(alias = "busid")]
        device: String,
        shared: bool,
    },
    /// Chooses the server-wide access policy.
    SetPolicy {
        policy: Policy,
    },
    /// Changes who may use a shared device; `allowed` (fingerprints) is
    /// left unchanged when omitted.
    SetDeviceAccess {
        device: String,
        mode: DeviceMode,
        #[serde(default)]
        allowed: Option<Vec<String>>,
    },
    /// Lets a paired computer use exactly `devices` among the shared devices
    /// whose access is limited to selected computers.
    SetClientAccess {
        fingerprint: String,
        devices: Vec<String>,
    },
    /// Usage log, newest entries first.
    Usage {
        #[serde(default)]
        limit: Option<usize>,
    },
    SetUsageRetention {
        days: u32,
    },
    OpenPairing {
        seconds: u64,
    },
    ClosePairing,
    Discover {
        seconds: u64,
    },
    Pair {
        address: String,
        pin: String,
    },
    /// Devices shared by a paired server (by fingerprint).
    RemoteDevices {
        server: String,
    },
    /// `device` is a device identity (`id` of a [`RemoteDeviceView`]).
    Attach {
        server: String,
        #[serde(alias = "busid")]
        device: String,
    },
    Detach {
        server: String,
        #[serde(alias = "busid")]
        device: String,
    },
    Attachments,
    Peers,
    Forget {
        fingerprint: String,
    },
    WebStatus,
    /// Changes what this computer is set up for, installing what the new
    /// roles need (drivers) first. At least one role must remain.
    SetRoles {
        server: bool,
        client: bool,
    },
    /// Changes the web interface settings; fields left out are unchanged.
    /// Refused when it arrives through the web interface itself.
    WebConfigure {
        #[serde(default)]
        enabled: Option<bool>,
        #[serde(default)]
        lan: Option<bool>,
        #[serde(default)]
        port: Option<u16>,
        #[serde(default)]
        password: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Response {
    Ok { data: serde_json::Value },
    Error { error: ApiError },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, thiserror::Error)]
#[error("{code}: {message}")]
pub struct ApiError {
    /// Stable machine-readable code, e.g. `pairing_required`, `device_busy`.
    pub code: String,
    /// English technical detail, for logs and the "details" view.
    pub message: String,
}

impl ApiError {
    pub fn new(code: &str, message: impl Into<String>) -> Self {
        ApiError { code: code.to_string(), message: message.into() }
    }

    /// Classifies an internal error into an API error code.
    pub fn from_anyhow(e: &anyhow::Error) -> Self {
        let message = format!("{e:#}");
        if let Some(c) = e.downcast_ref::<ClientError>() {
            let code = match c {
                ClientError::PairingRequired { .. } => "pairing_required",
                ClientError::NotFound(_) => "not_found",
            };
            return ApiError::new(code, message);
        }
        if let Some(ApiError { code, .. }) = e.downcast_ref::<ApiError>() {
            return ApiError::new(code, message);
        }
        if let Some(r) = e.downcast_ref::<RemoteError>() {
            let code = serde_json::to_value(r.code).ok().and_then(|v| v.as_str().map(str::to_string));
            return ApiError::new(code.as_deref().unwrap_or("protocol"), message);
        }
        for cause in e.chain() {
            if let Some(io) = cause.downcast_ref::<std::io::Error>() {
                use std::io::ErrorKind::*;
                match io.kind() {
                    PermissionDenied => return ApiError::new("permission_denied", message),
                    ConnectionRefused | ConnectionReset | TimedOut | HostUnreachable | NetworkUnreachable => {
                        return ApiError::new("unreachable", message)
                    }
                    _ => {}
                }
            }
            if cause.to_string().contains("timed out") {
                return ApiError::new("unreachable", message);
            }
        }
        ApiError::new("other", message)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PairingView {
    pub pin: String,
    pub remaining_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StatusView {
    pub name: String,
    /// What this computer is set up for; interfaces hide the other screens.
    #[serde(default)]
    pub roles: Roles,
    /// Something installed for a role needs a restart of the computer.
    #[serde(default)]
    pub reboot_required: bool,
    pub fingerprint: String,
    pub version: String,
    pub listen: String,
    pub pairing: Option<PairingView>,
    /// Server-wide access policy.
    pub policy: Policy,
    /// Whether the policy was chosen explicitly (user interfaces ask on
    /// first run otherwise).
    pub policy_chosen: bool,
}

/// What a computer is set up for. Both by default (and for configurations
/// from before roles existed).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Roles {
    /// Shares its own USB devices.
    pub server: bool,
    /// Uses USB devices of other computers.
    pub client: bool,
}

impl Default for Roles {
    fn default() -> Self {
        Roles { server: true, client: true }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LocalDeviceView {
    /// Device identity; use it to refer to the device in requests.
    pub id: String,
    /// Current bus id (none while unplugged).
    pub busid: Option<String>,
    pub vendor_id: u16,
    pub product_id: u16,
    pub product: Option<String>,
    pub manufacturer: Option<String>,
    /// Empty while unplugged.
    pub speed: String,
    /// Whether the device is plugged in (shared devices stay listed).
    pub present: bool,
    /// The device has no serial number, so it is recognised by its port.
    pub by_port: bool,
    pub shared: bool,
    /// Access settings of a shared device.
    pub access: Option<DeviceAccess>,
    /// Whether every paired computer may use the (shared) device.
    pub open_to_all: bool,
    /// Name of the client currently using the device.
    pub used_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiscoveredView {
    pub name: String,
    pub fingerprint: String,
    pub address: Option<String>,
    pub paired: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RemoteDeviceView {
    /// Device identity; use it to attach.
    pub id: String,
    /// Current bus id on the server (none while unplugged).
    pub busid: Option<String>,
    pub vendor_id: u16,
    pub product_id: u16,
    pub product: Option<String>,
    pub manufacturer: Option<String>,
    pub speed: String,
    /// Whether the device is plugged in on the server.
    pub present: bool,
    /// Whether this computer may use it.
    pub allowed: bool,
    pub by_port: bool,
    /// Used by some computer (possibly this one).
    pub in_use: bool,
    /// Attached (or being attached) on this computer.
    pub attached_here: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum AttachState {
    Connecting,
    Attached {
        port: u32,
    },
    /// Waiting `seconds` before the next attempt; `error` says what failed.
    Retrying {
        seconds: u64,
        error: ApiError,
    },
    /// The device is not plugged in on the server (`error.code` is
    /// `no_such_device`) or this computer may not use it (`access_denied`);
    /// asking again in `seconds`.
    Waiting {
        seconds: u64,
        error: ApiError,
    },
    /// Another computer uses the device; this one is `position` in the
    /// queue (1 = next).
    Queued {
        position: u32,
    },
    /// Detached on this computer (device removed or detached by the OS).
    Stopped,
    Failed {
        error: ApiError,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AttachmentView {
    pub server: String,
    pub server_name: String,
    /// Device identity on the server (a bus id for attachments saved by
    /// older versions, until the device is seen).
    pub device: String,
    /// Bus id on the server when last attached.
    pub busid: Option<String>,
    /// Device name, once the server's device list was seen.
    pub name: Option<String>,
    pub vendor_id: Option<u16>,
    pub product_id: Option<u16>,
    #[serde(flatten)]
    pub state: AttachState,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WebStatusView {
    pub enabled: bool,
    pub lan: bool,
    pub port: u16,
    pub password_set: bool,
    /// Where the interface can be opened (empty when off).
    pub urls: Vec<String>,
    /// SHA-256 fingerprint of the HTTPS certificate, to check browser warnings.
    pub fingerprint: Option<String>,
    /// Why the interface is not running although enabled.
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UsageView {
    /// Newest first.
    pub entries: Vec<UsageEntry>,
    pub retention_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PeersView {
    /// Servers this computer can use.
    pub servers: Vec<Peer>,
    /// Clients allowed to use this computer's devices.
    pub clients: Vec<Peer>,
}

/// Address of the system service: `$USBNEXUS_SOCKET`, or the platform
/// default (a Unix socket path, or a named pipe on Windows).
pub fn default_socket() -> std::path::PathBuf {
    if let Some(p) = std::env::var_os("USBNEXUS_SOCKET") {
        return p.into();
    }
    if cfg!(windows) {
        std::path::PathBuf::from(r"\\.\pipe\usbnexus")
    } else if cfg!(target_os = "macos") {
        std::path::PathBuf::from("/var/run/usbnexus/daemon.sock")
    } else {
        std::path::PathBuf::from("/run/usbnexus/daemon.sock")
    }
}

/// Largest accepted request line.
const MAX_LINE: usize = 64 * 1024;

/// Answers requests on one connection until the client disconnects.
async fn serve_connection<R, W>(r: R, mut w: W, daemon: crate::daemon::Daemon)
where
    R: tokio::io::AsyncRead + Unpin,
    W: tokio::io::AsyncWrite + Unpin,
{
    use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
    let mut r = BufReader::new(r);
    let mut line = String::new();
    loop {
        line.clear();
        match (&mut r).take(MAX_LINE as u64).read_line(&mut line).await {
            Ok(0) | Err(_) => return,
            Ok(_) => {}
        }
        let resp = match serde_json::from_str::<Request>(&line) {
            Ok(req) => daemon.handle(req).await,
            Err(e) => Response::Error { error: ApiError::new("invalid", e.to_string()) },
        };
        let Ok(mut out) = serde_json::to_vec(&resp) else { return };
        out.push(b'\n');
        if w.write_all(&out).await.is_err() {
            return;
        }
    }
}

/// Sends one request over a connected stream and decodes the response data.
async fn exchange<S, T>(stream: S, req: &Request) -> anyhow::Result<T>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
    T: serde::de::DeserializeOwned,
{
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    let (r, mut w) = tokio::io::split(stream);
    let mut out = serde_json::to_vec(req)?;
    out.push(b'\n');
    w.write_all(&out).await?;
    let mut line = String::new();
    BufReader::new(r).read_line(&mut line).await?;
    if line.is_empty() {
        anyhow::bail!("service closed the connection");
    }
    match serde_json::from_str::<Response>(&line)? {
        Response::Ok { data } => Ok(serde_json::from_value(data)?),
        Response::Error { error } => Err(error.into()),
    }
}

#[cfg(unix)]
pub use unix::{call, serve};

#[cfg(unix)]
mod unix {
    use std::path::Path;

    use anyhow::Result;
    use serde::de::DeserializeOwned;
    use tokio::net::{UnixListener, UnixStream};

    use super::Request;
    use crate::daemon::Daemon;

    /// Serves API requests on `listener` until it fails.
    pub async fn serve(listener: UnixListener, daemon: Daemon) {
        while let Ok((stream, _)) = listener.accept().await {
            let (r, w) = stream.into_split();
            tokio::spawn(super::serve_connection(r, w, daemon.clone()));
        }
    }

    /// Sends one request and decodes the response data.
    pub async fn call<T: DeserializeOwned>(socket: &Path, req: &Request) -> Result<T> {
        super::exchange(UnixStream::connect(socket).await?, req).await
    }
}

#[cfg(windows)]
pub use windows::{call, serve_pipe};

#[cfg(windows)]
mod windows {
    use std::path::Path;
    use std::time::Duration;

    use anyhow::{Context, Result};
    use serde::de::DeserializeOwned;
    use tokio::net::windows::named_pipe::{ClientOptions, NamedPipeServer, ServerOptions};

    use super::Request;
    use crate::daemon::Daemon;

    /// SYSTEM and administrators get full access; interactive (locally
    /// logged-on) users may read and write, i.e. use the API.
    const SDDL_INTERACTIVE: &str = "D:P(A;;GA;;;SY)(A;;GA;;;BA)(A;;GRGW;;;IU)";
    /// As above, but any user may use the API.
    const SDDL_EVERYONE: &str = "D:P(A;;GA;;;SY)(A;;GA;;;BA)(A;;GRGW;;;WD)";

    fn create(name: &Path, first: bool, allow_all: bool) -> Result<NamedPipeServer> {
        use windows_sys::Win32::Foundation::LocalFree;
        use windows_sys::Win32::Security::Authorization::ConvertStringSecurityDescriptorToSecurityDescriptorW;
        use windows_sys::Win32::Security::{PSECURITY_DESCRIPTOR, SECURITY_ATTRIBUTES};

        let sddl: Vec<u16> =
            (if allow_all { SDDL_EVERYONE } else { SDDL_INTERACTIVE }).encode_utf16().chain(Some(0)).collect();
        let mut sd: PSECURITY_DESCRIPTOR = std::ptr::null_mut();
        // SAFETY: `sddl` is NUL-terminated; on success `sd` is allocated by
        // the system and freed below with LocalFree.
        if unsafe {
            ConvertStringSecurityDescriptorToSecurityDescriptorW(sddl.as_ptr(), 1, &mut sd, std::ptr::null_mut())
        } == 0
        {
            return Err(std::io::Error::last_os_error()).context("building pipe security descriptor");
        }
        let mut sa = SECURITY_ATTRIBUTES {
            nLength: std::mem::size_of::<SECURITY_ATTRIBUTES>() as u32,
            lpSecurityDescriptor: sd,
            bInheritHandle: 0,
        };
        // SAFETY: `sa` points to a valid descriptor for the duration of the call.
        let server = unsafe {
            ServerOptions::new()
                .first_pipe_instance(first)
                .reject_remote_clients(true)
                .create_with_security_attributes_raw(name.as_os_str(), &mut sa as *mut SECURITY_ATTRIBUTES as *mut _)
        };
        // SAFETY: `sd` came from ConvertStringSecurityDescriptor...
        unsafe { LocalFree(sd as _) };
        server.with_context(|| format!("creating pipe {}", name.display()))
    }

    /// Serves API requests on the named pipe `name` until creating a new
    /// instance fails. The first instance refuses to start if another
    /// process already owns the name.
    pub fn serve_pipe(name: &Path, daemon: Daemon, allow_all: bool) -> Result<impl std::future::Future<Output = ()>> {
        let name = name.to_path_buf();
        let mut server = create(&name, true, allow_all)?;
        Ok(async move {
            loop {
                let connected = server.connect().await;
                // Every connection attempt, failed or not, uses up this pipe
                // instance: a failed one (e.g. ERROR_NO_DATA when a client
                // closed before we accepted it) fails again immediately and
                // without yielding, which would spin forever and starve the
                // runtime. Always continue with a fresh instance.
                let next = loop {
                    match create(&name, false, allow_all) {
                        Ok(s) => break s,
                        Err(e) => {
                            tracing::error!("creating the named pipe failed: {e:#}");
                            tokio::time::sleep(Duration::from_secs(1)).await;
                        }
                    }
                };
                let used = std::mem::replace(&mut server, next);
                match connected {
                    Ok(()) => {
                        let (r, w) = tokio::io::split(used);
                        tokio::spawn(super::serve_connection(r, w, daemon.clone()));
                    }
                    Err(e) => {
                        tracing::debug!("named pipe client went away: {e}");
                        drop(used);
                        tokio::task::yield_now().await;
                    }
                }
            }
        })
    }

    /// Sends one request and decodes the response data.
    pub async fn call<T: DeserializeOwned>(pipe: &Path, req: &Request) -> Result<T> {
        const ERROR_PIPE_BUSY: i32 = 231;
        let mut tries = 0;
        let client = loop {
            match ClientOptions::new().open(pipe.as_os_str()) {
                Ok(c) => break c,
                Err(e) if e.raw_os_error() == Some(ERROR_PIPE_BUSY) && tries < 20 => {
                    tries += 1;
                    tokio::time::sleep(Duration::from_millis(50)).await;
                }
                Err(e) => return Err(e.into()),
            }
        };
        super::exchange(client, req).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::control::ErrorCode;

    #[test]
    fn wire_format() {
        // Older clients send bus ids.
        let r: Request = serde_json::from_str(r#"{"cmd":"set_shared","busid":"1-2","shared":true}"#).unwrap();
        assert_eq!(r, Request::SetShared { device: "1-2".into(), shared: true });
        let r: Request = serde_json::from_str(r#"{"cmd":"set_policy","policy":"restricted"}"#).unwrap();
        assert_eq!(r, Request::SetPolicy { policy: Policy::Restricted });
        let v = serde_json::to_value(AttachmentView {
            server: "fp".into(),
            server_name: "pc".into(),
            device: "0781:5567:X".into(),
            busid: Some("1-2".into()),
            name: None,
            vendor_id: None,
            product_id: None,
            state: AttachState::Attached { port: 3 },
        })
        .unwrap();
        assert_eq!(v["state"], "attached");
        assert_eq!(v["port"], 3);
    }

    #[test]
    fn error_codes() {
        let e: anyhow::Error =
            RemoteError { code: ErrorCode::DeviceBusy, message: "x".into(), queue_position: None }.into();
        assert_eq!(ApiError::from_anyhow(&e).code, "device_busy");
        let e: anyhow::Error = ClientError::PairingRequired { name: "a".into(), fingerprint: "b".into() }.into();
        assert_eq!(ApiError::from_anyhow(&e).code, "pairing_required");
        let e = anyhow::Error::from(std::io::Error::from(std::io::ErrorKind::ConnectionRefused)).context("connecting");
        assert_eq!(ApiError::from_anyhow(&e).code, "unreachable");
        assert_eq!(ApiError::from_anyhow(&anyhow::anyhow!("boom")).code, "other");
    }
}
