// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! URB phase messages (`USBIP_CMD_SUBMIT`, `USBIP_RET_SUBMIT`,
//! `USBIP_CMD_UNLINK`, `USBIP_RET_UNLINK`).

use bytes::{Buf, BufMut};

use crate::{need, ProtoError, Result};

pub const CMD_SUBMIT: u32 = 0x0000_0001;
pub const CMD_UNLINK: u32 = 0x0000_0002;
pub const RET_SUBMIT: u32 = 0x0000_0003;
pub const RET_UNLINK: u32 = 0x0000_0004;

/// Size of every URB header (basic header + command specific part).
pub const URB_HEADER_SIZE: usize = 48;
/// Size of one isochronous packet descriptor.
pub const ISO_DESC_SIZE: usize = 16;

pub const DIR_OUT: u32 = 0;
pub const DIR_IN: u32 = 1;

/// Upper bound accepted for a single transfer buffer, to protect against
/// malicious peers asking us to allocate huge buffers.
pub const MAX_TRANSFER: usize = 16 * 1024 * 1024;
/// Upper bound on isochronous packets per URB.
pub const MAX_ISO_PACKETS: usize = 1024;

/// Common header present in every URB message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BasicHeader {
    pub command: u32,
    pub seqnum: u32,
    pub devid: u32,
    pub direction: u32,
    pub ep: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct IsoPacket {
    pub offset: u32,
    pub length: u32,
    pub actual_length: u32,
    pub status: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmdSubmit {
    pub header: BasicHeader,
    pub transfer_flags: u32,
    pub transfer_buffer_length: i32,
    pub start_frame: i32,
    pub number_of_packets: i32,
    pub interval: i32,
    pub setup: [u8; 8],
    /// Present only for OUT transfers.
    pub data: Vec<u8>,
    pub iso: Vec<IsoPacket>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetSubmit {
    pub header: BasicHeader,
    pub status: i32,
    pub actual_length: i32,
    pub start_frame: i32,
    pub number_of_packets: i32,
    pub error_count: i32,
    /// Present only for IN transfers.
    pub data: Vec<u8>,
    pub iso: Vec<IsoPacket>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmdUnlink {
    pub header: BasicHeader,
    pub unlink_seqnum: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetUnlink {
    pub header: BasicHeader,
    pub status: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UrbMessage {
    CmdSubmit(CmdSubmit),
    RetSubmit(RetSubmit),
    CmdUnlink(CmdUnlink),
    RetUnlink(RetUnlink),
}

impl BasicHeader {
    fn put(&self, out: &mut Vec<u8>) {
        out.put_u32(self.command);
        out.put_u32(self.seqnum);
        out.put_u32(self.devid);
        out.put_u32(self.direction);
        out.put_u32(self.ep);
    }

    fn get(b: &mut &[u8]) -> Self {
        BasicHeader {
            command: b.get_u32(),
            seqnum: b.get_u32(),
            devid: b.get_u32(),
            direction: b.get_u32(),
            ep: b.get_u32(),
        }
    }

    /// Peeks the basic header of a URB message.
    pub fn peek(buf: &[u8]) -> Result<Self> {
        need(buf, URB_HEADER_SIZE)?;
        Ok(Self::get(&mut &buf[..20]))
    }
}

fn put_iso(out: &mut Vec<u8>, iso: &[IsoPacket]) {
    for p in iso {
        out.put_u32(p.offset);
        out.put_u32(p.length);
        out.put_u32(p.actual_length);
        out.put_u32(p.status);
    }
}

fn iso_count(n: i32) -> Result<usize> {
    // 0xffffffff (-1) means "not isochronous" in the Linux implementation.
    if n <= 0 {
        return Ok(0);
    }
    let n = n as usize;
    if n > MAX_ISO_PACKETS {
        return Err(ProtoError::OutOfRange("number_of_packets"));
    }
    Ok(n)
}

fn get_iso(buf: &[u8], n: usize) -> Result<Vec<IsoPacket>> {
    need(buf, n * ISO_DESC_SIZE)?;
    let mut b = &buf[..n * ISO_DESC_SIZE];
    Ok((0..n)
        .map(|_| IsoPacket {
            offset: b.get_u32(),
            length: b.get_u32(),
            actual_length: b.get_u32(),
            status: b.get_u32(),
        })
        .collect())
}

fn data_len(v: i32) -> Result<usize> {
    if v < 0 || v as usize > MAX_TRANSFER {
        return Err(ProtoError::OutOfRange("transfer length"));
    }
    Ok(v as usize)
}

impl UrbMessage {
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(URB_HEADER_SIZE);
        match self {
            UrbMessage::CmdSubmit(c) => {
                BasicHeader { command: CMD_SUBMIT, ..c.header }.put(&mut out);
                out.put_u32(c.transfer_flags);
                out.put_i32(c.transfer_buffer_length);
                out.put_i32(c.start_frame);
                out.put_i32(c.number_of_packets);
                out.put_i32(c.interval);
                out.extend_from_slice(&c.setup);
                if c.header.direction == DIR_OUT {
                    out.extend_from_slice(&c.data);
                }
                put_iso(&mut out, &c.iso);
            }
            UrbMessage::RetSubmit(r) => {
                BasicHeader { command: RET_SUBMIT, ..r.header }.put(&mut out);
                out.put_i32(r.status);
                out.put_i32(r.actual_length);
                out.put_i32(r.start_frame);
                out.put_i32(r.number_of_packets);
                out.put_i32(r.error_count);
                out.put_u64(0);
                if r.header.direction == DIR_IN {
                    out.extend_from_slice(&r.data);
                }
                put_iso(&mut out, &r.iso);
            }
            UrbMessage::CmdUnlink(c) => {
                BasicHeader { command: CMD_UNLINK, ..c.header }.put(&mut out);
                out.put_u32(c.unlink_seqnum);
                out.resize(URB_HEADER_SIZE, 0);
            }
            UrbMessage::RetUnlink(r) => {
                BasicHeader { command: RET_UNLINK, ..r.header }.put(&mut out);
                out.put_i32(r.status);
                out.resize(URB_HEADER_SIZE, 0);
            }
        }
        out
    }

    /// Decodes one URB message, returning it and the bytes consumed.
    ///
    /// Servers may leave `direction` zero in `RET_SUBMIT`, so the caller
    /// passes the direction of the matching `CMD_SUBMIT` via `ret_direction`.
    pub fn decode(buf: &[u8], ret_direction: Option<u32>) -> Result<(Self, usize)> {
        need(buf, URB_HEADER_SIZE)?;
        let mut b = &buf[..URB_HEADER_SIZE];
        let header = BasicHeader::get(&mut b);
        let rest = &buf[URB_HEADER_SIZE..];
        match header.command {
            CMD_SUBMIT => {
                let transfer_flags = b.get_u32();
                let transfer_buffer_length = b.get_i32();
                let start_frame = b.get_i32();
                let number_of_packets = b.get_i32();
                let interval = b.get_i32();
                let mut setup = [0u8; 8];
                b.copy_to_slice(&mut setup);
                let dlen = if header.direction == DIR_OUT { data_len(transfer_buffer_length)? } else { 0 };
                need(rest, dlen)?;
                let data = rest[..dlen].to_vec();
                let n = iso_count(number_of_packets)?;
                let iso = get_iso(&rest[dlen..], n)?;
                let used = URB_HEADER_SIZE + dlen + n * ISO_DESC_SIZE;
                Ok((
                    UrbMessage::CmdSubmit(CmdSubmit {
                        header,
                        transfer_flags,
                        transfer_buffer_length,
                        start_frame,
                        number_of_packets,
                        interval,
                        setup,
                        data,
                        iso,
                    }),
                    used,
                ))
            }
            RET_SUBMIT => {
                let status = b.get_i32();
                let actual_length = b.get_i32();
                let start_frame = b.get_i32();
                let number_of_packets = b.get_i32();
                let error_count = b.get_i32();
                let dir = ret_direction.unwrap_or(header.direction);
                let dlen = if dir == DIR_IN { data_len(actual_length)? } else { 0 };
                need(rest, dlen)?;
                let data = rest[..dlen].to_vec();
                let n = iso_count(number_of_packets)?;
                let iso = get_iso(&rest[dlen..], n)?;
                let used = URB_HEADER_SIZE + dlen + n * ISO_DESC_SIZE;
                Ok((
                    UrbMessage::RetSubmit(RetSubmit {
                        header: BasicHeader { direction: dir, ..header },
                        status,
                        actual_length,
                        start_frame,
                        number_of_packets,
                        error_count,
                        data,
                        iso,
                    }),
                    used,
                ))
            }
            CMD_UNLINK => {
                Ok((UrbMessage::CmdUnlink(CmdUnlink { header, unlink_seqnum: b.get_u32() }), URB_HEADER_SIZE))
            }
            RET_UNLINK => Ok((UrbMessage::RetUnlink(RetUnlink { header, status: b.get_i32() }), URB_HEADER_SIZE)),
            other => Err(ProtoError::UnknownCommand(other)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hdr(direction: u32) -> BasicHeader {
        BasicHeader { command: 0, seqnum: 7, devid: 0x0001_0002, direction, ep: 1 }
    }

    fn roundtrip(m: UrbMessage, dir: Option<u32>) {
        let enc = m.encode();
        let (dec, used) = UrbMessage::decode(&enc, dir).unwrap();
        assert_eq!(used, enc.len());
        // The encoder fills in the command code.
        assert_eq!(dec.encode(), enc);
    }

    #[test]
    fn submit_out_and_in() {
        roundtrip(
            UrbMessage::CmdSubmit(CmdSubmit {
                header: hdr(DIR_OUT),
                transfer_flags: 0,
                transfer_buffer_length: 4,
                start_frame: 0,
                number_of_packets: -1,
                interval: 0,
                setup: [0; 8],
                data: vec![1, 2, 3, 4],
                iso: vec![],
            }),
            None,
        );
        roundtrip(
            UrbMessage::RetSubmit(RetSubmit {
                header: hdr(DIR_IN),
                status: 0,
                actual_length: 3,
                start_frame: 0,
                number_of_packets: 0,
                error_count: 0,
                data: vec![9, 8, 7],
                iso: vec![],
            }),
            Some(DIR_IN),
        );
    }

    #[test]
    fn iso_packets() {
        roundtrip(
            UrbMessage::CmdSubmit(CmdSubmit {
                header: hdr(DIR_IN),
                transfer_flags: 0,
                transfer_buffer_length: 64,
                start_frame: 0,
                number_of_packets: 2,
                interval: 1,
                setup: [0; 8],
                data: vec![],
                iso: vec![IsoPacket { offset: 0, length: 32, ..Default::default() }; 2],
            }),
            None,
        );
    }

    #[test]
    fn unlink() {
        roundtrip(UrbMessage::CmdUnlink(CmdUnlink { header: hdr(0), unlink_seqnum: 5 }), None);
        roundtrip(UrbMessage::RetUnlink(RetUnlink { header: hdr(0), status: -104 }), None);
        assert_eq!(
            UrbMessage::CmdUnlink(CmdUnlink { header: hdr(0), unlink_seqnum: 5 }).encode().len(),
            URB_HEADER_SIZE
        );
    }

    #[test]
    fn rejects_hostile_lengths() {
        let mut m = UrbMessage::CmdSubmit(CmdSubmit {
            header: hdr(DIR_OUT),
            transfer_flags: 0,
            transfer_buffer_length: 0,
            start_frame: 0,
            number_of_packets: 0,
            interval: 0,
            setup: [0; 8],
            data: vec![],
            iso: vec![],
        })
        .encode();
        // transfer_buffer_length = i32::MAX
        m[24..28].copy_from_slice(&i32::MAX.to_be_bytes());
        assert!(UrbMessage::decode(&m, None).is_err());
    }
}
