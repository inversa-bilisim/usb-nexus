# macOS

**English** · [Türkçe](README.tr.md)

## What is supported

| Scenario | Status |
|---|---|
| **Sharing a device plugged into this Mac** (Mac as server) | Through libusb. Devices without a macOS driver of their own (programmers, development boards, some printers/scanners, custom hardware) can be shared. |
| Devices macOS uses with its own driver (USB sticks, keyboards, mice) | Need Apple's `com.apple.vm.device-access` entitlement; without it they are reported as "used by the operating system". |
| **Using a remote device on the Mac** (Mac as client) | Not supported: macOS does not let third parties provide a virtual USB controller. |
| Isochronous transfers (audio/video) | Not yet. |

## Building the package (on a Mac)

```sh
cargo install tauri-cli --version "^2" --locked
packaging/macos/build-pkg.sh
```

Output: `target/USB Nexus-<version>.pkg`. It installs:

- `/usr/local/bin/usbnexus` (command line + service)
- `/Library/LaunchDaemons/org.usbnexus.daemon.plist` (starts as root at boot)
- `/Applications/USB Nexus.app`

Settings and keys live in `/Library/Application Support/USB Nexus`, the log in
`/Library/Logs/USB Nexus/daemon.log`. Members of the `admin` group can use the desktop app.
To uninstall: `sudo packaging/macos/uninstall.sh`.

## Signing and notarization

The unsigned package and app work, but Gatekeeper warns on first launch (right-click → Open in
Finder). For warning-free distribution, sign with an Apple Developer account (paid, yearly) by
setting `SIGN_APP` / `SIGN_PKG`, and notarize with `xcrun notarytool`.
