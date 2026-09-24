// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Demli contributors

//! Platform backends.
//!
//! A backend connects Demli to the operating system's USB/IP implementation.
//! It never parses URBs itself: it hands one end of a loopback TCP socket to
//! the OS stack and returns the other end, which Demli relays over TLS.

use std::io;
use std::net::{TcpListener, TcpStream};

use anyhow::Result;
use demli_proto::DeviceInfo;

/// A local USB device that can be exported.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalDevice {
    pub info: DeviceInfo,
    pub product: Option<String>,
    pub manufacturer: Option<String>,
    /// Kernel driver currently bound to the device, if any.
    pub driver: Option<String>,
}

/// Server side: shares local devices.
pub trait ExportBackend: Send + Sync + 'static {
    /// Devices this server offers.
    fn list(&self) -> Result<Vec<LocalDevice>>;

    /// Hands `busid` to the USB/IP stack and returns the socket that carries
    /// its URB traffic. Dropping the socket ends the export.
    fn export(&self, busid: &str) -> Result<tokio::net::TcpStream>;
}

/// Client side: attaches remote devices to a virtual host controller.
pub trait ImportBackend: Send + Sync + 'static {
    /// Attaches `device`; returns the virtual port and the socket that
    /// carries its URB traffic. Dropping the socket detaches the device.
    fn attach(&self, device: &DeviceInfo) -> Result<(u32, tokio::net::TcpStream)>;

    /// Explicitly detaches a virtual port.
    fn detach(&self, port: u32) -> Result<()>;
}

/// Creates a connected pair of loopback TCP sockets.
///
/// The accepted connection is checked against the connecting socket's
/// address, so another local process cannot slip in between.
pub fn loopback_pair() -> io::Result<(TcpStream, TcpStream)> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let ours = TcpStream::connect(listener.local_addr()?)?;
    let expected = ours.local_addr()?;
    loop {
        let (theirs, from) = listener.accept()?;
        if from == expected {
            ours.set_nodelay(true)?;
            theirs.set_nodelay(true)?;
            return Ok((ours, theirs));
        }
    }
}

/// Converts a std socket to a tokio socket.
pub fn into_tokio(s: TcpStream) -> io::Result<tokio::net::TcpStream> {
    s.set_nonblocking(true)?;
    tokio::net::TcpStream::from_std(s)
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};

    #[test]
    fn loopback_pair_is_connected() {
        let (mut a, mut b) = super::loopback_pair().unwrap();
        a.write_all(b"ping").unwrap();
        let mut buf = [0u8; 4];
        b.read_exact(&mut buf).unwrap();
        assert_eq!(&buf, b"ping");
    }
}
