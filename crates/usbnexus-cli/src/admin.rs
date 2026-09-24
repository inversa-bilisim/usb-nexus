// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! Local administration socket of a running server.
//!
//! A Unix socket readable only by the server's user; the CLI (and later the
//! GUI) uses it to open pairing windows. One JSON request and one JSON reply
//! per line.

use std::path::Path;
use std::time::Duration;

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};
use usbnexus_core::server::Server;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "cmd", rename_all = "snake_case")]
pub enum Request {
    OpenPairing { seconds: u64 },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Reply {
    Pairing { pin: String, seconds: u64, name: String },
    Error { message: String },
}

pub fn listen(path: &Path, server: Server) -> Result<()> {
    if path.exists() {
        std::fs::remove_file(path).ok();
    }
    let listener = UnixListener::bind(path).with_context(|| format!("binding {}", path.display()))?;
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))?;
    }
    tokio::spawn(async move {
        while let Ok((stream, _)) = listener.accept().await {
            let server = server.clone();
            tokio::spawn(async move {
                let _ = handle(stream, server).await;
            });
        }
    });
    Ok(())
}

async fn handle(stream: UnixStream, server: Server) -> Result<()> {
    let (r, mut w) = stream.into_split();
    let mut line = String::new();
    BufReader::new(r).read_line(&mut line).await?;
    let reply = match serde_json::from_str::<Request>(&line) {
        Ok(Request::OpenPairing { seconds }) => {
            let seconds = seconds.clamp(30, 3600);
            let pin = server.open_pairing(Duration::from_secs(seconds));
            Reply::Pairing { pin, seconds, name: server.name().to_string() }
        }
        Err(e) => Reply::Error { message: e.to_string() },
    };
    let mut out = serde_json::to_vec(&reply)?;
    out.push(b'\n');
    w.write_all(&out).await?;
    Ok(())
}

/// Sends one request to a running server.
pub async fn request(path: &Path, req: &Request) -> Result<Reply> {
    let stream = UnixStream::connect(path).await?;
    let (r, mut w) = stream.into_split();
    let mut out = serde_json::to_vec(req)?;
    out.push(b'\n');
    w.write_all(&out).await?;
    let mut line = String::new();
    BufReader::new(r).read_line(&mut line).await?;
    if line.is_empty() {
        bail!("server closed the admin connection");
    }
    Ok(serde_json::from_str(&line)?)
}
