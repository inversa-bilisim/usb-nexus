# Windows

**English** · [Türkçe](README.tr.md)

## Components

| Component | Role |
|---|---|
| `usbnexus.exe` | Background service (`USB Nexus`, LocalSystem) and command line tool |
| `usbnexus-desktop.exe` | Desktop app; talks to the service over `\\.\pipe\usbnexus` |
| [usbip-win2](https://github.com/vadimgrn/usbip-win2) | Microsoft-signed driver that attaches remote devices to Windows (BSD-2-Clause, [license](usbip-win2/LICENSE.txt)); bundled with the installer |
| `drivers\` (VBoxUSB, VBoxUSBMon) | VirtualBox USB drivers for sharing this computer's devices (Oracle + Microsoft signed, GPL-3.0; [details](drivers/README.md)) |

## The installer

`USB Nexus_*_x64-setup.exe` runs as administrator, in the language of Windows (English if that
language is not available). The installer is not code-signed yet, so Windows SmartScreen may show
"Windows protected your PC": click **More info → Run anyway**. After the install folder it asks
how the computer will be used; all options are ticked by default:

| Option | What it does |
|---|---|
| **Use as a server** | Installs the VBoxUSB drivers; the app shows the "This computer" screen. |
| **Use as a client** | Installs the bundled usbip-win2 if it is missing or older than 0.9.7.6 (a note on the page says so; USB devices stop for a few seconds and Windows must be restarted afterwards). The app shows the "Computers on the network" and "Connected devices" screens. |
| **Web access** | The next page asks: access from this computer only (default) or the whole network, the port (3242; checked for being free while you type) and a password (at least 8 characters). When the installation finishes, the web interface opens in the browser (after the restart, if one is needed). |

At least one of server and client must be ticked. The service is always installed and started, and
a firewall rule is added. Installing over an earlier version stops the old service first, keeps
the previous choices preselected, and an empty password keeps the current one. A silent install
(`/S`) installs server and client without the web interface.

Roles can be changed later under **Settings** in the app; adding a role installs what it needs.
Uninstalling removes the service; usbip-win2 stays installed (other programs may use it; it has
its own entry under "Installed apps").

## Building the installer (on Windows)

```powershell
cargo install tauri-cli --version "=2.12.0" --locked   # the version whose NSIS template installer.nsi copies
cargo build --release -p usbnexus-cli
New-Item -ItemType Directory -Force apps\desktop\src-tauri\binaries
Copy-Item target\release\usbnexus.exe apps\desktop\src-tauri\binaries\usbnexus-x86_64-pc-windows-msvc.exe
packaging\windows\fetch-usbip-win2.ps1   # downloads the usbip-win2 installer and checks its SHA-256
# packaging\windows\drivers is bundled automatically (tauri.bundle.json -> resources)
cd apps\desktop\src-tauri
cargo tauri build --config ..\..\..\packaging\windows\tauri.bundle.json
```

Output: `target\release\bundle\nsis\USB Nexus_*_x64-setup.exe`.

- `installer.nsi` is a copy of the Tauri NSIS template with two extra pages (`usbnexus-pages.nsh`).
- Installer texts come from the `setup-*` messages in `locales/*.ftl`; `usbnexus-strings.nsh` is
  generated from them (`UPDATE_NSIS_STRINGS=1 cargo test -p usbnexus-i18n`). A new language needs
  an entry in the `languages` list of `tauri.bundle.json` too.

## Manual installation

```powershell
# Administrator PowerShell
usbnexus.exe service install                       # installs and starts the service (both roles)
usbnexus.exe service install --no-client --web local --web-port 3242 --web-password-file pw.txt
usbnexus.exe service uninstall
```

- Settings, keys and log: `C:\ProgramData\USB Nexus\` (`service.log`)
- Control channel `\\.\pipe\usbnexus`: SYSTEM, administrators and signed-in local users
- `usbip.exe` is looked for in `C:\Program Files\USBip\usbip.exe`, on `PATH`, or where the
  `USBNEXUS_USBIP` environment variable points

## Status

Tested on two Windows 11 computers: sharing a USB stick (VBoxUSB), using it on the other computer
(usbip-win2), the desktop app, the installer pages and the web interface from the network. While a
device is shared it is taken from Windows and handed to VBoxUSB; when the session ends it goes back
to its normal driver. `service install` adds the driver package and registers the `VBoxUSBMon`
kernel service (or reuses the one registered by usbipd-win, if present).
