# SPDX-License-Identifier: GPL-3.0-or-later
# $1 == 0: package is being removed (not upgraded).
if [ "$1" -eq 0 ] && [ -d /run/systemd/system ]; then
    systemctl disable --now usbnexus.service >/dev/null 2>&1 || :
fi
