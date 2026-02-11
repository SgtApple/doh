#!/bin/bash
# Build Debian package for Doh!

set -e

VERSION="0.1.4"
PACKAGE_NAME="doh"
ARCH="amd64"

echo "🔨 Building Doh! v${VERSION}..."

# Build release binary
echo "📦 Building release binary..."
cargo build --release

# Create package directory structure
BUILD_DIR="/tmp/${PACKAGE_NAME}-${VERSION}"
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR"/{DEBIAN,usr/bin,usr/share/applications,usr/share/icons/hicolor/scalable/apps}

# Create DEBIAN/control file
cat > "$BUILD_DIR/DEBIAN/control" << EOF
Package: doh
Version: ${VERSION}
Section: utils
Priority: optional
Architecture: ${ARCH}
Maintainer: sgtapple <sgtapple@users.noreply.github.com>
Description: Multi-platform social media posting applet for COSMIC
 Doh! is a beautiful COSMIC desktop applet that allows you to post
 to multiple social media platforms simultaneously.
 .
 Supported platforms:
  - X (Twitter)
  - BlueSky
  - Nostr
  - Mastodon
 .
 Features secure credential storage, image uploads, and an elegant
 interface that integrates seamlessly with the COSMIC desktop.
EOF

# Copy files
echo "📋 Copying files..."
cp target/release/doh "$BUILD_DIR/usr/bin/"
cp com.sgtapple.doh.desktop "$BUILD_DIR/usr/share/applications/"
cp resources/icon.svg "$BUILD_DIR/usr/share/icons/hicolor/scalable/apps/com.sgtapple.doh.svg"

# Set permissions
chmod 755 "$BUILD_DIR/usr/bin/doh"
chmod 644 "$BUILD_DIR/usr/share/applications/com.sgtapple.doh.desktop"
chmod 644 "$BUILD_DIR/usr/share/icons/hicolor/scalable/apps/com.sgtapple.doh.svg"

# Build package
echo "📦 Building .deb package..."
dpkg-deb --build "$BUILD_DIR" "${PACKAGE_NAME}_${VERSION}_${ARCH}.deb"

# Cleanup
rm -rf "$BUILD_DIR"

echo "✅ Package built: ${PACKAGE_NAME}_${VERSION}_${ARCH}.deb"
echo ""
echo "To install: sudo dpkg -i ${PACKAGE_NAME}_${VERSION}_${ARCH}.deb"
