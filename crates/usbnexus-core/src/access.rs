// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! Which paired computers may use which shared devices.
//!
//! Identity is per computer (its paired certificate). Pairing is always
//! required; on top of that the server has a default policy:
//!
//! * **open**: every paired computer may use every shared device;
//! * **restricted**: a computer may use a device only when allowed for it.
//!
//! Each shared device may override the default: always open, or only the
//! selected computers. A device following a restricted default and a device
//! set to "selected computers" use the same per-device list of computers.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Policy {
    /// Every paired computer may use every shared device.
    #[default]
    Open,
    /// Only computers allowed per device.
    Restricted,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceMode {
    /// Follow the server-wide policy.
    #[default]
    Default,
    /// Every paired computer.
    Open,
    /// Only the computers in the device's list.
    Selected,
}

/// Access settings of one shared device.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceAccess {
    #[serde(default)]
    pub mode: DeviceMode,
    /// Fingerprints of the computers allowed when access is limited.
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub allowed: BTreeSet<String>,
}

impl DeviceAccess {
    /// Whether every paired computer may use the device under `policy`.
    pub fn is_open(&self, policy: Policy) -> bool {
        match self.mode {
            DeviceMode::Default => policy == Policy::Open,
            DeviceMode::Open => true,
            DeviceMode::Selected => false,
        }
    }

    /// Whether the computer with `fingerprint` may use the device.
    pub fn allows(&self, policy: Policy, fingerprint: &str) -> bool {
        self.is_open(policy) || self.allowed.contains(fingerprint)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn effective_access() {
        let mut a = DeviceAccess::default();
        assert!(a.allows(Policy::Open, "pc1"));
        assert!(!a.allows(Policy::Restricted, "pc1"));
        a.allowed.insert("pc1".into());
        assert!(a.allows(Policy::Restricted, "pc1"));
        assert!(!a.allows(Policy::Restricted, "pc2"));

        a.mode = DeviceMode::Selected;
        assert!(!a.allows(Policy::Open, "pc2"), "selected overrides an open default");
        assert!(a.allows(Policy::Open, "pc1"));

        a.mode = DeviceMode::Open;
        assert!(a.allows(Policy::Restricted, "pc2"), "open overrides a restricted default");
    }

    #[test]
    fn wire_format() {
        assert_eq!(serde_json::to_string(&Policy::Restricted).unwrap(), "\"restricted\"");
        let a: DeviceAccess = serde_json::from_str("{}").unwrap();
        assert_eq!(a, DeviceAccess::default());
    }
}
