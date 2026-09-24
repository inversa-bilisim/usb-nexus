// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Demli contributors

//! Demli core library.
//!
//! Demli carries standard USB/IP traffic inside a mutually authenticated
//! TLS 1.3 tunnel. The kernel (or, on Windows, a signed USB/IP driver) keeps
//! speaking plain USB/IP to a loopback socket owned by this library, which
//! relays the bytes to the peer over TLS. On top of that the library adds:
//!
//! * certificate based identities and a trust store ([`identity`], [`trust`]),
//! * PIN pairing using a PAKE bound to the TLS session ([`pairing`]),
//! * LAN discovery over mDNS ([`discovery`]),
//! * automatic reconnection ([`client::attach_forever`], [`backoff`]).

pub mod backend;
pub mod backoff;
pub mod client;
pub mod control;
pub mod discovery;
pub mod frame;
pub mod identity;
pub mod pairing;
pub mod relay;
pub mod server;
pub mod tls;
pub mod trust;

#[cfg(target_os = "linux")]
pub mod linux;

/// Default TCP port of the Demli service. The plain USB/IP port (3240) is
/// left free so Demli can coexist with the stock `usbipd`.
pub const DEFAULT_PORT: u16 = 3241;

/// Version of the Demli control protocol.
pub const CONTROL_VERSION: u32 = 1;
