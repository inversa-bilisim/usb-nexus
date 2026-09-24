// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! Persistent store of paired peers, keyed by certificate fingerprint.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Peer {
    pub name: String,
    pub fingerprint: String,
    /// Last address the peer was reached at (clients remember servers).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_addr: Option<String>,
    /// Unix timestamp of pairing.
    pub paired_at: u64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct TrustFile {
    peers: Vec<Peer>,
}

/// Thread-safe trust store backed by a JSON file. A store created with
/// [`TrustStore::in_memory`] is never written to disk.
#[derive(Debug, Clone)]
pub struct TrustStore {
    path: Option<PathBuf>,
    inner: Arc<Mutex<TrustFile>>,
}

impl TrustStore {
    pub fn load(path: &Path) -> Result<Self> {
        let file = if path.exists() {
            let data = std::fs::read(path).with_context(|| format!("reading {}", path.display()))?;
            serde_json::from_slice(&data).with_context(|| format!("parsing {}", path.display()))?
        } else {
            TrustFile::default()
        };
        Ok(TrustStore { path: Some(path.to_path_buf()), inner: Arc::new(Mutex::new(file)) })
    }

    pub fn in_memory() -> Self {
        TrustStore { path: None, inner: Arc::default() }
    }

    fn save(&self, file: &TrustFile) -> Result<()> {
        let Some(path) = &self.path else { return Ok(()) };
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let tmp = path.with_extension("tmp");
        crate::identity::write_private(&tmp, &serde_json::to_vec_pretty(file)?)?;
        std::fs::rename(&tmp, path).with_context(|| format!("writing {}", path.display()))?;
        Ok(())
    }

    pub fn is_trusted(&self, fingerprint: &str) -> bool {
        self.get(fingerprint).is_some()
    }

    pub fn get(&self, fingerprint: &str) -> Option<Peer> {
        self.inner.lock().unwrap().peers.iter().find(|p| p.fingerprint == fingerprint).cloned()
    }

    pub fn peers(&self) -> Vec<Peer> {
        self.inner.lock().unwrap().peers.clone()
    }

    /// Finds a peer by exact name, fingerprint, or unique fingerprint prefix.
    pub fn find(&self, query: &str) -> Option<Peer> {
        let peers = self.peers();
        if let Some(p) = peers.iter().find(|p| p.name == query || p.fingerprint == query) {
            return Some(p.clone());
        }
        let q = query.replace('-', "").to_lowercase();
        if q.len() < 4 {
            return None;
        }
        let mut hits = peers.into_iter().filter(|p| p.fingerprint.starts_with(&q));
        match (hits.next(), hits.next()) {
            (Some(p), None) => Some(p),
            _ => None,
        }
    }

    /// Adds or replaces a peer.
    pub fn add(&self, peer: Peer) -> Result<()> {
        let mut file = self.inner.lock().unwrap();
        file.peers.retain(|p| p.fingerprint != peer.fingerprint);
        file.peers.push(peer);
        self.save(&file)
    }

    pub fn set_last_addr(&self, fingerprint: &str, addr: &str) -> Result<()> {
        let mut file = self.inner.lock().unwrap();
        let Some(p) = file.peers.iter_mut().find(|p| p.fingerprint == fingerprint) else { return Ok(()) };
        if p.last_addr.as_deref() == Some(addr) {
            return Ok(());
        }
        p.last_addr = Some(addr.to_string());
        self.save(&file)
    }

    /// Removes a peer; returns whether one was removed.
    pub fn remove(&self, fingerprint: &str) -> Result<bool> {
        let mut file = self.inner.lock().unwrap();
        let before = file.peers.len();
        file.peers.retain(|p| p.fingerprint != fingerprint);
        let removed = file.peers.len() != before;
        if removed {
            self.save(&file)?;
        }
        Ok(removed)
    }
}

pub(crate) fn now_unix() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn peer(name: &str, fp: &str) -> Peer {
        Peer { name: name.into(), fingerprint: fp.into(), last_addr: None, paired_at: 0 }
    }

    #[test]
    fn persist_and_find() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("trust.json");
        let t = TrustStore::load(&path).unwrap();
        t.add(peer("office", "abcdef01")).unwrap();
        t.add(peer("lab", "abcd9999")).unwrap();
        t.set_last_addr("abcdef01", "10.0.0.2:3241").unwrap();

        let t2 = TrustStore::load(&path).unwrap();
        assert!(t2.is_trusted("abcdef01"));
        assert_eq!(t2.find("office").unwrap().last_addr.as_deref(), Some("10.0.0.2:3241"));
        assert_eq!(t2.find("abcd-ef").unwrap().name, "office");
        assert!(t2.find("abcd").is_none(), "ambiguous prefix");
        assert!(t2.remove("abcd9999").unwrap());
        assert_eq!(TrustStore::load(&path).unwrap().peers().len(), 1);
    }
}
