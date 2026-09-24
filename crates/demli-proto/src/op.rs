// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Demli contributors

//! Operation phase messages (device list and import).

use bytes::{Buf, BufMut};

use crate::device::DeviceInfo;
use crate::{get_fixed_str, need, put_fixed_str, ProtoError, Result, BUSID_SIZE, USBIP_VERSION};

pub const OP_REQ_DEVLIST: u16 = 0x8005;
pub const OP_REP_DEVLIST: u16 = 0x0005;
pub const OP_REQ_IMPORT: u16 = 0x8003;
pub const OP_REP_IMPORT: u16 = 0x0003;

/// Size of the common operation header.
pub const OP_HEADER_SIZE: usize = 8;

pub const ST_OK: u32 = 0;
pub const ST_NA: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpMessage {
    ReqDevlist,
    RepDevlist { status: u32, devices: Vec<DeviceInfo> },
    ReqImport { busid: String },
    /// `device` is present only when `status == ST_OK`.
    RepImport { status: u32, device: Option<DeviceInfo> },
}

impl OpMessage {
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut out = Vec::new();
        let (code, status) = match self {
            OpMessage::ReqDevlist => (OP_REQ_DEVLIST, ST_OK),
            OpMessage::RepDevlist { status, .. } => (OP_REP_DEVLIST, *status),
            OpMessage::ReqImport { .. } => (OP_REQ_IMPORT, ST_OK),
            OpMessage::RepImport { status, .. } => (OP_REP_IMPORT, *status),
        };
        out.put_u16(USBIP_VERSION);
        out.put_u16(code);
        out.put_u32(status);
        match self {
            OpMessage::ReqDevlist => {}
            OpMessage::RepDevlist { devices, .. } => {
                out.put_u32(devices.len() as u32);
                for d in devices {
                    d.encode(&mut out, true)?;
                }
            }
            OpMessage::ReqImport { busid } => put_fixed_str(&mut out, busid, BUSID_SIZE)?,
            OpMessage::RepImport { device, .. } => {
                if let Some(d) = device {
                    d.encode(&mut out, false)?;
                }
            }
        }
        Ok(out)
    }

    /// Decodes one complete message, returning it and the bytes consumed.
    pub fn decode(buf: &[u8]) -> Result<(Self, usize)> {
        need(buf, OP_HEADER_SIZE)?;
        let mut h = &buf[..OP_HEADER_SIZE];
        let version = h.get_u16();
        let code = h.get_u16();
        let status = h.get_u32();
        if version != USBIP_VERSION {
            return Err(ProtoError::Version(version));
        }
        let body = &buf[OP_HEADER_SIZE..];
        match code {
            OP_REQ_DEVLIST => Ok((OpMessage::ReqDevlist, OP_HEADER_SIZE)),
            OP_REP_DEVLIST => {
                need(body, 4)?;
                let n = (&body[..4]).get_u32() as usize;
                let mut off = 4;
                // Every record is at least DEVICE_WIRE_SIZE bytes; reject absurd counts early.
                if n > body.len() / crate::device::DEVICE_WIRE_SIZE + 1 {
                    return Err(ProtoError::Truncated {
                        need: n * crate::device::DEVICE_WIRE_SIZE,
                        have: body.len(),
                    });
                }
                let mut devices = Vec::with_capacity(n);
                for _ in 0..n {
                    let (d, used) = DeviceInfo::decode(&body[off..], true)?;
                    off += used;
                    devices.push(d);
                }
                Ok((OpMessage::RepDevlist { status, devices }, OP_HEADER_SIZE + off))
            }
            OP_REQ_IMPORT => {
                need(body, BUSID_SIZE)?;
                let busid = get_fixed_str(&body[..BUSID_SIZE]);
                Ok((OpMessage::ReqImport { busid }, OP_HEADER_SIZE + BUSID_SIZE))
            }
            OP_REP_IMPORT => {
                if status != ST_OK {
                    return Ok((OpMessage::RepImport { status, device: None }, OP_HEADER_SIZE));
                }
                let (d, used) = DeviceInfo::decode(body, false)?;
                Ok((OpMessage::RepImport { status, device: Some(d) }, OP_HEADER_SIZE + used))
            }
            other => Err(ProtoError::UnknownOp(other)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::sample_device;

    fn roundtrip(m: OpMessage) {
        let enc = m.encode().unwrap();
        let (dec, used) = OpMessage::decode(&enc).unwrap();
        assert_eq!(used, enc.len());
        assert_eq!(dec, m);
    }

    #[test]
    fn all_messages_roundtrip() {
        roundtrip(OpMessage::ReqDevlist);
        roundtrip(OpMessage::RepDevlist { status: ST_OK, devices: vec![sample_device(), sample_device()] });
        roundtrip(OpMessage::ReqImport { busid: "3-1.4".into() });
        roundtrip(OpMessage::RepImport { status: ST_NA, device: None });
        let mut d = sample_device();
        d.interfaces.clear();
        roundtrip(OpMessage::RepImport { status: ST_OK, device: Some(d) });
    }

    #[test]
    fn req_devlist_bytes() {
        assert_eq!(OpMessage::ReqDevlist.encode().unwrap(), [0x01, 0x11, 0x80, 0x05, 0, 0, 0, 0]);
    }

    #[test]
    fn rejects_bad_version() {
        let buf = [0x01, 0x00, 0x80, 0x05, 0, 0, 0, 0];
        assert_eq!(OpMessage::decode(&buf), Err(ProtoError::Version(0x0100)));
    }

    #[test]
    fn rejects_truncated_and_huge_counts() {
        let mut enc = OpMessage::RepDevlist { status: 0, devices: vec![sample_device()] }.encode().unwrap();
        enc.truncate(enc.len() - 1);
        assert!(OpMessage::decode(&enc).is_err());

        let mut huge = vec![0x01, 0x11, 0x00, 0x05, 0, 0, 0, 0];
        huge.extend_from_slice(&u32::MAX.to_be_bytes());
        assert!(OpMessage::decode(&huge).is_err());
    }
}
