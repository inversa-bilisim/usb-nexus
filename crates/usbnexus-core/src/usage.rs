// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! Usage log of the server: pairings, failed PIN attempts, device use and
//! refused requests.
//!
//! Stored as JSON lines in `usage.log` in the state directory. Entries older
//! than the retention period are dropped, and the file is kept below
//! [`MAX_BYTES`] by dropping the oldest entries.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::trust::now_unix;

/// Default number of days entries are kept.
pub const DEFAULT_RETENTION_DAYS: u32 = 90;
/// Largest size of the log file.
pub const MAX_BYTES: u64 = 10 * 1024 * 1024;
/// Size the file is trimmed to when it grows past [`MAX_BYTES`], so trimming
/// does not happen on every new entry.
const TRIM_TO: u64 = MAX_BYTES * 8 / 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UsageKind {
    /// A new computer was paired.
    Paired,
    /// Someone entered a wrong PIN.
    PairingFailed,
    /// A computer started using a device.
    Attached,
    /// A computer stopped using a device (`duration_secs` says how long).
    Detached,
    /// A computer asked for a device it may not use.
    Denied,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UsageEntry {
    /// Unix time in seconds.
    pub time: u64,
    pub kind: UsageKind,
    /// Name of the computer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub computer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// Network address of the computer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Device identity (see [`crate::device_id::DeviceId`]).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// Human readable device name, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration_secs: Option<u64>,
}

impl UsageEntry {
    /// An entry of `kind` stamped with the current time.
    pub fn now(kind: UsageKind) -> Self {
        UsageEntry {
            time: now_unix(),
            kind,
            computer: None,
            fingerprint: None,
            address: None,
            device: None,
            device_name: None,
            duration_secs: None,
        }
    }
}

pub struct UsageLog {
    path: PathBuf,
    state: Mutex<LogState>,
}

struct LogState {
    retention_days: u32,
    max_bytes: u64,
}

fn read_entries(path: &Path) -> Vec<UsageEntry> {
    let Ok(text) = fs::read_to_string(path) else { return vec![] };
    // Unreadable lines (e.g. cut off by a crash) are skipped.
    text.lines().filter_map(|l| serde_json::from_str(l).ok()).collect()
}

impl UsageLog {
    /// Opens (or creates on first entry) the log at `path` and applies the
    /// retention period.
    pub fn open(path: &Path, retention_days: u32) -> Self {
        let log = UsageLog {
            path: path.to_path_buf(),
            state: Mutex::new(LogState { retention_days: retention_days.max(1), max_bytes: MAX_BYTES }),
        };
        log.prune();
        log
    }

    #[cfg(test)]
    fn with_max_bytes(path: &Path, retention_days: u32, max_bytes: u64) -> Self {
        let log = UsageLog::open(path, retention_days);
        log.state.lock().unwrap().max_bytes = max_bytes;
        log
    }

    pub fn retention_days(&self) -> u32 {
        self.state.lock().unwrap().retention_days
    }

    /// Changes the retention period and drops entries that are now too old.
    pub fn set_retention_days(&self, days: u32) {
        self.state.lock().unwrap().retention_days = days.max(1);
        self.prune();
    }

    /// Appends an entry. Failures are logged, never fatal.
    pub fn record(&self, entry: UsageEntry) {
        let st = self.state.lock().unwrap();
        if let Err(e) = self.append(&entry) {
            tracing::warn!("could not write the usage log: {e:#}");
            return;
        }
        let size = fs::metadata(&self.path).map(|m| m.len()).unwrap_or(0);
        if size > st.max_bytes {
            self.rewrite(&st, st.max_bytes * TRIM_TO / MAX_BYTES);
        }
    }

    fn append(&self, entry: &UsageEntry) -> Result<()> {
        if let Some(dir) = self.path.parent() {
            fs::create_dir_all(dir)?;
        }
        let mut line = serde_json::to_vec(entry)?;
        line.push(b'\n');
        let mut opts = fs::OpenOptions::new();
        opts.create(true).append(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            opts.mode(0o600);
        }
        let mut f = opts.open(&self.path).with_context(|| format!("opening {}", self.path.display()))?;
        f.write_all(&line)?;
        Ok(())
    }

    /// All entries, oldest first.
    pub fn entries(&self) -> Vec<UsageEntry> {
        let _st = self.state.lock().unwrap();
        read_entries(&self.path)
    }

    /// Drops entries older than the retention period.
    pub fn prune(&self) {
        let st = self.state.lock().unwrap();
        if self.path.exists() {
            self.rewrite(&st, st.max_bytes);
        }
    }

    /// Rewrites the file without entries past retention, dropping the oldest
    /// further until it fits in `limit` bytes.
    fn rewrite(&self, st: &LogState, limit: u64) {
        let entries = read_entries(&self.path);
        let cutoff = now_unix().saturating_sub(st.retention_days as u64 * 86_400);
        let lines: Vec<String> =
            entries.iter().filter(|e| e.time >= cutoff).filter_map(|e| serde_json::to_string(e).ok()).collect();
        let mut total: u64 = lines.iter().map(|l| l.len() as u64 + 1).sum();
        let mut skip = 0;
        while total > limit && skip < lines.len() {
            total -= lines[skip].len() as u64 + 1;
            skip += 1;
        }
        if skip == 0 && lines.len() == entries.len() {
            return; // nothing to drop
        }
        let mut out = String::with_capacity(total as usize);
        for l in &lines[skip..] {
            out.push_str(l);
            out.push('\n');
        }
        let tmp = self.path.with_extension("tmp");
        let written = crate::identity::write_private(&tmp, out.as_bytes())
            .and_then(|()| fs::rename(&tmp, &self.path).context("replacing the usage log"));
        if let Err(e) = written {
            tracing::warn!("could not trim the usage log: {e:#}");
        }
    }
}

/// Formats a Unix time as `YYYY-MM-DD HH:MM:SS` (UTC).
pub fn format_utc(unix: u64) -> String {
    let days = (unix / 86_400) as i64;
    let secs = unix % 86_400;
    // Civil date from days since 1970-01-01 (proleptic Gregorian calendar).
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    format!("{year:04}-{month:02}-{day:02} {:02}:{:02}:{:02}", secs / 3600, secs % 3600 / 60, secs % 60)
}

/// Quotes a CSV field when needed (RFC 4180). Fields that a spreadsheet
/// would run as a formula are prefixed with an apostrophe.
pub fn csv_field(s: &str) -> String {
    let s = if s.starts_with(['=', '+', '-', '@', '\t', '\r']) { format!("'{s}") } else { s.to_string() };
    if s.contains([',', '"', '\n', '\r', ';']) {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(time: u64, name: &str) -> UsageEntry {
        UsageEntry { time, computer: Some(name.into()), ..UsageEntry::now(UsageKind::Attached) }
    }

    #[test]
    fn records_and_applies_retention() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("usage.log");
        let log = UsageLog::open(&path, 90);
        let now = now_unix();
        log.record(entry(now - 100 * 86_400, "old"));
        log.record(entry(now - 10 * 86_400, "recent"));
        log.record(entry(now, "new"));
        assert_eq!(log.entries().len(), 3, "retention is applied on open and when trimming");

        let log = UsageLog::open(&path, 90);
        let names: Vec<_> = log.entries().into_iter().map(|e| e.computer.unwrap()).collect();
        assert_eq!(names, ["recent", "new"]);

        log.set_retention_days(5);
        assert_eq!(log.entries().len(), 1);
    }

    #[test]
    fn size_is_capped() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("usage.log");
        let log = UsageLog::with_max_bytes(&path, 90, 2_000);
        for i in 0..100 {
            log.record(entry(now_unix(), &format!("pc{i}")));
        }
        assert!(fs::metadata(&path).unwrap().len() <= 2_000);
        let entries = log.entries();
        assert_eq!(entries.last().unwrap().computer.as_deref(), Some("pc99"), "newest entries are kept");
        assert!(entries.len() > 5);
    }

    #[test]
    fn skips_damaged_lines() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("usage.log");
        fs::write(&path, format!("{}\n{{\"time\":1,\"ki", serde_json::to_string(&entry(now_unix(), "a")).unwrap()))
            .unwrap();
        assert_eq!(UsageLog::open(&path, 90).entries().len(), 1);
    }

    #[test]
    fn formatting() {
        assert_eq!(format_utc(0), "1970-01-01 00:00:00");
        assert_eq!(format_utc(951_782_400), "2000-02-29 00:00:00");
        assert_eq!(format_utc(1_790_000_000), "2026-09-21 14:13:20");
        assert_eq!(csv_field("plain"), "plain");
        assert_eq!(csv_field("a,\"b\""), "\"a,\"\"b\"\"\"");
        assert_eq!(csv_field("=cmd()"), "'=cmd()");
    }
}
