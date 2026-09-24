#!/bin/sh
# SPDX-License-Identifier: GPL-3.0-or-later
# Builds USB Nexus-<version>.pkg on macOS: CLI + service + desktop app.
#
#   packaging/macos/build-pkg.sh            # unsigned
#   SIGN_APP="Developer ID Application: ..." SIGN_PKG="Developer ID Installer: ..." \
#     packaging/macos/build-pkg.sh          # signed (then notarize with notarytool)
set -eu
cd "$(dirname "$0")/../.."
VERSION=$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -1)
TARGET=$(rustc -vV | sed -n 's/^host: //p')
OUT=target/macos-pkg
rm -rf "$OUT" && mkdir -p "$OUT/root/usr/local/bin" "$OUT/root/Library/LaunchDaemons" "$OUT/root/Applications"

cargo build --release -p usbnexus-cli
cp target/release/usbnexus "$OUT/root/usr/local/bin/usbnexus"
cp packaging/macos/org.usbnexus.daemon.plist "$OUT/root/Library/LaunchDaemons/"

(cd apps/desktop/src-tauri && cargo tauri build --bundles app --target "$TARGET")
cp -R "target/$TARGET/release/bundle/macos/USB Nexus.app" "$OUT/root/Applications/"

if [ -n "${SIGN_APP:-}" ]; then
    codesign --force --options runtime --timestamp --sign "$SIGN_APP" "$OUT/root/usr/local/bin/usbnexus"
    codesign --force --options runtime --timestamp --deep --sign "$SIGN_APP" "$OUT/root/Applications/USB Nexus.app"
fi

pkgbuild --root "$OUT/root" --scripts packaging/macos/scripts \
    --identifier org.usbnexus.pkg --version "$VERSION" --install-location / \
    ${SIGN_PKG:+--sign "$SIGN_PKG"} \
    "target/USB Nexus-$VERSION.pkg"
echo "Built target/USB Nexus-$VERSION.pkg"
