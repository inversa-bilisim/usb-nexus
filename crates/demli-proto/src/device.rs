//! Exported USB device description (`struct usbip_usb_device`).

use bytes::{Buf, BufMut};

use crate::{get_fixed_str, need, put_fixed_str, Result, BUSID_SIZE, PATH_SIZE};

/// Encoded size of a device record without interfaces.
pub const DEVICE_WIRE_SIZE: usize = PATH_SIZE + BUSID_SIZE + 4 * 3 + 2 * 3 + 6;
/// Encoded size of one interface record.
pub const INTERFACE_WIRE_SIZE: usize = 4;

/// USB device speed as defined by `enum usb_device_speed`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum Speed {
    Unknown = 0,
    Low = 1,
    Full = 2,
    High = 3,
    Wireless = 4,
    Super = 5,
    SuperPlus = 6,
}

impl Speed {
    pub fn from_u32(v: u32) -> Self {
        match v {
            1 => Speed::Low,
            2 => Speed::Full,
            3 => Speed::High,
            4 => Speed::Wireless,
            5 => Speed::Super,
            6 => Speed::SuperPlus,
            _ => Speed::Unknown,
        }
    }

    /// Parses the value of the sysfs `speed` attribute (in Mbit/s).
    pub fn from_sysfs(s: &str) -> Self {
        match s.trim() {
            "1.5" => Speed::Low,
            "12" => Speed::Full,
            "480" => Speed::High,
            "53.3-480" => Speed::Wireless,
            "5000" => Speed::Super,
            "10000" | "20000" => Speed::SuperPlus,
            _ => Speed::Unknown,
        }
    }

    /// Whether the device needs a SuperSpeed root hub port.
    pub fn is_superspeed(self) -> bool {
        matches!(self, Speed::Super | Speed::SuperPlus)
    }

