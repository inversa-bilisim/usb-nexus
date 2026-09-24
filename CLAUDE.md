# USB Nexus – project notes for contributors and AI assistants

USB Nexus is a GPL-3.0-or-later, clean-room USB-over-IP product (Rust). It
carries standard USB/IP inside mutually authenticated TLS 1.3, with PIN
pairing, mDNS discovery, automatic reconnection, a desktop app, a web UI and
installers. The GitHub repository is still `lippton/demli`; it will move to the
owner's company account later (only `Cargo.toml` `repository` needs changing).

## Ground rules (from the project owner)

- Code comments and docs inside source files are **English**.
- The user interface must support multiple languages; **Turkish is mandatory**.
  All UI text lives in `locales/*.ftl` (Fluent); `cargo test -p usbnexus-i18n`
  fails if any locale is missing a message. Never hard-code UI strings.
- Never copy GPL-2.0-only code (Linux kernel). Other projects (usbipd-win,
  usbip-win2, VirtualBox headers) may be read for ABI/behaviour only.
- Hardware testing is postponed; the owner will test everything at the end
  (see "Pending end-to-end tests").
- Discuss designs with the owner before large features; the owner writes in
  Turkish, reply in Turkish.

## Layout

| Path | What |
|---|---|
| `crates/usbnexus-proto` | USB/IP wire format (op + URB messages) |
| `crates/usbnexus-core` | TLS tunnel, pairing (SPAKE2), trust stores, discovery, client/server, daemon + local API, web UI, platform backends |
| `crates/usbnexus-i18n` | Fluent translations, `t!` macro, templates for the UIs |
| `crates/usbnexus-cli` | `usbnexus` binary (CLI, daemon, Windows service) |
| `apps/desktop` | Tauri 2 app; `ui/` is plain HTML/CSS/JS shared with the web UI (`web.js` = browser shim) |
| `packaging/{linux,windows,macos}` | systemd/sysusers + deb/rpm scripts; NSIS hooks + VBoxUSB drivers; launchd + pkg |
| `.github/workflows` | `ci.yml` (fmt, clippy -D warnings, tests on Linux/Windows/macOS), `release.yml` (packages on `v*` tags) |

Backends: Linux `usbip-host`/`vhci-hcd` via sysfs; Windows client via
usbip-win2 (`windows.rs` + `bridge.rs`), Windows server via VBoxUSB
(`windows_host.rs`); macOS server via libusb (`libusb_host.rs`, also
`--features libusb` elsewhere); userspace URB server `usb_server.rs`;
`backend::demo` for UI work (`usbnexus daemon --demo`).

## Checks before pushing

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo clippy --workspace --all-targets --target x86_64-pc-windows-gnu -- -D warnings   # needs mingw
CC_x86_64_unknown_freebsd=gcc AR_x86_64_unknown_freebsd=ar \
  cargo clippy --target x86_64-unknown-freebsd -p usbnexus-cli -p usbnexus-core --all-targets -- -D warnings  # macOS-like cfgs
cargo test --workspace
node --check apps/desktop/ui/app.js apps/desktop/ui/web.js
```

## Agreed next features (approved, not implemented yet)

### 1. Hotplug-tolerant sharing and attaching
- Identify shared devices by **VID:PID + serial number** instead of bus id,
  so a device keeps being shared on any port. Devices **without a serial**
  stay tracked by port; the UI notes "tracked by port".
- A shared device that is unplugged stays listed as "not plugged in" and is
  shared again automatically when it returns.
- Clients key attachments by server + device identity; if the device is
  missing they stay in a "waiting for device" state and keep retrying
  (today `NoSuchDevice` is treated as permanent – change that).
- Server detects changes by polling the device list every few seconds
  (all platforms); push notifications can come later.
- Migrate existing configs keyed by bus id automatically.

### 2. Access control and usage log
- Identity is per **computer** (paired certificate), not per person.
- Server-wide default policy: **open** (every paired computer may use every
  shared device – current behaviour) or **restricted** (only allowed
  computers). Pairing stays mandatory in both.
- Per-device override: follow default / open / only selected computers.
- First run of the desktop app or web UI asks which default to use;
  changeable later in settings and with `usbnexus policy open|restricted`.
  Headless installs default to **open**.
- Computers without permission still **see** restricted devices, marked
  "no permission".
- Revoking permission **disconnects** an active session immediately.
- With a restricted default, the server's PIN dialog also lets the admin
  choose which devices the new computer may use (none if skipped).
- Usage log on the server: pairings, failed PIN attempts, attach/detach
  (computer, device, duration), denied attempts. "History" page in the UI,
  CSV export, retention **90 days** (configurable), capped at 10 MB.

## Pending end-to-end tests (to run with real hardware)

1. Linux ↔ Linux with a real USB stick.
2. Linux server → Windows client (usbip-win2); check the kernel-peer (PID 4) check.
3. Windows server (VBoxUSB) → Linux client; capture, bus id, release.
4. Device types: bulk (storage), interrupt (keyboard/mouse), isochronous (audio/camera).
5. Windows installer (`setup.exe` built on Windows): service, drivers, Turkish texts.
6. Desktop app on Windows.
7. macOS: share a device without a macOS driver; `.pkg`, launchd, Turkish UI.
8. `.rpm` on Fedora/openSUSE; systemd service actually starting (not verified in the container).
9. Release workflow with a test tag.
10. Web UI with `web enable --lan` from another computer (e.g. Raspberry Pi server).
