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

/// Identifies a version of the file on disk.
type Stamp = Option<(std::time::SystemTime, u64)>;

#[derive(Debug, Default)]
struct State {
    file: TrustFile,
    stamp: Stamp,
}

/// Thread-safe trust store backed by a JSON file. A store created with
/// [`TrustStore::in_memory`] is never written to disk.
///
/// Another process (e.g. the CLI while the service runs) may change the file;
/// such changes are picked up automatically on the next access.
#[derive(Debug, Clone)]
pub struct TrustStore {
    path: Option<PathBuf>,
    inner: Arc<Mutex<State>>,
}

fn stamp(path: &Path) -> Stamp {
    let m = std::fs::metadata(path).ok()?;
    Some((m.modified().ok()?, m.len()))
}

fn read(path: &Path) -> Result<TrustFile> {
    if !path.exists() {
        return Ok(TrustFile::default());
    }
    let data = std::fs::read(path).with_context(|| format!("reading {}", path.display()))?;
    serde_json::from_slice(&data).with_context(|| format!("parsing {}", path.display()))
}

impl TrustStore {
    pub fn load(path: &Path) -> Result<Self> {
        let state = State { stamp: stamp(path), file: read(path)? };
        Ok(TrustStore { path: Some(path.to_path_buf()), inner: Arc::new(Mutex::new(state)) })
    }

    pub fn in_memory() -> Self {
        TrustStore { path: None, inner: Arc::default() }
    }

    /// Locks the state, reloading it first if the file changed on disk.
    fn locked(&self) -> std::sync::MutexGuard<'_, State> {
        let mut st = self.inner.lock().unwrap();
        if let Some(path) = &self.path {
            let now = stamp(path);
            if now != st.stamp {
                match read(path) {
                    Ok(file) => {
                        st.file = file;
                        st.stamp = now;
                    }
                    Err(e) => tracing::warn!("keeping previous trust store: {e:#}"),
                }
            }
        }
        st
    }

    fn save(&self, st: &mut State) -> Result<()> {
        let Some(path) = &self.path else { return Ok(()) };
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let tmp = path.with_extension("tmp");
        crate::identity::write_private(&tmp, &serde_json::to_vec_pretty(&st.file)?)?;
        std::fs::rename(&tmp, path).with_context(|| format!("writing {}", path.display()))?;
        st.stamp = stamp(path);
        Ok(())
    }

    pub fn is_trusted(&self, fingerprint: &str) -> bool {
        self.get(fingerprint).is_some()
    }

    pub fn get(&self, fingerprint: &str) -> Option<Peer> {
        self.locked().file.peers.iter().find(|p| p.fingerprint == fingerprint).cloned()
    }

    pub fn peers(&self) -> Vec<Peer> {
        self.locked().file.peers.clone()
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
        let mut st = self.locked();
        st.file.peers.retain(|p| p.fingerprint != peer.fingerprint);
        st.file.peers.push(peer);
        self.save(&mut st)
    }

    pub fn set_last_addr(&self, fingerprint: &str, addr: &str) -> Result<()> {
        let mut st = self.locked();
        let Some(p) = st.file.peers.iter_mut().find(|p| p.fingerprint == fingerprint) else { return Ok(()) };
        if p.last_addr.as_deref() == Some(addr) {
            return Ok(());
        }
        p.last_addr = Some(addr.to_string());
        self.save(&mut st)
    }

    /// Removes a peer; returns whether one was removed.
    pub fn remove(&self, fingerprint: &str) -> Result<bool> {
        let mut st = self.locked();
        let before = st.file.peers.len();
        st.file.peers.retain(|p| p.fingerprint != fingerprint);
        let removed = st.file.peers.len() != before;
        if removed {
            self.save(&mut st)?;
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

    #[test]
    fn sees_changes_made_by_another_process() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("trust.json");
        let service = TrustStore::load(&path).unwrap();
        assert!(!service.is_trusted("feedbeef"));
        // e.g. `usbnexus pair` running while the service is up
        TrustStore::load(&path).unwrap().add(peer("new", "feedbeef")).unwrap();
        assert!(service.is_trusted("feedbeef"));
    }
}
