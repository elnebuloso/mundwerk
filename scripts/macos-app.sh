#!/usr/bin/env bash
# Wraps the macOS binary in Mundwerk.app and zips it.
# Usage: scripts/macos-app.sh <binary> <version> <output.zip>
set -euo pipefail

binary=$1
version=${2:-0.0.0}
output=$3

work=$(mktemp -d)
app="$work/Mundwerk.app"
mkdir -p "$app/Contents/MacOS" "$app/Contents/Resources"
cp "$binary" "$app/Contents/MacOS/mundwerk"

# Dock and Finder take the icon from the bundle; the 512 px logo is the largest size, so no 1024 px variant.
iconset="$work/Mundwerk.iconset"
mkdir "$iconset"
logo="$(dirname "$0")/../logo.png"
for size in 16 32 128 256 512; do
  sips -z $size $size "$logo" --out "$iconset/icon_${size}x${size}.png" >/dev/null
done
for size in 16 32 128 256; do
  sips -z $((size * 2)) $((size * 2)) "$logo" --out "$iconset/icon_${size}x${size}@2x.png" >/dev/null
done
iconutil -c icns "$iconset" -o "$app/Contents/Resources/Mundwerk.icns"

# Without NSMicrophoneUsageDescription macOS refuses the app access to the microphone.
cat > "$app/Contents/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>CFBundleName</key><string>Mundwerk</string>
	<key>CFBundleDisplayName</key><string>Mundwerk</string>
	<key>CFBundleIdentifier</key><string>io.github.elnebuloso.mundwerk</string>
	<key>CFBundleExecutable</key><string>mundwerk</string>
	<key>CFBundleIconFile</key><string>Mundwerk</string>
	<key>CFBundlePackageType</key><string>APPL</string>
	<key>CFBundleShortVersionString</key><string>$version</string>
	<key>CFBundleVersion</key><string>$version</string>
	<key>LSMinimumSystemVersion</key><string>${MACOSX_DEPLOYMENT_TARGET:-13.0}</string>
	<key>NSHighResolutionCapable</key><true/>
	<key>NSMicrophoneUsageDescription</key><string>Mundwerk nimmt deine Sprache auf und erkennt sie lokal auf diesem Mac.</string>
</dict>
</plist>
PLIST

# Apple Silicon runs no unsigned code; an ad-hoc signature over the whole bundle (no Apple account)
# lets Gatekeeper offer "Open Anyway" instead of calling the app damaged.
codesign --force --sign - "$app"
codesign --verify --strict "$app"

# ditto keeps the bundle's signature and attributes intact, unlike plain zip.
ditto -c -k --keepParent "$app" "$output"
rm -rf "$work"
