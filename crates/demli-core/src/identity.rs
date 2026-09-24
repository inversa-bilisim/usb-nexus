// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Demli contributors

//! Long-term identity of a Demli node: a self-signed certificate and its key.
//!
//! Certificates are not validated against a CA. A peer is identified by the
//! SHA-256 fingerprint of its certificate, which is pinned during pairing.

use std::fs;
use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result};
use sha2::{Digest, Sha256};

const CERT_FILE: &str = "identity.der";
const KEY_FILE: &str = "identity.key";

#[derive(Clone)]
pub struct Identity {
    pub cert_der: Vec<u8>,
    pub key_der: Vec<u8>,
}

impl std::fmt::Debug for Identity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Identity").field("fingerprint", &self.fingerprint()).finish_non_exhaustive()
    }
}

/// SHA-256 fingerprint of a DER certificate, lowercase hex.
pub fn fingerprint(cert_der: &[u8]) -> String {
    hex::encode(Sha256::digest(cert_der))
}

/// Short human friendly form of a fingerprint (`ab12-cd34-ef56-7890`).
pub fn short_fingerprint(fp: &str) -> String {
    fp.as_bytes().chunks(4).take(4).map(|c| std::str::from_utf8(c).unwrap_or("")).collect::<Vec<_>>().join("-")
}

impl Identity {
    /// Generates a fresh identity.
    pub fn generate(name: &str) -> Result<Self> {
        let key = rcgen::KeyPair::generate().context("generating key pair")?;
        let mut params = rcgen::CertificateParams::new(vec!["demli.invalid".to_string()])
            .context("building certificate parameters")?;
        params.distinguished_name.push(rcgen::DnType::CommonName, name);
        let cert = params.self_signed(&key).context("self-signing certificate")?;
        Ok(Identity { cert_der: cert.der().to_vec(), key_der: key.serialize_der() })
    }

    /// Loads the identity from `dir`, creating one if it does not exist yet.
    pub fn load_or_create(dir: &Path, name: &str) -> Result<Self> {
        let cert_path = dir.join(CERT_FILE);
        let key_path = dir.join(KEY_FILE);
        if cert_path.exists() && key_path.exists() {
            return Ok(Identity {
                cert_der: fs::read(&cert_path).with_context(|| format!("reading {}", cert_path.display()))?,
                key_der: fs::read(&key_path).with_context(|| format!("reading {}", key_path.display()))?,
            });
        }
        fs::create_dir_all(dir).with_context(|| format!("creating {}", dir.display()))?;
        let id = Self::generate(name)?;
        write_private(&key_path, &id.key_der)?;
        fs::write(&cert_path, &id.cert_der).with_context(|| format!("writing {}", cert_path.display()))?;
        Ok(id)
    }

    pub fn fingerprint(&self) -> String {
        fingerprint(&self.cert_der)
    }
}

/// Writes a file readable only by the current user.
pub(crate) fn write_private(path: &Path, data: &[u8]) -> Result<()> {
    let mut opts = fs::OpenOptions::new();
    opts.write(true).create(true).truncate(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        opts.mode(0o600);
    }
    let mut f = opts.open(path).with_context(|| format!("writing {}", path.display()))?;
    f.write_all(data)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_then_reload() {
        let dir = tempfile::tempdir().unwrap();
        let a = Identity::load_or_create(dir.path(), "test").unwrap();
        let b = Identity::load_or_create(dir.path(), "test").unwrap();
        assert_eq!(a.fingerprint(), b.fingerprint());
        assert_eq!(a.fingerprint().len(), 64);
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = fs::metadata(dir.path().join(KEY_FILE)).unwrap().permissions().mode();
            assert_eq!(mode & 0o077, 0);
        }
    }

    #[test]
    fn short_fp() {
        assert_eq!(short_fingerprint("0123456789abcdef0123"), "0123-4567-89ab-cdef");
    }
}
