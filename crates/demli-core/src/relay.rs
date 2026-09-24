// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Demli contributors

//! Bidirectional byte relay between the TLS tunnel and the local USB/IP socket.

use std::io;

use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt, BufReader};

const BUF_SIZE: usize = 64 * 1024;

/// Which side ended the relay first.
#[derive(Debug)]
pub enum RelayEnd {
    /// The remote peer closed the tunnel or the network failed.
    Remote(Option<io::Error>),
    /// The local USB/IP stack closed its socket (device detached or unplugged).
    Local(Option<io::Error>),
}

impl RelayEnd {
    pub fn is_remote(&self) -> bool {
        matches!(self, RelayEnd::Remote(_))
    }
}

/// Copies bytes both ways until one side stops, then closes both.
pub async fn relay<R, L>(remote: R, local: L) -> RelayEnd
where
    R: AsyncRead + AsyncWrite + Unpin,
    L: AsyncRead + AsyncWrite + Unpin,
{
    let (rr, mut rw) = tokio::io::split(remote);
    let (lr, mut lw) = tokio::io::split(local);
    let mut rr = BufReader::with_capacity(BUF_SIZE, rr);
    let mut lr = BufReader::with_capacity(BUF_SIZE, lr);

    // tokio's copy flushes whenever the reader has no more data ready, which
    // keeps latency low for the small, interactive URB messages.
    let end = tokio::select! {
        r = tokio::io::copy_buf(&mut rr, &mut lw) => match r {
            // Reading the remote ended: remote closed. A write error here
            // means the local socket went away.
            Ok(_) => RelayEnd::Remote(None),
            Err(e) if is_local_write_error(&e) => RelayEnd::Local(Some(e)),
            Err(e) => RelayEnd::Remote(Some(e)),
        },
        r = tokio::io::copy_buf(&mut lr, &mut rw) => match r {
            Ok(_) => RelayEnd::Local(None),
            Err(e) if is_local_write_error(&e) => RelayEnd::Remote(Some(e)),
            Err(e) => RelayEnd::Local(Some(e)),
        },
    };
    let _ = rw.shutdown().await;
    let _ = lw.shutdown().await;
    end
}

/// Errors that typically come from writing to a peer that has gone away.
fn is_local_write_error(e: &io::Error) -> bool {
    matches!(e.kind(), io::ErrorKind::BrokenPipe | io::ErrorKind::ConnectionReset)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::AsyncReadExt;

    #[tokio::test]
    async fn relays_and_reports_side() {
        let (remote, mut remote_peer) = tokio::io::duplex(1024);
        let (local, mut local_peer) = tokio::io::duplex(1024);
        let task = tokio::spawn(relay(remote, local));

        remote_peer.write_all(b"urb-out").await.unwrap();
        let mut buf = [0u8; 7];
        local_peer.read_exact(&mut buf).await.unwrap();
        assert_eq!(&buf, b"urb-out");

        local_peer.write_all(b"urb-in").await.unwrap();
        let mut buf = [0u8; 6];
        remote_peer.read_exact(&mut buf).await.unwrap();
        assert_eq!(&buf, b"urb-in");

        drop(remote_peer);
        assert!(task.await.unwrap().is_remote());
    }

    #[tokio::test]
    async fn local_close_is_reported() {
        let (remote, _remote_peer) = tokio::io::duplex(1024);
        let (local, local_peer) = tokio::io::duplex(1024);
        let task = tokio::spawn(relay(remote, local));
        drop(local_peer);
        assert!(!task.await.unwrap().is_remote());
    }
}
