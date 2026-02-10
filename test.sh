#!/bin/bash
# Quick test script for Doh! applet

set -e

cd /home/sgtapple/Projects/doh/doh

echo "╔════════════════════════════════════════════════╗"
echo "║         Doh! Applet - Quick Test              ║"
echo "╚════════════════════════════════════════════════╝"
echo ""

# Check if built
if [ ! -f "target/release/doh" ]; then
    echo "⚠️  Release binary not found. Building..."
    just build-release
fi

# Check credentials
echo "📋 Checking credentials..."
if secret-tool lookup application com.sgtapple.doh type credentials > /dev/null 2>&1; then
    echo "✅ Credentials found in keyring"
    
    # Parse and show configured platforms
    CREDS=$(secret-tool lookup application com.sgtapple.doh type credentials)
    echo ""
    echo "Configured platforms:"
    
    if echo "$CREDS" | grep -q '"bluesky_handle"' | grep -v 'null'; then
        HANDLE=$(echo "$CREDS" | grep -o '"bluesky_handle":"[^"]*"' | cut -d'"' -f4)
        if [ -n "$HANDLE" ] && [ "$HANDLE" != "null" ]; then
            echo "  ✅ BlueSky: $HANDLE"
        fi
    fi
    
    if echo "$CREDS" | grep -q '"nostr_nsec"' | grep -v 'null'; then
        echo "  ✅ Nostr: configured"
    fi
    
    if echo "$CREDS" | grep -q '"twitter_consumer_key"' | grep -v 'null'; then
        echo "  ✅ X/Twitter: configured"
    fi
    
    if echo "$CREDS" | grep -q '"threads_access_token"' | grep -v 'null'; then
        echo "  ✅ Threads: configured"
    fi
else
    echo "❌ No credentials found!"
    echo ""
    echo "Run this first:"
    echo "  python3 setup_credentials.py"
    exit 1
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "🚀 Starting Doh! applet..."
echo ""
echo "What to do:"
echo "  1. Click the Doh! icon (or use the window)"
echo "  2. Type a test message"
echo "  3. Enable platform(s) you want to test"
echo "  4. Click 'Post'"
echo "  5. Check your social media to verify!"
echo ""
echo "Press Ctrl+C to stop when done testing"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Run the applet
exec target/release/doh
