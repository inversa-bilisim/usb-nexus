// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! Control protocol spoken inside the TLS tunnel before the URB phase.
//!
//! ```text
//! client                                   server
//!   Hello ─────────────────────────────────▶
//!   ◀──────────────────────────────── Welcome (tells whether client is trusted)
//!   [PairStart ──▶ ◀── PairStart]            only when either side is untrusted
//!   [PairConfirm ──▶ ◀── PairConfirm]
//!   ListDevices ──▶ ◀── Devices              any number of requests
//!   Import ──▶ ◀── Imported                  after this the stream carries raw USB/IP URBs
//! ```

use serde::{Deserialize, Serialize};
use usbnexus_proto::DeviceInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMsg {
    Hello {
        version: u32,
        name: String,
    },
    PairStart {
        spake: Vec<u8>,
    },
    PairConfirm {
        mac: Vec<u8>,
    },
    ListDevices,
    /// `device` is a device identity (see [`crate::device_id::DeviceId`]);
    /// older clients send a bus id.
    Import {
        #[serde(alias = "busid")]
        device: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMsg {
    Welcome {
        version: u32,
        name: String,
        client_trusted: bool,
        pairing_open: bool,
    },
    PairStart {
        spake: Vec<u8>,
    },
    PairConfirm {
        mac: Vec<u8>,
    },
    Devices {
        devices: Vec<ExportedDevice>,
    },
    Imported {
        device: DeviceInfo,
        /// Identity of the imported device; clients asking by bus id should
        /// use it from now on.
        #[serde(default)]
        id: Option<String>,
    },
    Error {
        code: ErrorCode,
        message: String,
    },
}

fn yes() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExportedDevice {
    /// Device identity (see [`crate::device_id::DeviceId`]).
    #[serde(default)]
    pub id: String,
    /// Description of the device. While it is unplugged only the vendor and
    /// product id are filled in.
    pub info: DeviceInfo,
    /// Whether the device is plugged in.
    #[serde(default = "yes")]
    pub present: bool,
    /// Whether the asking computer may use the device.
    #[serde(default = "yes")]
    pub allowed: bool,
    /// Human readable product name, when known.
    #[serde(default)]
    pub product: Option<String>,
    #[serde(default)]
    pub manufacturer: Option<String>,
    /// Whether another client is currently using the device.
    pub in_use: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    Version,
    NotTrusted,
    PairingClosed,
    PairingFailed,
    NoSuchDevice,
    DeviceBusy,
    /// The computer is paired but not allowed to use the device.
    AccessDenied,
    Internal,
    Protocol,
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ErrorCode::Version => "incompatible version",
            ErrorCode::NotTrusted => "not paired",
            ErrorCode::PairingClosed => "pairing is not open on the server",
            ErrorCode::PairingFailed => "pairing failed (wrong PIN?)",
            ErrorCode::NoSuchDevice => "no such device",
            ErrorCode::DeviceBusy => "device is in use",
            ErrorCode::AccessDenied => "not allowed to use this device",
            ErrorCode::Internal => "internal server error",
            ErrorCode::Protocol => "protocol error",
        })
    }
}

/// Error returned by the server, surfaced to client callers.
#[derive(Debug, thiserror::Error)]
#[error("{code}: {message}")]
pub struct RemoteError {
    pub code: ErrorCode,
    pub message: String,
}
