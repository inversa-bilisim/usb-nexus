// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! Stable identity of a USB device across unplugging and port changes.
//!
//! A device with a serial number is identified by vendor id, product id and
//! serial, so it keeps its identity on any port. A device without a serial
//! can only be told apart by where it is plugged in, so it is tracked by
//! port (bus id) together with its vendor and product id.
//!
//! Text form (used in configuration files, the local API and the control
//! protocol):
//!
//! ```text
//! 0781:5567:4C530001231120115142   vendor:product:serial
//! 046d:c52b@1-2                    vendor:product@busid (tracked by port)
//! 1-2                              bus id only (older configurations)
//! ```

use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::backend::LocalDevice;

/// Longest serial number kept; longer ones are truncated.
const MAX_SERIAL: usize = 126;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DeviceId {
    Serial {
        vendor: u16,
        product: u16,
        serial: String,
    },
    Port {
        vendor: u16,
        product: u16,
        busid: String,
    },
    /// A bus id from an older configuration; replaced by one of the other
    /// forms as soon as a device is seen on that port.
    Legacy {
        busid: String,
    },
}

/// Cleans a serial number reported by a device; `None` if it is unusable.
pub fn clean_serial(serial: &str) -> Option<String> {
    let s: String = serial.trim().chars().filter(|c| !c.is_control()).take(MAX_SERIAL).collect();
    // Some devices report a serial made of one repeated character (often
    // zeros); identical devices then share it, so it identifies nothing.
    let mut chars = s.chars();
    let first = chars.next()?;
    if s.chars().count() > 1 && chars.all(|c| c == first) {
        return None;
    }
    Some(s)
}

fn parse_ids(s: &str) -> Option<(u16, u16)> {
    let (v, p) = s.split_once(':')?;
    if v.len() != 4 || p.len() != 4 {
        return None;
    }
    Some((u16::from_str_radix(v, 16).ok()?, u16::from_str_radix(p, 16).ok()?))
}

impl DeviceId {
    /// Identity of a connected device.
    pub fn of(d: &LocalDevice) -> DeviceId {
        let (vendor, product) = (d.info.id_vendor, d.info.id_product);
        match d.serial.as_deref().and_then(clean_serial) {
            Some(serial) => DeviceId::Serial { vendor, product, serial },
            None => DeviceId::Port { vendor, product, busid: d.info.busid.clone() },
        }
    }

    /// Parses the text form. Anything that is not a full identity is taken
    /// as a bus id.
    pub fn parse(s: &str) -> DeviceId {
        let s = s.trim();
        if s.len() > 9 && s.is_char_boundary(9) {
            if let Some((vendor, product)) = parse_ids(&s[..9]) {
                let rest = &s[10..];
                match s.as_bytes()[9] {
                    b':' if !rest.is_empty() => {
                        return DeviceId::Serial { vendor, product, serial: rest.to_string() };
                    }
                    b'@' if !rest.is_empty() => return DeviceId::Port { vendor, product, busid: rest.to_string() },
                    _ => {}
                }
            }
        }
        DeviceId::Legacy { busid: s.to_string() }
    }

    /// Whether `d` is this device.
    pub fn matches(&self, d: &LocalDevice) -> bool {
        match self {
            DeviceId::Legacy { busid } => d.info.busid == *busid,
            id => DeviceId::of(d) == *id,
        }
    }

    /// Whether the device is told apart by its port only.
    pub fn by_port(&self) -> bool {
        !matches!(self, DeviceId::Serial { .. })
    }

    /// Vendor and product id, when known.
    pub fn ids(&self) -> Option<(u16, u16)> {
        match self {
            DeviceId::Serial { vendor, product, .. } | DeviceId::Port { vendor, product, .. } => {
                Some((*vendor, *product))
            }
            DeviceId::Legacy { .. } => None,
        }
    }

    /// The port the device is tracked on, for devices without a serial.
    pub fn busid(&self) -> Option<&str> {
        match self {
            DeviceId::Serial { .. } => None,
            DeviceId::Port { busid, .. } | DeviceId::Legacy { busid } => Some(busid),
        }
    }

    pub fn is_legacy(&self) -> bool {
        matches!(self, DeviceId::Legacy { .. })
    }
}

impl fmt::Display for DeviceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceId::Serial { vendor, product, serial } => write!(f, "{vendor:04x}:{product:04x}:{serial}"),
            DeviceId::Port { vendor, product, busid } => write!(f, "{vendor:04x}:{product:04x}@{busid}"),
            DeviceId::Legacy { busid } => f.write_str(busid),
        }
    }
}

impl Serialize for DeviceId {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for DeviceId {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(DeviceId::parse(&String::deserialize(d)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use usbnexus_proto::{DeviceInfo, Speed};

    pub(crate) fn dev(busid: &str, serial: Option<&str>) -> LocalDevice {
        LocalDevice {
            info: DeviceInfo {
                path: String::new(),
                busid: busid.into(),
                busnum: 1,
                devnum: 2,
                speed: Speed::High,
                id_vendor: 0x0781,
                id_product: 0x5567,
                bcd_device: 0,
                device_class: 0,
                device_subclass: 0,
                device_protocol: 0,
                configuration_value: 1,
                num_configurations: 1,
                interfaces: vec![],
            },
            product: None,
            manufacturer: None,
            serial: serial.map(str::to_string),
            driver: None,
        }
    }

    #[test]
    fn text_form_round_trips() {
        for s in ["0781:5567:AB:C@1", "046d:c52b@1-2.3", "1-2", "3-1.4"] {
            assert_eq!(DeviceId::parse(s).to_string(), s);
        }
        assert_eq!(
            DeviceId::parse("0781:5567:SER"),
            DeviceId::Serial { vendor: 0x0781, product: 0x5567, serial: "SER".into() }
        );
        assert!(DeviceId::parse("1-2").is_legacy());
        assert!(DeviceId::parse("0781:5567:").is_legacy());
        assert!(DeviceId::parse("zzzz:5567@1-1").is_legacy());
        let json = serde_json::to_string(&DeviceId::parse("046d:c52b@1-2")).unwrap();
        assert_eq!(json, "\"046d:c52b@1-2\"");
        assert_eq!(serde_json::from_str::<DeviceId>(&json).unwrap(), DeviceId::parse("046d:c52b@1-2"));
    }

    #[test]
    fn serial_devices_follow_the_device() {
        let id = DeviceId::of(&dev("1-1", Some(" 4C5300 ")));
        assert_eq!(id.to_string(), "0781:5567:4C5300");
        assert!(!id.by_port());
        assert!(id.matches(&dev("2-4", Some("4C5300"))), "same device, other port");
        assert!(!id.matches(&dev("1-1", Some("other"))));
    }

    #[test]
    fn devices_without_serial_are_tracked_by_port() {
        for serial in [None, Some(""), Some("000000000"), Some("  ")] {
            let id = DeviceId::of(&dev("1-1", serial));
            assert_eq!(id.to_string(), "0781:5567@1-1", "{serial:?}");
            assert!(id.by_port());
            assert!(!id.matches(&dev("1-2", None)));
        }
        assert_eq!(clean_serial("7"), Some("7".into()));
        assert!(DeviceId::parse("1-1").matches(&dev("1-1", Some("x"))));
    }
}
