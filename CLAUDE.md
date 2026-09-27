# USB Nexus – project notes for contributors and AI assistants

USB Nexus is a GPL-3.0-or-later, clean-room USB-over-IP product (Rust). It
carries standard USB/IP inside mutually authenticated TLS 1.3, with PIN
pairing, mDNS discovery, automatic reconnection, a desktop app, a web UI and
installers. The GitHub repository is `inversa-bilisim/usb-nexus` (moved from
`lippton/demli`, which is no longer used).

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
| `crates/usbnexus-core` | TLS tunnel, pairing (SPAKE2), trust stores, discovery, client/server, daemon + local API, device identities, access control, usage log, web UI, platform backends |
| `crates/usbnexus-i18n` | Fluent translations, `t!` macro, templates for the UIs |
| `crates/usbnexus-cli` | `usbnexus` binary (CLI, daemon, Windows service) |
| `apps/desktop` | Tauri 2 app; `ui/` is plain HTML/CSS/JS shared with the web UI (`web.js` = browser shim) |
| `packaging/{linux,windows,macos}` | systemd/sysusers + deb/rpm scripts; NSIS hooks + VBoxUSB drivers; launchd + pkg |
| `.github/workflows` | `ci.yml` (fmt, clippy -D warnings, tests on Linux/Windows/macOS), `release.yml` (packages on `v*` tags) |

Windows client needs usbip-win2 ≥ 0.9.7.6 (`attach --once`); its installer is
downloaded by `packaging/windows/fetch-usbip-win2.ps1` (release CI) and offered
by `hooks.nsh` when missing or too old.

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

## Implemented features (keep in mind when changing related code)

### Hotplug-tolerant sharing and attaching
- `device_id.rs`: `DeviceId` = `vid:pid:serial`, or `vid:pid@busid` for
  devices without a (usable) serial ("tracked by port"), or a bare bus id
  from older configs (`Legacy`, migrated as soon as the device is seen).
- `backend::SharedExport` stores `SharedDevice`s (id, last known names,
  access) and resolves them against `DeviceHost::list_all()`. Unplugged
  shared devices are still offered (`Offered::device == None`, protocol
  `ExportedDevice::present == false`).
- The daemon polls every 3 s (`poll_devices`): remembers names, migrates
  legacy ids and releases ports a shared device left (Linux: removes the
  stale `usbip-host` `match_busid` entry).
- Clients key attachments by (server fingerprint, device id).
  `NoSuchDevice` and `AccessDenied` are not permanent: `AttachState::Waiting`,
  retried every ≤5 s (missing device) or with backoff (no permission).
  An attachment saved by bus id is re-keyed when the server reports the id
  (`ServerMsg::Imported::id`).

### Access control and usage log
- `access.rs`: `Policy` (open/restricted, `None` in config until chosen →
  open) and per-device `DeviceAccess` (default/open/selected + allowed
  fingerprints; one list shared by "restricted default" and "selected").
- Server checks `Offered::allowed` on import (`ErrorCode::AccessDenied`) and
  marks listings (`allowed`). `Server::enforce()` disconnects sessions no
  longer allowed (called after every sharing/access change and `forget`).
- API: `set_policy`, `set_device_access`, `set_client_access`, `usage`,
  `set_usage_retention`; `status.policy_chosen` drives the first-run dialog.
  CLI: `usbnexus policy [open|restricted]`, `usbnexus history [--csv]`.
- `usage.rs`: JSON lines in `usage.log`, retention 90 days (configurable),
  capped at 10 MB. Repeated refusals of one device to one computer are
  logged once per hour.

## Agreed features (owner, 2026-09-26)

### Language follows the operating system
- Done: installer has no language selector (`displayLanguageSelector:
  false`, English first = fallback); `usbnexus_i18n::detect` falls back to
  the OS preferred UI languages (`sys-locale`) after the `LANG`-style
  variables; the app's language box still wins.
- Installer texts are `setup-*` messages in `locales/*.ftl` (plain text),
  generated into `packaging/windows/usbnexus-strings.nsh`; the web UI
  follows the browser's languages.
