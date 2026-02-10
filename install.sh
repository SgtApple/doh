#!/bin/bash
# Installation script for Doh! applet

set -e

echo "🚀 Installing Doh! applet..."

# Install binary
echo "📦 Installing binary to /usr/bin/doh..."
sudo install -Dm755 target/release/doh /usr/bin/doh

# Install icon
echo "🎨 Installing icon..."
sudo install -Dm644 resources/icon.svg /usr/share/icons/hicolor/scalable/apps/com.sgtapple.doh.svg

# Install desktop file
echo "📄 Installing desktop file..."
sudo install -Dm644 resources/app.desktop /usr/share/applications/com.sgtapple.doh.desktop

# Update icon cache
echo "🔄 Updating icon cache..."
sudo gtk-update-icon-cache -f -t /usr/share/icons/hicolor 2>/dev/null || true

echo ""
echo "✅ Installation complete!"
echo ""
echo "📋 Next steps:"
echo "1. Open COSMIC Settings → Desktop → Panel"
echo "2. Click 'Configure Applets'"
echo "3. Find 'Doh' and click '+' to add it"
echo "4. Click the Doh icon in your panel"
echo "5. Configure your accounts in Settings"
echo ""
echo "📚 See INSTALL.md for detailed instructions"
echo "🚀 See QUICKSTART.md for a 5-minute test with BlueSky"
