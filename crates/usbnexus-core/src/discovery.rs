// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! LAN discovery of USB Nexus servers via mDNS / DNS-SD.
//!
//! Servers advertise `_usbnexus._tcp.local.` with their certificate fingerprint
//! in the TXT record. Discovery only finds candidates: trust is always
//! established by the TLS fingerprint check, never by mDNS data.

use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;

use anyhow::{Context, Result};
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};

pub const SERVICE_TYPE: &str = "_usbnexus._tcp.local.";

/// Keeps a service registered while alive.
pub struct Advertiser {
    daemon: ServiceDaemon,
    fullname: String,
}

impl Drop for Advertiser {
    fn drop(&mut self) {
        let _ = self.daemon.unregister(&self.fullname);
        let _ = self.daemon.shutdown();
    }
}

/// Makes a string safe to use as a DNS label.
fn label(name: &str) -> String {
    let s: String =
        name.chars().map(|c| if c.is_ascii_alphanumeric() || c == '-' { c } else { '-' }).take(40).collect();
    let s = s.trim_matches('-').to_string();
    if s.is_empty() {
        "usbnexus".into()
    } else {
        s
    }
}

pub fn advertise(name: &str, port: u16, fingerprint: &str) -> Result<Advertiser> {
    let daemon = ServiceDaemon::new().context("starting mDNS")?;
    let host = format!("{}-{}.local.", label(name), &fingerprint[..8.min(fingerprint.len())]);
    let props = [("fp", fingerprint), ("v", "1")];
    let info = ServiceInfo::new(SERVICE_TYPE, name, &host, "", port, &props[..])
        .context("building mDNS record")?
        .enable_addr_auto();
    let fullname = info.get_fullname().to_string();
    daemon.register(info).context("registering mDNS service")?;
    Ok(Advertiser { daemon, fullname })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Discovered {
    pub name: String,
    pub fingerprint: String,
    pub addrs: Vec<SocketAddr>,
}

fn to_discovered(info: &ServiceInfo) -> Option<Discovered> {
    let fingerprint = info.get_property_val_str("fp")?.to_string();
    let name = info.get_fullname().strip_suffix(SERVICE_TYPE)?.trim_end_matches('.').to_string();
    let port = info.get_port();
    let mut addrs: Vec<SocketAddr> = info.get_addresses().iter().map(|ip| SocketAddr::new(*ip, port)).collect();
    // Prefer IPv4, then global IPv6; link-local IPv6 needs a scope id we do not have.
    addrs.retain(|a| match a.ip() {
        IpAddr::V6(v6) => (v6.segments()[0] & 0xffc0) != 0xfe80,
        IpAddr::V4(_) => true,
    });
    addrs.sort_by_key(|a| a.is_ipv6());
    Some(Discovered { name, fingerprint, addrs })
}

async fn run_browse(wait: Duration, stop_on: Option<&str>) -> Result<Vec<Discovered>> {
    let daemon = ServiceDaemon::new().context("starting mDNS")?;
    let rx = daemon.browse(SERVICE_TYPE).context("browsing mDNS")?;
    let mut found: HashMap<String, Discovered> = HashMap::new();
    let deadline = tokio::time::Instant::now() + wait;
    loop {
        let ev = tokio::time::timeout_at(deadline, rx.recv_async()).await;
        match ev {
            Ok(Ok(ServiceEvent::ServiceResolved(info))) => {
                if let Some(d) = to_discovered(&info) {
                    let hit = stop_on == Some(d.fingerprint.as_str());
                    found.insert(d.fingerprint.clone(), d);
                    if hit {
                        break;
                    }
                }
            }
            Ok(Ok(_)) => {}
            Ok(Err(_)) | Err(_) => break,
        }
    }
    let _ = daemon.stop_browse(SERVICE_TYPE);
    let _ = daemon.shutdown();
    let mut v: Vec<_> = found.into_values().collect();
    v.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(v)
}

/// Lists servers seen on the LAN within `wait`.
pub async fn browse(wait: Duration) -> Result<Vec<Discovered>> {
    run_browse(wait, None).await
}

/// Looks for a server with the given fingerprint, returning as soon as found.
pub async fn find(fingerprint: &str, wait: Duration) -> Result<Option<Discovered>> {
    Ok(run_browse(wait, Some(fingerprint)).await?.into_iter().find(|d| d.fingerprint == fingerprint))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labels() {
        assert_eq!(label("Ofis PC (1)"), "Ofis-PC--1");
        assert_eq!(label("***"), "usbnexus");
    }
}
