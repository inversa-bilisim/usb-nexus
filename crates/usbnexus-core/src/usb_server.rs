// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! Userspace USB/IP device server.
//!
//! On platforms without an in-kernel USB/IP stub driver (Windows, macOS) the
//! URB phase is handled here: `CMD_SUBMIT` / `CMD_UNLINK` messages are turned
//! into calls on a [`UsbDevice`] and the results are sent back as
//! `RET_SUBMIT` / `RET_UNLINK`, following the USB/IP protocol specification:
//!
//! * Many URBs may be in flight at once. Replies for the same endpoint are
//!   sent in request order; different endpoints do not wait for each other.
//! * `CMD_UNLINK` races with completion. If the unlink is processed first the
//!   URB counts as cancelled: its `RET_SUBMIT` is never sent and the unlink
//!   reply carries `-ECONNRESET`. Otherwise the unlink reply carries 0.
//! * Requests that change the device state (SET_CONFIGURATION,
//!   SET_INTERFACE, CLEAR_FEATURE(ENDPOINT_HALT)) are handed to dedicated
//!   device operations and completed before the next request is read.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

use anyhow::{bail, Context, Result};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::sync::{mpsc, oneshot};
use usbnexus_proto::urb::{
    BasicHeader, IsoPacket, RetSubmit, RetUnlink, UrbMessage, CMD_SUBMIT, CMD_UNLINK, DIR_IN, DIR_OUT, ISO_DESC_SIZE,
    MAX_ISO_PACKETS, MAX_TRANSFER, RET_SUBMIT, RET_UNLINK, URB_HEADER_SIZE,
};

use crate::backend::BoxFuture;

/// Linux errno values used in USB/IP status fields (sent negated).
pub mod errno {
    pub const ENOENT: i32 = 2;
    pub const EPIPE: i32 = 32;
    pub const EINVAL: i32 = 22;
    pub const ETIME: i32 = 62;
    pub const EPROTO: i32 = 71;
    pub const EOVERFLOW: i32 = 75;
    pub const EILSEQ: i32 = 84;
    pub const ECONNRESET: i32 = 104;
    pub const ESHUTDOWN: i32 = 108;
    pub const EREMOTEIO: i32 = 121;
}

/// `URB_SHORT_NOT_OK` in `transfer_flags`.
const URB_SHORT_NOT_OK: u32 = 0x0001;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferKind {
    Control,
    Isochronous,
    Bulk,
    Interrupt,
}

/// One transfer handed to the device.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transfer {
    /// Endpoint number (0..=15).
    pub endpoint: u8,
    pub dir_in: bool,
    pub kind: TransferKind,
    /// Setup packet; meaningful for control transfers only.
    pub setup: [u8; 8],
    /// OUT: the data to send. IN: a zeroed buffer of the requested length
    /// (isochronous packets laid out back to back in request order).
    pub buffer: Vec<u8>,
    /// Whether a short IN transfer is acceptable.
    pub short_ok: bool,
    /// Requested length of each isochronous packet.
    pub iso_lengths: Vec<u32>,
}