    pub fn label(self) -> &'static str {
        match self {
            Speed::Unknown => "unknown",
            Speed::Low => "1.5 Mbit/s",
            Speed::Full => "12 Mbit/s",
            Speed::High => "480 Mbit/s",
            Speed::Wireless => "wireless",
            Speed::Super => "5 Gbit/s",
            Speed::SuperPlus => "10+ Gbit/s",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct InterfaceInfo {
    pub class: u8,
    pub subclass: u8,
    pub protocol: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceInfo {
    pub path: String,
    pub busid: String,
    pub busnum: u32,
    pub devnum: u32,
    pub speed: Speed,
    pub id_vendor: u16,
    pub id_product: u16,
    pub bcd_device: u16,
    pub device_class: u8,
    pub device_subclass: u8,
    pub device_protocol: u8,
    pub configuration_value: u8,
    pub num_configurations: u8,
    pub interfaces: Vec<InterfaceInfo>,
}

impl DeviceInfo {
    /// Device id used in URB headers: `busnum << 16 | devnum`.
    pub fn devid(&self) -> u32 {
        (self.busnum << 16) | (self.devnum & 0xffff)
    }

    /// Encodes the device record. Interfaces are appended when `with_interfaces`
    /// is set (OP_REP_DEVLIST) and omitted otherwise (OP_REP_IMPORT).
    pub fn encode(&self, out: &mut Vec<u8>, with_interfaces: bool) -> Result<()> {
        if self.interfaces.len() > u8::MAX as usize {
            return Err(crate::ProtoError::OutOfRange("too many interfaces"));
        }
        put_fixed_str(out, &self.path, PATH_SIZE)?;
        put_fixed_str(out, &self.busid, BUSID_SIZE)?;
        out.put_u32(self.busnum);
        out.put_u32(self.devnum);
        out.put_u32(self.speed as u32);
        out.put_u16(self.id_vendor);
        out.put_u16(self.id_product);
        out.put_u16(self.bcd_device);
        out.put_u8(self.device_class);
        out.put_u8(self.device_subclass);
        out.put_u8(self.device_protocol);
        out.put_u8(self.configuration_value);
        out.put_u8(self.num_configurations);
        out.put_u8(self.interfaces.len() as u8);
        if with_interfaces {
            for i in &self.interfaces {
                out.put_u8(i.class);
                out.put_u8(i.subclass);
                out.put_u8(i.protocol);
                out.put_u8(0);
            }
        }
        Ok(())
    }

    /// Decodes a device record, returning it and the number of bytes consumed.
    pub fn decode(buf: &[u8], with_interfaces: bool) -> Result<(Self, usize)> {
        need(buf, DEVICE_WIRE_SIZE)?;
        let path = get_fixed_str(&buf[..PATH_SIZE]);
        let busid = get_fixed_str(&buf[PATH_SIZE..PATH_SIZE + BUSID_SIZE]);
        let mut b = &buf[PATH_SIZE + BUSID_SIZE..DEVICE_WIRE_SIZE];
        let busnum = b.get_u32();
        let devnum = b.get_u32();
        let speed = Speed::from_u32(b.get_u32());
        let id_vendor = b.get_u16();
        let id_product = b.get_u16();
        let bcd_device = b.get_u16();
        let device_class = b.get_u8();
        let device_subclass = b.get_u8();
        let device_protocol = b.get_u8();
        let configuration_value = b.get_u8();
        let num_configurations = b.get_u8();
        let num_interfaces = b.get_u8() as usize;

        let mut used = DEVICE_WIRE_SIZE;
        let mut interfaces = Vec::new();
        if with_interfaces {
            let total = used + num_interfaces * INTERFACE_WIRE_SIZE;
            need(buf, total)?;
            let mut b = &buf[used..total];
            for _ in 0..num_interfaces {
                let class = b.get_u8();
                let subclass = b.get_u8();
                let protocol = b.get_u8();
                b.advance(1);
                interfaces.push(InterfaceInfo { class, subclass, protocol });
            }
            used = total;
        }

        Ok((
            DeviceInfo {
                path,
                busid,
                busnum,
                devnum,
                speed,
                id_vendor,
                id_product,
                bcd_device,
                device_class,
                device_subclass,
                device_protocol,
                configuration_value,
                num_configurations,
                interfaces,
            },
            used,
        ))
    }
}

#[cfg(test)]
pub(crate) fn sample_device() -> DeviceInfo {
    DeviceInfo {
        path: "/sys/devices/pci0000:00/0000:00:14.0/usb1/1-2".into(),
        busid: "1-2".into(),
        busnum: 1,
        devnum: 5,
        speed: Speed::High,
        id_vendor: 0x0781,
        id_product: 0x5567,
        bcd_device: 0x0100,
        device_class: 0,
        device_subclass: 0,
        device_protocol: 0,
        configuration_value: 1,
        num_configurations: 1,
        interfaces: vec![InterfaceInfo { class: 8, subclass: 6, protocol: 0x50 }],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_with_interfaces() {
        let d = sample_device();
        let mut out = Vec::new();
        d.encode(&mut out, true).unwrap();
        assert_eq!(out.len(), DEVICE_WIRE_SIZE + INTERFACE_WIRE_SIZE);
        let (back, used) = DeviceInfo::decode(&out, true).unwrap();
        assert_eq!(used, out.len());
        assert_eq!(back, d);
    }

    #[test]
    fn roundtrip_without_interfaces() {
        let d = sample_device();
        let mut out = Vec::new();
        d.encode(&mut out, false).unwrap();
        assert_eq!(out.len(), DEVICE_WIRE_SIZE);
        let (back, _) = DeviceInfo::decode(&out, false).unwrap();
        assert_eq!(back.busid, d.busid);
        assert!(back.interfaces.is_empty());
    }

    #[test]
    fn devid() {
        assert_eq!(sample_device().devid(), 0x0001_0005);
    }

    #[test]
    fn rejects_long_busid() {
        let mut d = sample_device();
        d.busid = "x".repeat(BUSID_SIZE);
        assert!(d.encode(&mut Vec::new(), false).is_err());
    }

    #[test]
    fn sysfs_speed() {
        assert_eq!(Speed::from_sysfs("480\n"), Speed::High);
        assert!(Speed::from_sysfs("5000").is_superspeed());
    }
}
