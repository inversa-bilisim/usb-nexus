# SPDX-License-Identifier: GPL-3.0-or-later
if command -v systemd-sysusers >/dev/null 2>&1; then
    systemd-sysusers usbnexus.conf || :
elif ! getent group usbnexus >/dev/null 2>&1; then
    groupadd --system usbnexus || :
fi
if [ -d /run/systemd/system ]; then
    systemctl daemon-reload || :
    systemctl enable usbnexus.service >/dev/null 2>&1 || :
    systemctl restart usbnexus.service || :
fi
if command -v modinfo >/dev/null 2>&1 && ! modinfo -n usbip-host >/dev/null 2>&1; then
    echo "USB Nexus: the usbip-host/vhci-hcd kernel modules were not found for kernel $(uname -r)."
    if [ -r /etc/os-release ] && grep -Eqi '^ID(_LIKE)?=.*(fedora|rhel|centos)' /etc/os-release; then
        echo "USB Nexus: install them with: sudo dnf install kernel-modules-extra"
    fi
fi
if [ "$1" -eq 1 ]; then
    echo "USB Nexus: add desktop users to the 'usbnexus' group: sudo usermod -aG usbnexus <user>"
fi