impl Transfer {
    /// Endpoint address as used in USB descriptors (bit 7 = IN).
    pub fn address(&self) -> u8 {
        endpoint_address(self.endpoint, self.dir_in)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct IsoResult {
    pub actual_length: u32,
    /// 0 or a negative errno.
    pub status: i32,
}

/// Result of a [`Transfer`].
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Completion {
    /// 0 or a negative errno.
    pub status: i32,
    /// IN: the transfer buffer (same layout as requested). OUT: unused.
    pub buffer: Vec<u8>,
    /// Bytes transferred (non-isochronous transfers).
    pub actual_length: u32,
    /// Per-packet results (isochronous transfers).
    pub iso: Vec<IsoResult>,
}

impl Completion {
    pub fn error(status: i32) -> Self {
        Completion { status, ..Default::default() }
    }
}

/// A USB device driven from userspace.
pub trait UsbDevice: Send + Sync + 'static {
    /// Starts a transfer; the future resolves when it completes (possibly
    /// much later, e.g. an interrupt IN endpoint waiting for input).
    fn submit(&self, transfer: Transfer) -> BoxFuture<'static, Completion>;
    fn set_configuration(&self, value: u8) -> BoxFuture<'_, Result<()>>;
    fn set_interface(&self, interface: u8, alternate: u8) -> BoxFuture<'_, Result<()>>;
    /// Clears a halt (stall) condition on an endpoint address.
    fn clear_halt(&self, endpoint: u8) -> BoxFuture<'_, Result<()>>;
    /// Cancels every transfer pending on an endpoint address. Aborted
    /// transfers complete with an error status.
    fn abort_endpoint(&self, endpoint: u8) -> BoxFuture<'_, Result<()>>;
}

fn endpoint_address(number: u8, dir_in: bool) -> u8 {
    if dir_in {
        number | 0x80
    } else {
        number
    }
}

/// Queue key: the control endpoint is bidirectional and ordered as one.
fn queue_key(number: u8, dir_in: bool) -> u8 {
    if number == 0 {
        0
    } else {
        endpoint_address(number, dir_in)
    }
}

enum Out {
    Submit { seqnum: u32, bytes: Vec<u8> },
    Unlink { seqnum: u32, target: u32 },
}

struct Endpoint {
    /// Replies in request order.
    queue: mpsc::UnboundedSender<oneshot::Receiver<(u32, Vec<u8>)>>,
    /// Transfers submitted to the device and not yet completed.
    in_flight: Arc<AtomicU32>,
    /// Unlinks received since the last abort.
    unlinks: u32,
    address: u8,
}

/// Reads the rest of a `CMD_SUBMIT` whose 48-byte header is in `header`.
async fn read_submit_body<R: AsyncRead + Unpin>(r: &mut R, header: &[u8; URB_HEADER_SIZE]) -> Result<Vec<u8>> {
    let word = |o: usize| u32::from_be_bytes(header[o..o + 4].try_into().unwrap());
    let direction = word(12);
    let length = word(24) as i32;
    let packets = word(32) as i32;
    if length < 0 || length as usize > MAX_TRANSFER {
        bail!("transfer length {length} out of range");
    }
    let packets = if packets > 0 { packets as usize } else { 0 };
    if packets > MAX_ISO_PACKETS {
        bail!("too many isochronous packets ({packets})");
    }
    let data = if direction == DIR_OUT { length as usize } else { 0 };
    let mut msg = header.to_vec();
    msg.resize(URB_HEADER_SIZE + data + packets * ISO_DESC_SIZE, 0);
    r.read_exact(&mut msg[URB_HEADER_SIZE..]).await?;
    Ok(msg)
}

fn ret_header(command: u32, seqnum: u32, direction: u32) -> BasicHeader {
    BasicHeader { command, seqnum, devid: 0, direction, ep: 0 }
}

/// Encodes a reply. The basic header's devid/direction/ep are 0 on the wire
/// (the direction is only used to decide whether data follows).
fn encode_reply(m: &UrbMessage) -> Vec<u8> {
    let mut bytes = m.encode();
    bytes[8..20].fill(0);
    bytes
}

/// Hosts remote wakeup is not forwarded over USB/IP; hide it so the client
/// does not rely on it.
fn mask_remote_wakeup(setup: &[u8; 8], data: &mut [u8]) {
    let get_config_descriptor = setup[0] == 0x80 && setup[1] == 0x06 && setup[3] == 0x02;
    if get_config_descriptor && data.len() >= 8 && data[1] == 0x02 {
        data[7] &= !0x20;
    }
}

/// What a reply needs to know about its request.
struct Origin {
    seqnum: u32,
    dir_in: bool,
    number_of_packets: i32,
    start_frame: i32,
    iso_lengths: Vec<u32>,
    setup: [u8; 8],
    endpoint: u8,
}

fn build_ret_submit(o: &Origin, c: Completion) -> Vec<u8> {
    let Origin { seqnum, dir_in, number_of_packets, start_frame, ref iso_lengths, ref setup, endpoint } = *o;
    let direction = if dir_in { DIR_IN } else { DIR_OUT };
    if iso_lengths.is_empty() {
        let actual = (c.actual_length as usize).min(c.buffer.len());
        let mut data = if dir_in { c.buffer[..actual].to_vec() } else { vec![] };
        if endpoint == 0 && dir_in {
            mask_remote_wakeup(setup, &mut data);
        }
        return encode_reply(&UrbMessage::RetSubmit(RetSubmit {
            header: ret_header(RET_SUBMIT, seqnum, direction),
            status: c.status,
            actual_length: if dir_in { data.len() as i32 } else { c.actual_length as i32 },
            start_frame: 0,
            number_of_packets,
            error_count: 0,
            data,
            iso: vec![],
        }));
    }

    // Isochronous: IN data is sent compacted, packet by packet.
    let mut data = Vec::new();
    let mut iso = Vec::with_capacity(iso_lengths.len());
    let mut offset = 0usize;
    let mut total = 0u32;
    let mut errors = 0;
    for (i, &len) in iso_lengths.iter().enumerate() {
        let r = c.iso.get(i).copied().unwrap_or(IsoResult { actual_length: 0, status: -errno::EPROTO });
        let actual = r.actual_length.min(len);
        if dir_in {
            let end = (offset + actual as usize).min(c.buffer.len());
            data.extend_from_slice(&c.buffer[offset.min(end)..end]);
        }
        if r.status != 0 {
            errors += 1;
        }
        iso.push(IsoPacket { offset: offset as u32, length: len, actual_length: actual, status: r.status as u32 });
        offset += len as usize;
        total += actual;
    }
    encode_reply(&UrbMessage::RetSubmit(RetSubmit {
        header: ret_header(RET_SUBMIT, seqnum, direction),
        status: c.status,
        actual_length: total as i32,
        start_frame,
        number_of_packets,
        error_count: errors,
        data,
        iso,
    }))
}

/// Serves the URB phase for `device` on `stream` until the peer disconnects.
/// Outstanding transfers are aborted before returning.
pub async fn serve<S>(stream: S, device: Arc<dyn UsbDevice>) -> Result<()>
where
    S: AsyncRead + AsyncWrite + Send + 'static,
{
    let (mut rd, mut wr) = tokio::io::split(stream);
    // seqnum -> queue key of submits whose reply has not been sent.
    let pending: Arc<Mutex<HashMap<u32, u8>>> = Arc::default();
    let (out_tx, mut out_rx) = mpsc::unbounded_channel::<Out>();

    let writer_pending = pending.clone();
    let writer = tokio::spawn(async move {
        while let Some(out) = out_rx.recv().await {
            let bytes = match out {
                Out::Submit { seqnum, bytes } => {
                    // Dropped if it was unlinked first.
                    if writer_pending.lock().unwrap().remove(&seqnum).is_none() {
                        continue;
                    }
                    bytes
                }
                Out::Unlink { seqnum, target } => {
                    let won = writer_pending.lock().unwrap().remove(&target).is_some();
                    encode_reply(&UrbMessage::RetUnlink(RetUnlink {
                        header: ret_header(RET_UNLINK, seqnum, 0),
                        status: if won { -errno::ECONNRESET } else { 0 },
                    }))
                }
            };
            if wr.write_all(&bytes).await.is_err() {
                break;
            }
            let _ = wr.flush().await;
        }
    });

    let mut endpoints: HashMap<u8, Endpoint> = HashMap::new();
    let result = read_loop(&mut rd, &device, &pending, &out_tx, &mut endpoints).await;

    // The client is gone: cancel whatever is still running on the device.
    for ep in endpoints.values() {
        if ep.in_flight.load(Ordering::SeqCst) > 0 {
            let _ = device.abort_endpoint(ep.address).await;
        }
    }
    drop(endpoints);
    drop(out_tx);
    let _ = writer.await;
    result
}

async fn read_loop<R: AsyncRead + Unpin>(
    rd: &mut R,
    device: &Arc<dyn UsbDevice>,
    pending: &Arc<Mutex<HashMap<u32, u8>>>,
    out_tx: &mpsc::UnboundedSender<Out>,
    endpoints: &mut HashMap<u8, Endpoint>,
) -> Result<()> {
    loop {
        let mut header = [0u8; URB_HEADER_SIZE];
        match rd.read_exact(&mut header).await {
            Ok(_) => {}
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(()),
            Err(e) => return Err(e.into()),
        }
        let basic = BasicHeader::peek(&header)?;
        match basic.command {
            CMD_SUBMIT => {
                let msg = read_submit_body(rd, &header).await?;
                let (UrbMessage::CmdSubmit(cmd), _) = UrbMessage::decode(&msg, None).context("decoding CMD_SUBMIT")?
                else {
                    unreachable!("command checked above");
                };
                let number = (cmd.header.ep & 0x0f) as u8;
                let dir_in = cmd.header.direction == DIR_IN;
                let key = queue_key(number, dir_in);
                if pending.lock().unwrap().insert(cmd.header.seqnum, key).is_some() {
                    bail!("duplicate sequence number {}", cmd.header.seqnum);
                }
                let ep = endpoints.entry(key).or_insert_with(|| {
                    let (tx, mut rx) = mpsc::unbounded_channel::<oneshot::Receiver<(u32, Vec<u8>)>>();
                    let out = out_tx.clone();
                    tokio::spawn(async move {
                        while let Some(reply) = rx.recv().await {
                            if let Ok((seqnum, bytes)) = reply.await {
                                if out.send(Out::Submit { seqnum, bytes }).is_err() {
                                    break;
                                }
                            }
                        }
                    });
                    Endpoint {
                        queue: tx,
                        in_flight: Arc::new(AtomicU32::new(0)),
                        unlinks: 0,
                        address: endpoint_address(number, dir_in),
                    }
                });
                let (reply_tx, reply_rx) = oneshot::channel();
                let _ = ep.queue.send(reply_rx);
                submit(device, cmd, number, dir_in, ep.in_flight.clone(), reply_tx).await;
            }
            CMD_UNLINK => {
                let (UrbMessage::CmdUnlink(cmd), _) = UrbMessage::decode(&header, None)? else {
                    unreachable!("command checked above");
                };
                let target_key = pending.lock().unwrap().get(&cmd.unlink_seqnum).copied();
                let _ = out_tx.send(Out::Unlink { seqnum: cmd.header.seqnum, target: cmd.unlink_seqnum });
                // Devices can only abort whole endpoints. Clients usually unlink
                // every URB of an endpoint in a row, so abort once the last
                // outstanding one is unlinked rather than on the first.
                if let Some(ep) = target_key.and_then(|k| endpoints.get_mut(&k)) {
                    ep.unlinks += 1;
                    let in_flight = ep.in_flight.load(Ordering::SeqCst);
                    if in_flight > 0 && ep.unlinks >= in_flight {
                        ep.unlinks = 0;
                        if let Err(e) = device.abort_endpoint(ep.address).await {
                            tracing::debug!("aborting endpoint {:#04x} failed: {e:#}", ep.address);
                        }
                    }
                }
            }
            other => bail!("unexpected USB/IP command {other:#x}"),
        }
    }
}

/// Handles one `CMD_SUBMIT`; sends its reply through `reply`.
async fn submit(
    device: &Arc<dyn UsbDevice>,
    cmd: usbnexus_proto::urb::CmdSubmit,
    number: u8,
    dir_in: bool,
    in_flight: Arc<AtomicU32>,
    reply: oneshot::Sender<(u32, Vec<u8>)>,
) {
    let seqnum = cmd.header.seqnum;
    let setup = cmd.setup;
    let iso_lengths: Vec<u32> = cmd.iso.iter().map(|p| p.length).collect();
    let kind = if number == 0 {
        TransferKind::Control
    } else if cmd.number_of_packets > 0 {
        TransferKind::Isochronous
    } else if cmd.interval == 0 {
        TransferKind::Bulk
    } else {
        TransferKind::Interrupt
    };
    let origin = Origin {
        seqnum,
        dir_in,
        number_of_packets: cmd.number_of_packets,
        start_frame: cmd.start_frame,
        iso_lengths,
        setup,
        endpoint: number,
    };
    let finish = move |c: Completion| build_ret_submit(&origin, c);

    if kind == TransferKind::Isochronous
        && cmd.iso.iter().map(|p| p.length as u64).sum::<u64>() != cmd.transfer_buffer_length as u64
    {
        let _ = reply.send((seqnum, finish(Completion::error(-errno::EINVAL))));
        return;
    }

    // Standard requests that change device state go to dedicated operations,
    // completed before the next request is read.
    if kind == TransferKind::Control && !dir_in {
        let (request_type, request) = (setup[0], setup[1]);
        let value = u16::from_le_bytes([setup[2], setup[3]]);
        let index = u16::from_le_bytes([setup[4], setup[5]]);
        let special = match (request_type, request) {
            (0x00, 0x09) => Some(device.set_configuration(value as u8)),
            (0x01, 0x0b) => Some(device.set_interface(index as u8, value as u8)),
            (0x02, 0x01) if value == 0 => Some(device.clear_halt(index as u8)),
            _ => None,
        };
        if let Some(op) = special {
            let status = match op.await {
                Ok(()) => 0,
                Err(e) => {
                    tracing::debug!("control request {request:#04x} failed: {e:#}");
                    -errno::EPIPE
                }
            };
            let _ = reply.send((seqnum, finish(Completion { status, ..Default::default() })));
            return;
        }
    }

    let buffer = if dir_in { vec![0u8; cmd.transfer_buffer_length.max(0) as usize] } else { cmd.data };
    let transfer = Transfer {
        endpoint: number,
        dir_in,
        kind,
        setup,
        buffer,
        short_ok: dir_in && cmd.transfer_flags & URB_SHORT_NOT_OK == 0,
        iso_lengths: cmd.iso.iter().map(|p| p.length).collect(),
    };
    in_flight.fetch_add(1, Ordering::SeqCst);
    let fut = device.submit(transfer);
    tokio::spawn(async move {
        let completion = fut.await;
        in_flight.fetch_sub(1, Ordering::SeqCst);
        let _ = reply.send((seqnum, finish(completion)));
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::sync::Notify;
    use usbnexus_proto::urb::{CmdSubmit, CmdUnlink};

    /// A fake device: control IN returns a configuration descriptor, bulk
    /// OUT/IN on endpoint 1 echo, interrupt IN on endpoint 3 never completes
    /// until aborted, endpoint 2 IN completes after a delay given in setup[0].
    #[derive(Default)]
    struct Mock {
        calls: Mutex<Vec<String>>,
        stored: Arc<Mutex<Vec<u8>>>,
        aborted: Arc<Notify>,
    }

    impl UsbDevice for Mock {
        fn submit(&self, t: Transfer) -> BoxFuture<'static, Completion> {
            self.calls.lock().unwrap().push(format!("submit {:#04x} {:?}", t.address(), t.kind));
            let stored = self.stored.clone();
            let aborted = self.aborted.clone();
            Box::pin(async move {
                match (t.endpoint, t.dir_in, t.kind) {
                    (0, true, _) => {
                        let desc = [9u8, 2, 32, 0, 1, 1, 0, 0xa0, 50];
                        let mut buf = t.buffer;
                        let n = desc.len().min(buf.len());
                        buf[..n].copy_from_slice(&desc[..n]);
                        Completion { status: 0, buffer: buf, actual_length: n as u32, iso: vec![] }
                    }
                    (1, false, _) => {
                        *stored.lock().unwrap() = t.buffer.clone();
                        Completion { status: 0, buffer: vec![], actual_length: t.buffer.len() as u32, iso: vec![] }
                    }
                    (1, true, _) => {
                        let data = stored.lock().unwrap().clone();
                        let n = data.len().min(t.buffer.len());
                        let mut buf = t.buffer;
                        buf[..n].copy_from_slice(&data[..n]);
                        Completion { status: 0, buffer: buf, actual_length: n as u32, iso: vec![] }
                    }
                    (2, true, _) => {
                        tokio::time::sleep(Duration::from_millis(t.buffer.len() as u64)).await;
                        let len = t.buffer.len() as u32;
                        Completion { status: 0, buffer: t.buffer, actual_length: len, iso: vec![] }
                    }
                    (4, true, TransferKind::Isochronous) => {
                        // Each packet delivers half of what was asked.
                        let mut buf = t.buffer;
                        let mut off = 0;
                        let mut iso = vec![];
                        for (i, &len) in t.iso_lengths.iter().enumerate() {
                            let half = len / 2;
                            buf[off..off + half as usize].fill(i as u8 + 1);
                            off += len as usize;
                            iso.push(IsoResult { actual_length: half, status: 0 });
                        }
                        Completion { status: 0, buffer: buf, actual_length: 0, iso }
                    }
                    _ => {
                        aborted.notified().await;
                        Completion::error(-errno::ENOENT)
                    }
                }
            })
        }

        fn set_configuration(&self, value: u8) -> BoxFuture<'_, Result<()>> {
            self.calls.lock().unwrap().push(format!("set_configuration {value}"));
            Box::pin(async { Ok(()) })
        }

        fn set_interface(&self, interface: u8, alternate: u8) -> BoxFuture<'_, Result<()>> {
            self.calls.lock().unwrap().push(format!("set_interface {interface} {alternate}"));
            Box::pin(async { Ok(()) })
        }

        fn clear_halt(&self, endpoint: u8) -> BoxFuture<'_, Result<()>> {
            self.calls.lock().unwrap().push(format!("clear_halt {endpoint:#04x}"));
            Box::pin(async { Ok(()) })
        }