- More languages (e.g. French) will be added later: new `locales/xx.ftl`,
  `LOCALES`, and the NSIS `languages` list.

### Role selection in the Windows installer (implemented; not yet tried on Windows)
- Implementation: `packaging/windows/installer.nsi` is the tauri-cli 2.12.0
  template (pinned in release.yml) + `usbnexus-pages.nsh`; texts are the
  `setup-*` messages, generated into `usbnexus-strings.nsh` (test
  `nsis_strings_are_current`). `service install --no-server --no-client
  --web off|local|network --web-port --web-password-file` applies the
  choices (`daemon::apply_setup`); config `roles` (None = both).
  `Request::SetRoles` + `Daemon::set_backend_factory` switch roles at
  runtime (Windows: `winservice::prepare_roles` installs VBoxUSB drivers or
  the bundled usbip-win2; `status.reboot_required`).
- Custom page after the install directory, all ticked by default:
  - "Use as server": installs VBoxUSB drivers (no extra explanation text).
  - "Use as client": installs the bundled usbip-win2 (replaces today's
    message box). While ticked, a note below says usbip-win2 will be
    installed (USB devices pause briefly, restart needed), or that it is
    already installed; the note disappears when unticked.
  - "Web access": if ticked, the next page configures it: password + repeat
    (min 8, required), port (default 3242, editable, checked for being free
    while still on the page, before installing; our own running service
    holding it counts as free), "only this computer" (default) or "the whole
    network".
- Role page is plain (labels only, no descriptions). "Next" is disabled
  (no message) unless server or client is ticked.
- Web page: access radio ("only this computer" default / "whole network"),
  port with live check message, password + repeat (min 8; "Next" disabled
  until valid). On upgrade with a password already set, empty fields keep
  it. Password only, no user name (single administrator).
- If web access was set up, the finish opens https://localhost:<port> in
  the default browser.
- The service is always installed.
- Upgrades remember the previous choices (registry) and preselect them.
- Silent install (`/S`): server + client, web off.
- In the app: screens of a role that is not installed are hidden
  completely; Settings has a place to set up the missing role later
  (server → drivers, client → usbip-win2) and the reverse.

### Device details, waiting queue and automatic handover (agreed 2026-09-27; not implemented yet)
- Clicking a device row on "This computer" opens a details dialog (the row
  switch keeps toggling sharing without opening it): name and ids; "In
  use by" (computer, since when) with a "Disconnect" button (ends the
  session only; blocking is done by unticking the computer); the access
  options and allowed-computer list (the user of the device is marked);
  for unshared devices a share switch (details question still open).
- A USB device serves one computer at a time. Computers that ask for a
  busy device wait in a FIFO queue (client state "queued, n-th"; row:
  "X is using it · 4 computers waiting"; dialog lists the queue).
- Automatic handover: when someone is waiting and the current user has
  had no traffic for the idle time, the session ends and the device is
  reserved for the head of the queue for a few seconds. Never taken away
  when nobody waits; continuously polling software keeps its device.
- Defaults by device kind (per-device override in the dialog: default /
  on / off + seconds; no global settings): storage, HID and others off;
  licence dongles (known vendors: Thales Sentinel/HASP, WIBU CodeMeter,
  Feitian/Rockey, Marx, ...) and printers (class 07) on, 30 s. Unknown
  dongles fall under "others".
- Decided: a computer that handed a device over rejoins the queue by
  itself (two idle computers may pass it back and forth; accepted).
- The device list itself stays as it is (few devices per computer).
- Sidebar entry "Ağ" was renamed "Ağdaki bilgisayarlar" / "Computers on
  the network".

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
11. Hotplug: unplug a shared/attached device and plug it into another port
    (Linux, Windows/VBoxUSB, macOS/libusb); the client must re-attach by
    itself. Check that a device without a serial is tracked by port and that
    no port stays reserved by `usbip-host` afterwards.
12. Serial numbers on Windows (hub string descriptor) and macOS (libusb;
    devices macOS will not open have no serial and are tracked by port).
    Check whether a device captured by VBoxUSB still shows up in the list.
13. Access control across machines: restricted policy, revoke while in use
    (the client's virtual device must disappear), history and CSV export.
