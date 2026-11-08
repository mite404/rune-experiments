#!/usr/bin/env bash
# Wrap a release binary in a minimal macOS .app bundle.
# Usage: scripts/bundle-mac.sh <bin-name> "<App Name>"
#   e.g. scripts/bundle-mac.sh l2_queue "Up Next"   →   dist/Up Next.app
# ponytail: no icon and an ad-hoc signature; use rune's scripts/mac-app when you need either.
set -euo pipefail

bin="$1"
name="$2"
exe="${CARGO_TARGET_DIR:-target}/release/$bin"
app="dist/$name.app"

[ -x "$exe" ] || { echo "no $exe - run: cargo build --release --bin $bin" >&2; exit 1; }

rm -rf "$app"
mkdir -p "$app/Contents/MacOS"
cp "$exe" "$app/Contents/MacOS/$bin"
cat > "$app/Contents/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
  <key>CFBundleName</key><string>$name</string>
  <key>CFBundleIdentifier</key><string>dev.tutorial.$bin</string>
  <key>CFBundleExecutable</key><string>$bin</string>
  <key>CFBundlePackageType</key><string>APPL</string>
  <key>CFBundleShortVersionString</key><string>0.1.0</string>
  <key>LSMinimumSystemVersion</key><string>14.0</string>
  <key>NSHighResolutionCapable</key><true/>
</dict></plist>
EOF
codesign --force --sign - "$app"
echo "built $app"
