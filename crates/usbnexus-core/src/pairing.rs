// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! PIN pairing.
//!
//! The server shows a short PIN; the user types it on the client. Both sides
//! run SPAKE2 with the PIN as password, so an attacker who intercepts the
//! exchange cannot brute-force the PIN offline, and an active attacker gets a
//! single guess per attempt. The resulting key is used to MAC the TLS
//! exporter value, binding the pairing to the TLS session and therefore to
//! the two certificates that get pinned.

use std::time::{Duration, Instant};

use anyhow::{bail, Result};
use hmac::{Hmac, Mac};
use rand::Rng;
use sha2::Sha256;
use spake2::{Ed25519Group, Identity as SpakeId, Password, Spake2};
use subtle::ConstantTimeEq;

const ID_CLIENT: &[u8] = b"usbnexus-client";
const ID_SERVER: &[u8] = b"usbnexus-server";

/// Number of digits in a pairing PIN.
pub const PIN_DIGITS: usize = 6;
/// Failed attempts after which a pairing window closes.
pub const MAX_ATTEMPTS: u32 = 3;

pub fn generate_pin() -> String {
    let n: u32 = rand::thread_rng().gen_range(0..10u32.pow(PIN_DIGITS as u32));
    format!("{n:0width$}", width = PIN_DIGITS)
}

/// Normalises user input: strips spaces and dashes.
pub fn normalize_pin(pin: &str) -> String {
    pin.chars().filter(|c| !c.is_whitespace() && *c != '-').collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Client,
    Server,
}

/// One side of a SPAKE2 exchange.
pub struct Pake {
    state: Spake2<Ed25519Group>,
    role: Role,
}

impl Pake {
    /// Starts the exchange; returns the state and the message for the peer.
    pub fn start(role: Role, pin: &str) -> (Self, Vec<u8>) {
        let pw = Password::new(normalize_pin(pin).as_bytes());
        let (a, b) = (SpakeId::new(ID_CLIENT), SpakeId::new(ID_SERVER));
        let (state, msg) = match role {
            Role::Client => Spake2::<Ed25519Group>::start_a(&pw, &a, &b),
            Role::Server => Spake2::<Ed25519Group>::start_b(&pw, &a, &b),
        };
        (Pake { state, role }, msg)
    }

    /// Completes the exchange with the peer's message, producing confirmation
    /// keys bound to `binding` (the TLS exporter value).
    pub fn finish(self, peer_msg: &[u8], binding: &[u8]) -> Result<Confirm> {
        let key = match self.state.finish(peer_msg) {
            Ok(k) => k,
            Err(_) => bail!("invalid pairing message"),
        };
        Ok(Confirm { key, binding: binding.to_vec(), role: self.role })
    }
}

pub struct Confirm {
    key: Vec<u8>,
    binding: Vec<u8>,
    role: Role,
}

impl Confirm {
    fn mac_for(&self, role: Role) -> Vec<u8> {
        let mut m = <Hmac<Sha256> as Mac>::new_from_slice(&self.key).expect("HMAC accepts any key length");
        m.update(match role {
            Role::Client => b"usbnexus-confirm-client",
            Role::Server => b"usbnexus-confirm-server",
        });
        m.update(&self.binding);
        m.finalize().into_bytes().to_vec()
    }

    /// MAC this side sends.
    pub fn own_mac(&self) -> Vec<u8> {
        self.mac_for(self.role)
    }

    /// Verifies the MAC received from the peer in constant time.
    pub fn verify_peer(&self, mac: &[u8]) -> bool {
        let peer = match self.role {
            Role::Client => Role::Server,
            Role::Server => Role::Client,
        };
        self.mac_for(peer).ct_eq(mac).into()
    }
}

/// A time-limited window during which a server accepts pairing requests.
#[derive(Debug, Clone)]
pub struct PairingWindow {
    pub pin: String,
    pub expires: Instant,
    pub failures: u32,
}

impl PairingWindow {
    pub fn open(duration: Duration) -> Self {
        PairingWindow { pin: generate_pin(), expires: Instant::now() + duration, failures: 0 }
    }

    pub fn with_pin(pin: &str, duration: Duration) -> Self {
        PairingWindow { pin: normalize_pin(pin), expires: Instant::now() + duration, failures: 0 }
    }

    pub fn is_active(&self) -> bool {
        Instant::now() < self.expires && self.failures < MAX_ATTEMPTS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(client_pin: &str, server_pin: &str, cb: &[u8], sb: &[u8]) -> (bool, bool) {
        let (c, cm) = Pake::start(Role::Client, client_pin);
        let (s, sm) = Pake::start(Role::Server, server_pin);
        let c = c.finish(&sm, cb).unwrap();
        let s = s.finish(&cm, sb).unwrap();
        (s.verify_peer(&c.own_mac()), c.verify_peer(&s.own_mac()))
    }

    #[test]
    fn matching_pin_confirms() {
        assert_eq!(run("123 456", "123-456", b"tls", b"tls"), (true, true));
    }

    #[test]
    fn wrong_pin_fails() {
        assert_eq!(run("123456", "654321", b"tls", b"tls"), (false, false));
    }

    #[test]
    fn different_tls_session_fails() {
        // A man in the middle terminates two TLS sessions with different
        // exporter values; confirmation must not verify across them.
        assert_eq!(run("123456", "123456", b"session-a", b"session-b"), (false, false));
    }

    #[test]
    fn pin_format() {
        let p = generate_pin();
        assert_eq!(p.len(), PIN_DIGITS);
        assert!(p.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn window_closes_after_failures() {
        let mut w = PairingWindow::open(Duration::from_secs(60));
        assert!(w.is_active());
        w.failures = MAX_ATTEMPTS;
        assert!(!w.is_active());
        assert!(!PairingWindow::open(Duration::ZERO).is_active());
    }
}
