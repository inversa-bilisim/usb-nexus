// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

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

/// When data last went through a [`Watched`] stream, in either direction.
#[derive(Clone)]
pub struct Activity {
    start: std::time::Instant,
    last_ms: std::sync::Arc<std::sync::atomic::AtomicU64>,
}

impl Default for Activity {
    fn default() -> Self {
        Activity { start: std::time::Instant::now(), last_ms: Default::default() }
    }
}

impl Activity {
    fn touch(&self) {
        let ms = self.start.elapsed().as_millis() as u64;
        self.last_ms.store(ms, std::sync::atomic::Ordering::Relaxed);
    }

    /// Time since the last byte (or since the start).
    pub fn idle(&self) -> std::time::Duration {
        let last = self.last_ms.load(std::sync::atomic::Ordering::Relaxed);
        self.start.elapsed().saturating_sub(std::time::Duration::from_millis(last))
    }
}

/// A stream that records its traffic in an [`Activity`]. On a USB/IP
/// socket, a request the device has not answered yet (e.g. a pending
/// interrupt transfer) is no traffic, so an unused device counts as idle.
pub struct Watched<S> {
    inner: S,
    activity: Activity,
}

impl<S> Watched<S> {
    pub fn new(inner: S, activity: Activity) -> Self {
        Watched { inner, activity }
    }
}

impl<S: AsyncRead + Unpin> AsyncRead for Watched<S> {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        let before = buf.filled().len();
        let r = std::pin::Pin::new(&mut self.inner).poll_read(cx, buf);
        if matches!(r, std::task::Poll::Ready(Ok(()))) && buf.filled().len() > before {
            self.activity.touch();
        }
        r
    }
}

impl<S: AsyncWrite + Unpin> AsyncWrite for Watched<S> {
    fn poll_write(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        data: &[u8],
    ) -> std::task::Poll<io::Result<usize>> {
        let r = std::pin::Pin::new(&mut self.inner).poll_write(cx, data);
        if matches!(r, std::task::Poll::Ready(Ok(n)) if n > 0) {
            self.activity.touch();
        }
        r
    }

    fn poll_flush(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        std::pin::Pin::new(&mut self.inner).poll_flush(cx)
    }

    fn poll_shutdown(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        std::pin::Pin::new(&mut self.inner).poll_shutdown(cx)
    }
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
    async fn watched_streams_record_traffic() {
        let (a, mut b) = tokio::io::duplex(64);
        let activity = Activity::default();
        let mut w = Watched::new(a, activity.clone());
        tokio::time::sleep(std::time::Duration::from_millis(60)).await;
        assert!(activity.idle() >= std::time::Duration::from_millis(50));
        w.write_all(b"x").await.unwrap();
        assert!(activity.idle() < std::time::Duration::from_millis(50));
        tokio::time::sleep(std::time::Duration::from_millis(60)).await;
        b.write_all(b"y").await.unwrap();
        let mut buf = [0u8; 1];
        w.read_exact(&mut buf).await.unwrap();
        assert!(activity.idle() < std::time::Duration::from_millis(50));
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
