#!/bin/sh
# SPDX-License-Identifier: GPL-3.0-or-later
# Removes USB Nexus. Run with sudo. Keeps keys and pairings unless --purge.
set -e
PLIST=/Library/LaunchDaemons/org.usbnexus.daemon.plist
launchctl bootout system "$PLIST" 2>/dev/null || true
rm -f "$PLIST" /usr/local/bin/usbnexus
rm -rf "/Applications/USB Nexus.app" "/Library/Logs/USB Nexus"
pkgutil --forget org.usbnexus.pkg >/dev/null 2>&1 || true
if [ "$1" = "--purge" ]; then
    rm -rf "/Library/Application Support/USB Nexus"
fi
echo "USB Nexus removed."
