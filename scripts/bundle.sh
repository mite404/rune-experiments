#!/usr/bin/env bash
# Builds "Rune Experiments.app" (ad-hoc signed) and a zip of it into dist/. macOS only.
set -euo pipefail
cd "$(dirname "$0")/.."
MACOSX_DEPLOYMENT_TARGET=14.0 cargo build --release
app="dist/Rune Experiments.app"
rm -rf "$app" && mkdir -p "$app/Contents/MacOS"
cp target/release/rune-experiments "$app/Contents/MacOS/"
cat > "$app/Contents/Info.plist" <<'PLIST'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
  <key>CFBundleName</key><string>Rune Experiments</string>
  <key>CFBundleIdentifier</key><string>dev.mite404.rune-experiments</string>
  <key>CFBundleExecutable</key><string>rune-experiments</string>
  <key>CFBundlePackageType</key><string>APPL</string>
  <key>CFBundleShortVersionString</key><string>0.0.1</string>
  <key>CFBundleVersion</key><string>1</string>
  <key>LSMinimumSystemVersion</key><string>14.0</string>
  <key>NSHighResolutionCapable</key><true/>
</dict></plist>
PLIST
codesign --force --sign - "$app"
ditto -c -k --keepParent "$app" "dist/Rune-Experiments.zip"
echo "Built $app and dist/Rune-Experiments.zip"
