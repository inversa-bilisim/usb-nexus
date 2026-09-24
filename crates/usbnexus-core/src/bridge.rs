// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! Plain USB/IP endpoint for drivers that open their own TCP connection.
//!
//! Some USB/IP client drivers (usbip-win2 on Windows) connect to a server
//! themselves and perform the `OP_REQ_IMPORT` exchange before URB traffic.
//! We point such a driver at a loopback listener, answer the import with the
//! device we already imported through the secure tunnel, and then relay the
//! URB stream like any other backend socket.

use std::time::Duration;

use anyhow::{bail, Context, Result};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use usbnexus_proto::op::{OpMessage, OP_HEADER_SIZE, ST_NA, ST_OK};
use usbnexus_proto::{DeviceInfo, BUSID_SIZE};

const IMPORT_TIMEOUT: Duration = Duration::from_secs(15);

/// Reads the driver's `OP_REQ_IMPORT` and answers it with `device`.
pub async fn answer_import(stream: &mut TcpStream, device: &DeviceInfo) -> Result<()> {
    let mut req = [0u8; OP_HEADER_SIZE + BUSID_SIZE];
    tokio::time::timeout(IMPORT_TIMEOUT, stream.read_exact(&mut req))
        .await
        .context("driver did not send an import request")??;
    let (msg, _) = OpMessage::decode(&req).context("decoding import request")?;
    let OpMessage::ReqImport { busid } = msg else {
        bail!("driver sent an unexpected request");
    };
    let reply = if busid == device.busid {
        let mut d = device.clone();
        // The reply carries no interface records.
        d.interfaces.clear();
        OpMessage::RepImport { status: ST_OK, device: Some(d) }
    } else {
        OpMessage::RepImport { status: ST_NA, device: None }
    };
    stream.write_all(&reply.encode()?).await?;
    stream.flush().await?;
    if busid != device.busid {
        bail!("driver asked for {busid}, expected {}", device.busid);
    }
    Ok(())
}

/// Accepts connections on `listener` until one passes `accept_peer` and
/// completes the import handshake; returns that connection.
pub async fn serve_one_import(
    listener: TcpListener,
    device: &DeviceInfo,
    accept_peer: impl Fn(&TcpStream) -> bool,
) -> Result<TcpStream> {
    loop {
        let (mut stream, peer) = listener.accept().await?;
        if !peer.ip().is_loopback() || !accept_peer(&stream) {
            tracing::warn!(%peer, "rejected unexpected local connection");
            continue;
        }
        stream.set_nodelay(true)?;
        answer_import(&mut stream, device).await?;
        return Ok(stream);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use usbnexus_proto::{InterfaceInfo, Speed};

    fn device() -> DeviceInfo {
        DeviceInfo {
            path: "/sys/devices/x/1-1".into(),
            busid: "1-1".into(),
            busnum: 1,
            devnum: 7,
            speed: Speed::High,
            id_vendor: 0x1234,
            id_product: 0xabcd,
            bcd_device: 0x0100,
            device_class: 0,
            device_subclass: 0,
            device_protocol: 0,
            configuration_value: 1,
            num_configurations: 1,
            interfaces: vec![InterfaceInfo::default()],
        }
    }

    /// Plays the driver's side of the handshake.
    async fn driver(addr: std::net::SocketAddr, busid: &str) -> (TcpStream, OpMessage) {
        let mut s = TcpStream::connect(addr).await.unwrap();
        s.write_all(&OpMessage::ReqImport { busid: busid.into() }.encode().unwrap()).await.unwrap();
        let mut buf = vec![0u8; 1024];
        let mut have = 0;
        loop {
            let n = s.read(&mut buf[have..]).await.unwrap();
            assert!(n > 0, "server closed early");
            have += n;
            if let Ok((m, _)) = OpMessage::decode(&buf[..have]) {
                return (s, m);
            }
        }
    }

    #[tokio::test]
    async fn answers_import_then_relays() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let dev = device();
        let server = tokio::spawn(async move { serve_one_import(listener, &dev, |_| true).await });
        let (mut drv, reply) = driver(addr, "1-1").await;
        match reply {
            OpMessage::RepImport { status: ST_OK, device: Some(d) } => {
                assert_eq!(d.devid(), (1 << 16) | 7);
                assert_eq!((d.id_vendor, d.id_product), (0x1234, 0xabcd));
            }
            other => panic!("unexpected reply {other:?}"),
        }
        // After the handshake the connection carries raw URB bytes.
        let mut ours = server.await.unwrap().unwrap();
        drv.write_all(b"urb").await.unwrap();
        let mut buf = [0u8; 3];
        ours.read_exact(&mut buf).await.unwrap();
        assert_eq!(&buf, b"urb");
    }

    #[tokio::test]
    async fn wrong_busid_is_refused() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let dev = device();
        let server = tokio::spawn(async move { serve_one_import(listener, &dev, |_| true).await });
        let (_drv, reply) = driver(addr, "9-9").await;
        assert_eq!(reply, OpMessage::RepImport { status: ST_NA, device: None });
        assert!(server.await.unwrap().is_err());
    }

    #[tokio::test]
    async fn rejected_peers_are_skipped() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let dev = device();
        let calls = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));
        let c = calls.clone();
        // Reject the first connection (an intruder), accept the second.
        let server = tokio::spawn(async move {
            serve_one_import(listener, &dev, move |_| c.fetch_add(1, std::sync::atomic::Ordering::SeqCst) > 0).await
        });
        let intruder = TcpStream::connect(addr).await.unwrap();
        let (_drv, reply) = driver(addr, "1-1").await;
        assert!(matches!(reply, OpMessage::RepImport { status: ST_OK, .. }));
        assert!(server.await.unwrap().is_ok());
        drop(intruder);
    }
}
