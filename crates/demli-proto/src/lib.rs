//! USB/IP wire protocol.
//!
//! Clean-room implementation of the message formats described in the public
//! USB/IP protocol specification (Linux `Documentation/usb/usbip_protocol.rst`).
//! All multi-byte fields are big endian on the wire.
//!
//! The protocol has two phases:
//! * **Operation phase** (`op` module): device listing and import requests.
//! * **URB phase** (`urb` module): USB request blocks exchanged after a
//!   successful import.

pub mod device;
pub mod op;
pub mod urb;

pub use device::{DeviceInfo, InterfaceInfo, Speed};

/// USB/IP protocol version spoken by this implementation (1.1.1).
pub const USBIP_VERSION: u16 = 0x0111;

/// Size of the fixed `busid` field.
pub const BUSID_SIZE: usize = 32;
/// Size of the fixed `path` field.
pub const PATH_SIZE: usize = 256;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ProtoError {
    #[error("message truncated: need {need} bytes, have {have}")]
    Truncated { need: usize, have: usize },
    #[error("unsupported protocol version {0:#06x}")]
    Version(u16),
    #[error("unknown operation code {0:#06x}")]
    UnknownOp(u16),
    #[error("unknown URB command {0:#010x}")]
    UnknownCommand(u32),
    #[error("string field too long ({len} > {max})")]
    StringTooLong { len: usize, max: usize },
    #[error("field value out of range: {0}")]
    OutOfRange(&'static str),
}

pub type Result<T> = std::result::Result<T, ProtoError>;

pub(crate) fn need(buf: &[u8], n: usize) -> Result<()> {
    if buf.len() < n {
        Err(ProtoError::Truncated { need: n, have: buf.len() })
    } else {
        Ok(())
    }
}

/// Writes `s` into a zero-padded fixed-size field.
pub(crate) fn put_fixed_str(out: &mut Vec<u8>, s: &str, size: usize) -> Result<()> {
    let b = s.as_bytes();
    // Leave room for the terminating NUL expected by C implementations.
    if b.len() >= size {
        return Err(ProtoError::StringTooLong { len: b.len(), max: size - 1 });
    }
    out.extend_from_slice(b);
    out.resize(out.len() + (size - b.len()), 0);
    Ok(())
}

/// Reads a NUL-terminated string from a fixed-size field.
pub(crate) fn get_fixed_str(field: &[u8]) -> String {
    let end = field.iter().position(|&c| c == 0).unwrap_or(field.len());
    String::from_utf8_lossy(&field[..end]).into_owned()
}
