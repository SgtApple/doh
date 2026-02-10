#!/bin/bash
# Debug script to test posting

echo "=== Doh Debug Test ==="
echo ""
echo "1. Checking credentials in keyring..."
CREDS=$(secret-tool search service com.sgtapple.doh 2>&1)
if [ -z "$CREDS" ]; then
    echo "❌ No credentials found in keyring!"
    echo "   Please configure accounts in the applet Settings"
else
    echo "✅ Credentials found"
    echo "$CREDS"
fi

echo ""
echo "2. Running applet with debug output..."
echo "   (Check terminal for [Credentials] and [BlueSky]/[Nostr] messages)"
echo ""

RUST_LOG=debug /usr/bin/doh 2>&1 | grep -E "\[Credentials\]|\[BlueSky\]|\[Nostr\]|Error|error" &
DOH_PID=$!

echo "Doh running with PID: $DOH_PID"
echo "Press Ctrl+C to stop"

wait $DOH_PID
