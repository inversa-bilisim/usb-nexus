// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Demli contributors

//! Length-prefixed JSON frames used by the control protocol.

use anyhow::{bail, Result};
use serde::{de::DeserializeOwned, Serialize};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

/// Largest accepted control frame.
pub const MAX_FRAME: usize = 1024 * 1024;

pub async fn write_frame<W, T>(w: &mut W, msg: &T) -> Result<()>
where
    W: AsyncWrite + Unpin,
    T: Serialize,
{
    let body = serde_json::to_vec(msg)?;
    if body.len() > MAX_FRAME {
        bail!("control frame too large ({} bytes)", body.len());
    }
    w.write_all(&(body.len() as u32).to_be_bytes()).await?;
    w.write_all(&body).await?;
    w.flush().await?;
    Ok(())
}

pub async fn read_frame<R, T>(r: &mut R) -> Result<T>
where
    R: AsyncRead + Unpin,
    T: DeserializeOwned,
{
    let mut len = [0u8; 4];
    r.read_exact(&mut len).await?;
    let len = u32::from_be_bytes(len) as usize;
    if len > MAX_FRAME {
        bail!("control frame too large ({len} bytes)");
    }
    let mut body = vec![0u8; len];
    r.read_exact(&mut body).await?;
    Ok(serde_json::from_slice(&body)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn roundtrip_and_limits() {
        let (mut a, mut b) = tokio::io::duplex(64 * 1024);
        write_frame(&mut a, &vec!["hi".to_string()]).await.unwrap();
        let got: Vec<String> = read_frame(&mut b).await.unwrap();
        assert_eq!(got, ["hi"]);

        a.write_all(&(u32::MAX).to_be_bytes()).await.unwrap();
        assert!(read_frame::<_, Vec<String>>(&mut b).await.is_err());
    }
}
