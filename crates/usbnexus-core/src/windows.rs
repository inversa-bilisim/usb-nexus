// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! Windows client backend on top of the usbip-win2 driver.
//!
//! usbip-win2 (BSD-2-Clause, attestation-signed UDE driver) is driven through
//! its `usbip.exe` tool. The driver opens its own TCP connection, so we point
//! it at a loopback listener (see [`crate::bridge`]) and relay the stream
//! over the TLS tunnel. `--once` disables the driver's own reconnect loop;
//! reconnecting is USB Nexus' job.

use std::os::windows::process::CommandExt;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::{bail, Context, Result};
use tokio::net::{TcpListener, TcpStream};
use usbnexus_proto::DeviceInfo;

use crate::api::ApiError;
use crate::backend::{BoxFuture, ImportBackend};
use crate::bridge::serve_one_import;

const CREATE_NO_WINDOW: u32 = windows_sys::Win32::System::Threading::CREATE_NO_WINDOW;
const ATTACH_TIMEOUT: Duration = Duration::from_secs(30);
/// Process id Windows reports for sockets opened by kernel drivers.
const SYSTEM_PID: u32 = 4;

pub struct WindowsImport {
    usbip: Option<PathBuf>,
}

impl Default for WindowsImport {
    fn default() -> Self {
        WindowsImport { usbip: std::env::var_os("USBNEXUS_USBIP").map(PathBuf::from) }
    }
}

impl WindowsImport {
    /// Locates `usbip.exe` from usbip-win2.
    pub fn usbip_exe(&self) -> Result<PathBuf> {
        if let Some(p) = &self.usbip {
            return Ok(p.clone());
        }
        let mut candidates = Vec::new();
        for var in ["ProgramFiles", "ProgramW6432"] {
            if let Some(dir) = std::env::var_os(var) {
                candidates.push(PathBuf::from(dir).join("USBip").join("usbip.exe"));
            }
        }
        if let Some(path) = std::env::var_os("PATH") {
            candidates.extend(std::env::split_paths(&path).map(|d| d.join("usbip.exe")));
        }
        candidates
            .into_iter()
            .find(|p| p.is_file())
            .ok_or_else(|| ApiError::new("driver_missing", "usbip-win2 is not installed (usbip.exe not found)").into())
    }

    async fn run(&self, args: &[&str]) -> Result<String> {
        let exe = self.usbip_exe()?;
        let out = tokio::process::Command::new(&exe)
            .args(args)
            .creation_flags(CREATE_NO_WINDOW)
            .kill_on_drop(true)
            .output()
            .await
            .with_context(|| format!("running {}", exe.display()))?;
        let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if !out.status.success() {
            let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
            bail!(
                "usbip.exe {} failed: {}",
                args.first().unwrap_or(&""),
                if stderr.is_empty() { stdout } else { stderr }
            );
        }
        Ok(stdout)
    }

    async fn attach_now(&self, device: &DeviceInfo) -> Result<(u32, TcpStream)> {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let local_port = listener.local_addr()?.port();
        let dev = device.clone();
        let accept = tokio::spawn(async move { serve_one_import(listener, &dev, is_kernel_peer).await });

        let port_arg = local_port.to_string();
        let args = ["-t", &port_arg, "attach", "-r", "127.0.0.1", "-b", &device.busid, "--once", "--terse"];
        let out = match tokio::time::timeout(ATTACH_TIMEOUT, self.run(&args)).await {
            Ok(Ok(out)) => out,
            Ok(Err(e)) => {
                accept.abort();
                // usbip-win2 before 0.9.7.6 rejects `--once`.
                if format!("{e:#}").contains("--once") {
                    return Err(ApiError::new("driver_outdated", format!("usbip-win2 is too old: {e:#}")).into());
                }
                return Err(e);
            }
            Err(_) => {
                accept.abort();
                bail!("usbip.exe attach timed out");
            }
        };
        let vport: u32 = out
            .lines()
            .last()
            .unwrap_or("")
            .trim()
            .parse()
            .with_context(|| format!("unexpected usbip.exe output: {out}"))?;
        let stream = tokio::time::timeout(Duration::from_secs(10), accept)
            .await
            .context("driver did not connect")?
            .context("import task failed")??;
        Ok((vport, stream))
    }
}

impl ImportBackend for WindowsImport {
    fn attach<'a>(&'a self, device: &'a DeviceInfo) -> BoxFuture<'a, Result<(u32, TcpStream)>> {
        Box::pin(self.attach_now(device))
    }

    fn detach(&self, port: u32) -> Result<()> {
        // Called from async code: start usbip.exe without waiting for it, so
        // a slow or stuck detach cannot block a runtime thread.
        let exe = self.usbip_exe()?;
        std::process::Command::new(exe)
            .args(["detach", "-p", &port.to_string()])
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
            .with_context(|| format!("running usbip.exe detach -p {port}"))?;
        Ok(())
    }
}

