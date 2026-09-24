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
if [ "$1" -eq 1 ]; then
    echo "USB Nexus: add desktop users to the 'usbnexus' group: sudo usermod -aG usbnexus <user>"
fi