        fn abort_endpoint(&self, endpoint: u8) -> BoxFuture<'_, Result<()>> {
            self.calls.lock().unwrap().push(format!("abort {endpoint:#04x}"));
            self.aborted.notify_waiters();
            Box::pin(async { Ok(()) })
        }
    }

    fn submit_msg(seq: u32, ep: u32, dir: u32, len: i32, interval: i32, setup: [u8; 8], data: Vec<u8>) -> Vec<u8> {
        UrbMessage::CmdSubmit(CmdSubmit {
            header: BasicHeader { command: CMD_SUBMIT, seqnum: seq, devid: 0x10002, direction: dir, ep },
            transfer_flags: 0,
            transfer_buffer_length: len,
            start_frame: 0,
            number_of_packets: 0,
            interval,
            setup,
            data,
            iso: vec![],
        })
        .encode()
    }

    struct Client {
        io: tokio::io::DuplexStream,
        dirs: HashMap<u32, u32>,
    }

    impl Client {
        async fn send(&mut self, msg: Vec<u8>) {
            let h = BasicHeader::peek(&msg).unwrap();
            self.dirs.insert(h.seqnum, h.direction);
            self.io.write_all(&msg).await.unwrap();
        }

        async fn recv(&mut self) -> UrbMessage {
            let mut header = [0u8; URB_HEADER_SIZE];
            tokio::time::timeout(Duration::from_secs(5), self.io.read_exact(&mut header)).await.unwrap().unwrap();
            let h = BasicHeader::peek(&header).unwrap();
            assert_eq!((h.devid, h.direction, h.ep), (0, 0, 0), "reply header fields must be zero");
            let mut msg = header.to_vec();
            if h.command == RET_SUBMIT {
                let word = |o: usize| i32::from_be_bytes(header[o..o + 4].try_into().unwrap());
                let dir = self.dirs[&h.seqnum];
                let data = if dir == DIR_IN { word(24) as usize } else { 0 };
                let packets = word(32).max(0) as usize;
                msg.resize(URB_HEADER_SIZE + data + packets * ISO_DESC_SIZE, 0);
                self.io.read_exact(&mut msg[URB_HEADER_SIZE..]).await.unwrap();
                return UrbMessage::decode(&msg, Some(dir)).unwrap().0;
            }
            UrbMessage::decode(&msg, None).unwrap().0
        }
    }

    fn start() -> (Client, Arc<Mock>, tokio::task::JoinHandle<Result<()>>) {
        let (a, b) = tokio::io::duplex(1 << 20);
        let mock = Arc::new(Mock::default());
        let dev: Arc<dyn UsbDevice> = mock.clone();
        let task = tokio::spawn(serve(b, dev));
        (Client { io: a, dirs: HashMap::new() }, mock, task)
    }

    fn ret(m: UrbMessage) -> RetSubmit {
        match m {
            UrbMessage::RetSubmit(r) => r,
            other => panic!("expected RET_SUBMIT, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn control_and_bulk_round_trip() {
        let (mut c, mock, task) = start();
        // GET_DESCRIPTOR(configuration); remote wakeup (0x20) must be masked.
        c.send(submit_msg(1, 0, DIR_IN, 64, 0, [0x80, 6, 0, 2, 0, 0, 64, 0], vec![])).await;
        let r = ret(c.recv().await);
        assert_eq!((r.header.seqnum, r.status, r.actual_length), (1, 0, 9));
        assert_eq!(r.data[7], 0x80, "remote wakeup bit cleared");

        c.send(submit_msg(2, 1, DIR_OUT, 3, 0, [0; 8], b"abc".to_vec())).await;
        let r = ret(c.recv().await);
        assert_eq!((r.header.seqnum, r.actual_length), (2, 3));
        assert!(r.data.is_empty());

        c.send(submit_msg(3, 1, DIR_IN, 512, 0, [0; 8], vec![])).await;
        let r = ret(c.recv().await);
        assert_eq!(r.data, b"abc");

        assert!(mock.calls.lock().unwrap().contains(&"submit 0x01 Bulk".to_string()));
        drop(c);
        task.await.unwrap().unwrap();
    }

    #[tokio::test]
    async fn state_changing_requests_use_dedicated_operations() {
        let (mut c, mock, _task) = start();
        c.send(submit_msg(1, 0, DIR_OUT, 0, 0, [0x00, 0x09, 2, 0, 0, 0, 0, 0], vec![])).await;
        c.send(submit_msg(2, 0, DIR_OUT, 0, 0, [0x01, 0x0b, 1, 0, 3, 0, 0, 0], vec![])).await;
        c.send(submit_msg(3, 0, DIR_OUT, 0, 0, [0x02, 0x01, 0, 0, 0x81, 0, 0, 0], vec![])).await;
        for seq in 1..=3 {
            let r = ret(c.recv().await);
            assert_eq!((r.header.seqnum, r.status), (seq, 0));
        }
        let calls = mock.calls.lock().unwrap().clone();
        assert_eq!(calls, ["set_configuration 2", "set_interface 3 1", "clear_halt 0x81"]);
    }

    #[tokio::test]
    async fn replies_keep_order_per_endpoint() {
        let (mut c, _mock, _task) = start();
        // Endpoint 2 IN sleeps for len ms: the first request finishes last.
        c.send(submit_msg(10, 2, DIR_IN, 150, 0, [0; 8], vec![])).await;
        c.send(submit_msg(11, 2, DIR_IN, 1, 0, [0; 8], vec![])).await;
        // A different endpoint is not held up by endpoint 2.
        c.send(submit_msg(12, 1, DIR_OUT, 1, 0, [0; 8], vec![7])).await;
        let order: Vec<u32> =
            [c.recv().await, c.recv().await, c.recv().await].into_iter().map(|m| ret(m).header.seqnum).collect();
        assert_eq!(order, [12, 10, 11]);
    }

    #[tokio::test]
    async fn unlink_before_completion_cancels() {
        let (mut c, mock, _task) = start();
        c.send(submit_msg(1, 3, DIR_IN, 8, 1, [0; 8], vec![])).await;
        c.send(submit_msg(2, 3, DIR_IN, 8, 1, [0; 8], vec![])).await;
        for (seq, target) in [(3, 1), (4, 2)] {
            c.send(
                UrbMessage::CmdUnlink(CmdUnlink {
                    header: BasicHeader { command: CMD_UNLINK, seqnum: seq, devid: 0, direction: 0, ep: 0 },
                    unlink_seqnum: target,
                })
                .encode(),
            )
            .await;
        }
        for seq in [3, 4] {
            match c.recv().await {
                UrbMessage::RetUnlink(u) => assert_eq!((u.header.seqnum, u.status), (seq, -errno::ECONNRESET)),
                other => panic!("{other:?}"),
            }
        }
        // Only one abort, after the last outstanding URB was unlinked, and
        // the cancelled URBs never get a RET_SUBMIT.
        tokio::time::sleep(Duration::from_millis(50)).await;
        let calls = mock.calls.lock().unwrap().clone();
        assert_eq!(calls.iter().filter(|c| c.starts_with("abort")).count(), 1);
        c.send(submit_msg(5, 1, DIR_OUT, 1, 0, [0; 8], vec![1])).await;
        assert_eq!(ret(c.recv().await).header.seqnum, 5);
    }

    #[tokio::test]
    async fn unlink_after_completion_is_too_late() {
        let (mut c, mock, _task) = start();
        c.send(submit_msg(1, 1, DIR_OUT, 1, 0, [0; 8], vec![9])).await;
        assert_eq!(ret(c.recv().await).header.seqnum, 1);
        c.send(
            UrbMessage::CmdUnlink(CmdUnlink {
                header: BasicHeader { command: CMD_UNLINK, seqnum: 2, devid: 0, direction: 0, ep: 0 },
                unlink_seqnum: 1,
            })
            .encode(),
        )
        .await;
        match c.recv().await {
            UrbMessage::RetUnlink(u) => assert_eq!((u.header.seqnum, u.status), (2, 0)),
            other => panic!("{other:?}"),
        }
        assert!(!mock.calls.lock().unwrap().iter().any(|c| c.starts_with("abort")));
    }

    #[tokio::test]
    async fn isochronous_in_is_compacted() {
        let (mut c, _mock, _task) = start();
        let msg = UrbMessage::CmdSubmit(CmdSubmit {
            header: BasicHeader { command: CMD_SUBMIT, seqnum: 7, devid: 0, direction: DIR_IN, ep: 4 },
            transfer_flags: 0,
            transfer_buffer_length: 12,
            start_frame: 100,
            number_of_packets: 3,
            interval: 1,
            setup: [0; 8],
            data: vec![],
            iso: [4u32, 4, 4].iter().map(|&l| IsoPacket { length: l, ..Default::default() }).collect(),
        })
        .encode();
        c.send(msg).await;
        let r = ret(c.recv().await);
        assert_eq!(r.actual_length, 6);
        assert_eq!(r.data, [1, 1, 2, 2, 3, 3]);
        assert_eq!(r.iso.iter().map(|p| (p.offset, p.actual_length)).collect::<Vec<_>>(), [(0, 2), (4, 2), (8, 2)]);
        assert_eq!(r.start_frame, 100);
    }

    #[tokio::test]
    async fn disconnect_aborts_outstanding_transfers() {
        let (mut c, mock, task) = start();
        c.send(submit_msg(1, 3, DIR_IN, 8, 1, [0; 8], vec![])).await;
        tokio::time::sleep(Duration::from_millis(20)).await;
        drop(c);
        tokio::time::timeout(Duration::from_secs(5), task).await.unwrap().unwrap().unwrap();
        assert!(mock.calls.lock().unwrap().contains(&"abort 0x83".to_string()));
    }

    #[tokio::test]
    async fn malformed_input_ends_the_session() {
        let (mut c, _mock, task) = start();
        let mut bad = submit_msg(1, 1, DIR_OUT, 0, 0, [0; 8], vec![]);
        bad[24..28].copy_from_slice(&i32::MAX.to_be_bytes());
        c.io.write_all(&bad).await.unwrap();
        assert!(tokio::time::timeout(Duration::from_secs(5), task).await.unwrap().unwrap().is_err());
    }
}