/// Whether the other end of `stream` is a kernel socket (the usbip-win2
/// driver) rather than some local program trying to grab the device.
fn is_kernel_peer(stream: &TcpStream) -> bool {
    let (Ok(peer), Ok(local)) = (stream.peer_addr(), stream.local_addr()) else { return false };
    match tcp_owner_pid(peer.port(), local.port()) {
        // PID 4 is "System"; 0 is reported for some kernel-owned sockets.
        Some(pid) => pid == SYSTEM_PID || pid == 0,
        None => {
            tracing::warn!(%peer, "could not determine the owner of the driver connection");
            false
        }
    }
}

const AF_INET: u32 = 2;
const AF_INET6: u32 = 23;

/// A TCP table from `GetExtendedTcpTable` (u32-aligned bytes).
fn tcp_table(family: u32, class: windows_sys::Win32::NetworkManagement::IpHelper::TCP_TABLE_CLASS) -> Option<Vec<u32>> {
    use windows_sys::Win32::NetworkManagement::IpHelper::GetExtendedTcpTable;
    const NO_ERROR: u32 = 0;
    const ERROR_INSUFFICIENT_BUFFER: u32 = 122;

    let mut size: u32 = 0;
    // SAFETY: a null buffer with size 0 asks for the required size.
    let rc = unsafe { GetExtendedTcpTable(std::ptr::null_mut(), &mut size, 0, family, class, 0) };
    if rc != ERROR_INSUFFICIENT_BUFFER && rc != NO_ERROR {
        return None;
    }
    // Room for connections created in between, u32-aligned.
    let mut buf = vec![0u32; (size as usize + 4096) / 4];
    let mut size = (buf.len() * 4) as u32;
    // SAFETY: `buf` is writable for `size` bytes and suitably aligned.
    let rc = unsafe { GetExtendedTcpTable(buf.as_mut_ptr().cast(), &mut size, 0, family, class, 0) };
    (rc == NO_ERROR).then_some(buf)
}

fn net_port(p: u32) -> u16 {
    u16::from_be(p as u16)
}

/// Owning process of the loopback IPv4 connection `local_port -> remote_port`.
fn tcp_owner_pid(local_port: u16, remote_port: u16) -> Option<u32> {
    use windows_sys::Win32::NetworkManagement::IpHelper::{
        MIB_TCPROW_OWNER_PID, MIB_TCPTABLE_OWNER_PID, TCP_TABLE_OWNER_PID_ALL,
    };
    let buf = tcp_table(AF_INET, TCP_TABLE_OWNER_PID_ALL)?;
    // SAFETY: the API filled `buf` with a MIB_TCPTABLE_OWNER_PID of
    // `dwNumEntries` rows, which fit in the returned size.
    let rows = unsafe {
        let table = &*(buf.as_ptr() as *const MIB_TCPTABLE_OWNER_PID);
        std::slice::from_raw_parts(table.table.as_ptr(), table.dwNumEntries as usize)
    };
    let loopback = u32::from_ne_bytes([127, 0, 0, 1]);
    rows.iter()
        .find(|r: &&MIB_TCPROW_OWNER_PID| {
            r.dwLocalAddr == loopback
                && net_port(r.dwLocalPort) == local_port
                && net_port(r.dwRemotePort) == remote_port
        })
        .map(|r| r.dwOwningPid)
}

/// Whether any program listens on TCP `port` (IPv4 or IPv6), read from the
/// system's tables. Unlike trying to listen ourselves, this never makes
/// Windows Firewall ask the user about the program.
pub fn tcp_port_listening(port: u16) -> bool {
    use windows_sys::Win32::NetworkManagement::IpHelper::{
        MIB_TCP6TABLE_OWNER_PID, MIB_TCPTABLE_OWNER_PID, TCP_TABLE_OWNER_PID_LISTENER,
    };
    let v4 = tcp_table(AF_INET, TCP_TABLE_OWNER_PID_LISTENER).is_some_and(|buf| {
        // SAFETY: as in `tcp_owner_pid`.
        let rows = unsafe {
            let table = &*(buf.as_ptr() as *const MIB_TCPTABLE_OWNER_PID);
            std::slice::from_raw_parts(table.table.as_ptr(), table.dwNumEntries as usize)
        };
        rows.iter().any(|r| net_port(r.dwLocalPort) == port)
    });
    v4 || tcp_table(AF_INET6, TCP_TABLE_OWNER_PID_LISTENER).is_some_and(|buf| {
        // SAFETY: the API filled `buf` with a MIB_TCP6TABLE_OWNER_PID.
        let rows = unsafe {
            let table = &*(buf.as_ptr() as *const MIB_TCP6TABLE_OWNER_PID);
            std::slice::from_raw_parts(table.table.as_ptr(), table.dwNumEntries as usize)
        };
        rows.iter().any(|r| net_port(r.dwLocalPort) == port)
    })
}
