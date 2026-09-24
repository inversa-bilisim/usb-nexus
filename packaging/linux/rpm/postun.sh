# SPDX-License-Identifier: GPL-3.0-or-later
if [ -d /run/systemd/system ]; then
    systemctl daemon-reload || :
fi
