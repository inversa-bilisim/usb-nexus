# Linux

**English** · [Türkçe](README.tr.md)

## Packages

| Package | Contents |
|---|---|
| `usbnexus` | `/usr/bin/usbnexus`, the systemd service, the `usbnexus` group. Enough on its own for headless servers. |
| `usb-nexus` | Desktop app (`usbnexus-desktop`); depends on `usbnexus`. |

Installing enables and starts the service:

```sh
sudo apt install ./usbnexus_*.deb ./usb-nexus_*.deb      # or: sudo dnf install ./usbnexus-*.rpm ./USB*.rpm
```

- **User access:** the desktop app can control the service only for members of the `usbnexus`
  group. When a user lacks it, the app shows an **"Allow this user"** button (asks for the
  administrator password; takes effect at once). By hand: `sudo usbnexus allow-user USER`.
- **Kernel modules:** `usbip-host` and `vhci-hcd` come with the kernel of most distributions; the
  service loads them itself. Where they are missing (Ubuntu cloud kernels:
  `linux-modules-extra-$(uname -r)`; Fedora/RHEL: `kernel-modules-extra`, which the RPM recommends)
  the app and the web interface show the package to install.
- Both roles (server and client) are active; the web interface is off until
  `sudo usbnexus web enable` (see the main README).

## Building the packages

```sh
cargo install cargo-deb cargo-generate-rpm --locked
cargo install tauri-cli --version "^2" --locked

cargo build --release -p usbnexus-cli
cargo deb -p usbnexus-cli --no-build                 # target/debian/usbnexus_*.deb
cargo generate-rpm -p crates/usbnexus-cli            # target/generate-rpm/usbnexus-*.rpm
(cd apps/desktop/src-tauri && cargo tauri build --config ../../../packaging/linux/tauri.bundle.json)
                                                     # target/release/bundle/{deb,rpm}/
```

## Manual installation (without packages)

```sh
sudo install -m 755 target/release/usbnexus /usr/bin/usbnexus
sudo install -m 644 packaging/linux/usbnexus.service /usr/lib/systemd/system/usbnexus.service
sudo install -m 644 packaging/linux/usbnexus.sysusers /usr/lib/sysusers.d/usbnexus.conf
sudo systemd-sysusers && sudo systemctl daemon-reload && sudo systemctl enable --now usbnexus
```

- Status: `systemctl status usbnexus`; log: `journalctl -u usbnexus -f`
- Settings and keys: `/var/lib/usbnexus`
- Control socket: `/run/usbnexus/daemon.sock` (`root` and the `usbnexus` group only)
- Purging the package (`purge`) deletes the settings too.
