// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! Device kinds and automatic handover.
//!
//! A USB device serves one computer at a time; others wait in a queue.
//! With automatic handover, a device whose user has not exchanged any data
//! with it for a while goes to the next computer in the queue (never while
//! nobody waits). Whether that is on by default depends on the kind of
//! device: licence dongles and printers are typically used briefly by many
//! computers, storage and input devices are not.

use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::backend::LocalDevice;

/// Idle time before a device is handed over, when that is on by default.
pub const DEFAULT_IDLE_SECS: u32 = 30;

/// Vendors whose devices are licence dongles: Aladdin/Thales (Sentinel
/// HASP), Rainbow (Sentinel SuperPro), WIBU-Systems (CodeMeter), Feitian
/// (Rockey), MARX CryptoTech.
const DONGLE_VENDORS: &[u16] = &[0x0529, 0x04b9, 0x064f, 0x096e, 0x0d7a];

const CLASS_HID: u8 = 0x03;
const CLASS_PRINTER: u8 = 0x07;
const CLASS_STORAGE: u8 = 0x08;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DeviceKind {
    Storage,
    /// Keyboards, mice and other HID devices.
    Input,
    Printer,
    Dongle,
    Other,
}

impl DeviceKind {
    pub fn of(device: &LocalDevice) -> DeviceKind {
        let info = &device.info;
        if DONGLE_VENDORS.contains(&info.id_vendor) {
            return DeviceKind::Dongle;
        }
        // The class is given per interface when the device class is 0.
        let has = |class: u8| info.device_class == class || info.interfaces.iter().any(|i| i.class == class);
        if has(CLASS_PRINTER) {
            DeviceKind::Printer
        } else if has(CLASS_STORAGE) {
            DeviceKind::Storage
        } else if has(CLASS_HID) {
            DeviceKind::Input
        } else {
            DeviceKind::Other
        }
    }

    /// Whether automatic handover is on for this kind unless chosen otherwise.
    pub fn hands_over_by_default(self) -> bool {
        matches!(self, DeviceKind::Dongle | DeviceKind::Printer)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HandoverMode {
    /// As the kind of device suggests.
    #[default]
    Default,
    On,
    Off,
}

/// The handover setting of one shared device.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Handover {
    #[serde(default)]
    pub mode: HandoverMode,
    /// Idle time; [`DEFAULT_IDLE_SECS`] if not set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seconds: Option<u32>,
}

impl Handover {
    pub fn is_default(&self) -> bool {
        *self == Handover::default()
    }

    /// Idle time after which a device of `kind` goes to the next computer,
    /// or `None` if it stays with its user.
    pub fn idle_time(&self, kind: DeviceKind) -> Option<Duration> {
        let on = match self.mode {
            HandoverMode::Default => kind.hands_over_by_default(),
            HandoverMode::On => true,
            HandoverMode::Off => false,
        };
        on.then(|| Duration::from_secs(u64::from(self.seconds.unwrap_or(DEFAULT_IDLE_SECS).max(1))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use usbnexus_proto::{DeviceInfo, InterfaceInfo, Speed};

    fn device(vendor: u16, class: u8, interfaces: &[u8]) -> LocalDevice {
        LocalDevice {
            info: DeviceInfo {
                path: String::new(),
                busid: "1-1".into(),
                busnum: 1,
                devnum: 2,
                speed: Speed::Full,
                id_vendor: vendor,
                id_product: 1,
                bcd_device: 0,
                device_class: class,
                device_subclass: 0,
                device_protocol: 0,
                configuration_value: 1,
                num_configurations: 1,
                interfaces: interfaces.iter().map(|c| InterfaceInfo { class: *c, subclass: 0, protocol: 0 }).collect(),
            },
            product: None,
            manufacturer: None,
            serial: None,
            driver: None,
        }
    }

    #[test]
    fn kinds_and_defaults() {
        // A Sentinel dongle is a dongle even though it presents HID.
        assert_eq!(DeviceKind::of(&device(0x0529, 0, &[CLASS_HID])), DeviceKind::Dongle);
        assert_eq!(DeviceKind::of(&device(0x0781, 0, &[CLASS_STORAGE])), DeviceKind::Storage);
        assert_eq!(DeviceKind::of(&device(0x046d, 0, &[CLASS_HID, CLASS_HID])), DeviceKind::Input);
        assert_eq!(DeviceKind::of(&device(0x03f0, 0, &[0xff, CLASS_PRINTER])), DeviceKind::Printer);
        assert_eq!(DeviceKind::of(&device(0x1234, 0xff, &[0xff])), DeviceKind::Other);

        let default = Handover::default();
        assert_eq!(default.idle_time(DeviceKind::Dongle), Some(Duration::from_secs(30)));
        assert_eq!(default.idle_time(DeviceKind::Printer), Some(Duration::from_secs(30)));
        assert_eq!(default.idle_time(DeviceKind::Storage), None);
        let on = Handover { mode: HandoverMode::On, seconds: Some(10) };
        assert_eq!(on.idle_time(DeviceKind::Storage), Some(Duration::from_secs(10)));
        let off = Handover { mode: HandoverMode::Off, seconds: None };
        assert_eq!(off.idle_time(DeviceKind::Dongle), None);
    }
}
