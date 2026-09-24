// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! Local API between the privileged service and user interfaces (GUI, CLI).
//!
//! Transport: newline-delimited JSON over a local socket. Each request line
//! gets exactly one response line. Errors carry a stable `code` that user
//! interfaces translate (message id `err-<code>` with `_` replaced by `-`).

use serde::{Deserialize, Serialize};

use crate::client::ClientError;
use crate::control::RemoteError;
use crate::trust::Peer;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "cmd", rename_all = "snake_case")]
pub enum Request {
    Status,
    /// USB devices of this computer, with their sharing state.
    LocalDevices,
    SetShared {
        busid: String,
        shared: bool,
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
    Attach {
        server: String,
        busid: String,
    },
    Detach {
        server: String,
        busid: String,
    },
    Attachments,
    Peers,
    Forget {
        fingerprint: String,
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
    pub fingerprint: String,
    pub version: String,
    pub listen: String,
    pub pairing: Option<PairingView>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LocalDeviceView {
    pub busid: String,
    pub vendor_id: u16,
    pub product_id: u16,
    pub product: Option<String>,
    pub manufacturer: Option<String>,
    pub speed: String,
    pub shared: bool,
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
    pub busid: String,
    pub vendor_id: u16,
    pub product_id: u16,
    pub product: Option<String>,
    pub manufacturer: Option<String>,
    pub speed: String,
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
    Retrying {
        seconds: u64,
        reason: String,
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
    pub busid: String,
    pub vendor_id: Option<u16>,
    pub product_id: Option<u16>,
    #[serde(flatten)]
    pub state: AttachState,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PeersView {
    /// Servers this computer can use.
    pub servers: Vec<Peer>,
    /// Clients allowed to use this computer's devices.
    pub clients: Vec<Peer>,
}

/// Socket of the system service: `$USBNEXUS_SOCKET`, or the platform default.
pub fn default_socket() -> std::path::PathBuf {
    if let Some(p) = std::env::var_os("USBNEXUS_SOCKET") {
        return p.into();
    }
    std::path::PathBuf::from("/run/usbnexus/daemon.sock")
}

#[cfg(unix)]
pub use unix::{call, serve};

#[cfg(unix)]
mod unix {
    use std::path::Path;

    use anyhow::{bail, Result};
    use serde::de::DeserializeOwned;
    use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
    use tokio::net::{UnixListener, UnixStream};

    use super::{ApiError, Request, Response};
    use crate::daemon::Daemon;

    /// Largest accepted request line.
    const MAX_LINE: usize = 64 * 1024;

    /// Serves API requests on `listener` until it fails.
    pub async fn serve(listener: UnixListener, daemon: Daemon) {
        while let Ok((stream, _)) = listener.accept().await {
            let daemon = daemon.clone();
            tokio::spawn(async move {
                let (r, mut w) = stream.into_split();
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
            });
        }
    }

    /// Sends one request and decodes the response data.
    pub async fn call<T: DeserializeOwned>(socket: &Path, req: &Request) -> Result<T> {
        let stream = UnixStream::connect(socket).await?;
        let (r, mut w) = stream.into_split();
        let mut out = serde_json::to_vec(req)?;
        out.push(b'\n');
        w.write_all(&out).await?;
        let mut line = String::new();
        BufReader::new(r).read_line(&mut line).await?;
        if line.is_empty() {
            bail!("service closed the connection");
        }
        match serde_json::from_str::<Response>(&line)? {
            Response::Ok { data } => Ok(serde_json::from_value(data)?),
            Response::Error { error } => Err(error.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::control::ErrorCode;

    #[test]
    fn wire_format() {
        let r: Request = serde_json::from_str(r#"{"cmd":"set_shared","busid":"1-2","shared":true}"#).unwrap();
        assert_eq!(r, Request::SetShared { busid: "1-2".into(), shared: true });
        let v = serde_json::to_value(AttachmentView {
            server: "fp".into(),
            server_name: "pc".into(),
            busid: "1-2".into(),
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
        let e: anyhow::Error = RemoteError { code: ErrorCode::DeviceBusy, message: "x".into() }.into();
        assert_eq!(ApiError::from_anyhow(&e).code, "device_busy");
        let e: anyhow::Error = ClientError::PairingRequired { name: "a".into(), fingerprint: "b".into() }.into();
        assert_eq!(ApiError::from_anyhow(&e).code, "pairing_required");
        let e = anyhow::Error::from(std::io::Error::from(std::io::ErrorKind::ConnectionRefused)).context("connecting");
        assert_eq!(ApiError::from_anyhow(&e).code, "unreachable");
        assert_eq!(ApiError::from_anyhow(&anyhow::anyhow!("boom")).code, "other");
    }
}
