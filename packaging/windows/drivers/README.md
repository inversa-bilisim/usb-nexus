# VirtualBox USB drivers (third-party binaries)

These unmodified driver binaries come from the binary distribution of
**VirtualBox 7.2.20** by Oracle Corporation, as redistributed by
[usbipd-win](https://github.com/dorssel/usbipd-win). They are licensed under the
[GNU General Public License, version 3](../../../LICENSE) (see the `.license`
files next to each binary).

| File | Purpose | Signature |
|---|---|---|
| `VBoxUSBMon.sys` | Capture filter (monitor) driver | Oracle America, Inc. + Microsoft Windows Hardware Compatibility Publisher |
| `VBoxUSB.sys`, `VBoxUSB.inf`, `VBoxUSB.cat` | Driver for captured devices | Oracle America, Inc.; catalog signed by Microsoft |

USB Nexus uses them to share USB devices of a Windows computer: the device is
handed to `VBoxUSB` for the duration of a sharing session and given back to its
normal driver afterwards.

## Corresponding source (GPL-3.0, section 6d)

The complete corresponding source code of these binaries is part of the
VirtualBox 7.2.20 source release, available from Oracle at:

- <https://download.virtualbox.org/virtualbox/7.2.20/>
  (`VirtualBox-7.2.20.tar.bz2`; drivers in `src/VBox/HostDrivers/VBoxUSB/win/`)

If that location ever becomes unavailable, open an issue in this project and the
USB Nexus maintainers will provide the source.
