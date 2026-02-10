#!/bin/bash
# Pleb_Signer key verification and debugging

echo "═══════════════════════════════════════════════"
echo "         PLEB_SIGNER KEY CHECK"
echo "═══════════════════════════════════════════════"
echo ""

echo "1. Checking if Pleb_Signer is running..."
if ps aux | grep -i pleb | grep -v grep > /dev/null; then
    echo "   ✅ Pleb_Signer process found:"
    ps aux | grep -i pleb | grep -v grep
else
    echo "   ❌ Pleb_Signer is NOT running!"
    echo "   Please start it from your applications menu"
    exit 1
fi

echo ""
echo "2. Checking D-Bus service..."
if dbus-send --session --print-reply --dest=org.freedesktop.DBus /org/freedesktop/DBus org.freedesktop.DBus.ListNames 2>/dev/null | grep -q "com.plebsigner.Signer"; then
    echo "   ✅ D-Bus service registered"
else
    echo "   ❌ D-Bus service NOT registered"
    echo "   Try restarting Pleb_Signer"
    exit 1
fi

echo ""
echo "3. Checking IsReady status..."
READY=$(dbus-send --session --print-reply --dest=com.plebsigner.Signer /com/plebsigner/Signer com.plebsigner.Signer1.IsReady 2>&1 | grep boolean | awk '{print $2}')
if [ "$READY" == "true" ]; then
    echo "   ✅ Pleb_Signer is ready"
else
    echo "   ❌ Pleb_Signer is locked or not ready"
    echo "   Please unlock it"
    exit 1
fi

echo ""
echo "4. Listing keys..."
KEYS=$(dbus-send --session --print-reply --dest=com.plebsigner.Signer /com/plebsigner/Signer com.plebsigner.Signer1.ListKeys 2>&1 | grep string | cut -d'"' -f2)
echo "   Keys response: $KEYS"

if [ "$KEYS" == "[]" ]; then
    echo "   ❌ NO KEYS IN PLEB_SIGNER!"
    echo ""
    echo "═══════════════════════════════════════════════"
    echo "         PROBLEM FOUND!"
    echo "═══════════════════════════════════════════════"
    echo ""
    echo "Pleb_Signer has NO keys configured."
    echo ""
    echo "To fix:"
    echo "  1. Open Pleb_Signer (look in system tray)"
    echo "  2. Click the Pleb_Signer icon"
    echo "  3. Look for a Keys, Identities, or Accounts tab"
    echo "  4. Click 'Add Key' or '+' or 'Import'"
    echo "  5. Either:"
    echo "     - Generate new key, OR"
    echo "     - Import your nsec (paste it)"
    echo "  6. Save the key"
    echo "  7. Make sure it's marked as ACTIVE"
    echo "  8. Run this script again to verify"
    echo ""
    echo "If you can't find where to add keys:"
    echo "  - Check Pleb_Signer documentation"
    echo "  - Look for Settings/Preferences"
    echo "  - Try right-clicking the tray icon"
    echo ""
    exit 1
else
    echo "   ✅ Keys found: $KEYS"
fi

echo ""
echo "5. Getting public key..."
PUBKEY_RESPONSE=$(dbus-send --session --print-reply --dest=com.plebsigner.Signer /com/plebsigner/Signer com.plebsigner.Signer1.GetPublicKey 2>&1 | grep string | cut -d'"' -f2)
echo "   Response: $PUBKEY_RESPONSE"

if echo "$PUBKEY_RESPONSE" | grep -q '"success":false'; then
    echo "   ❌ GetPublicKey failed"
    ERROR=$(echo "$PUBKEY_RESPONSE" | grep -o '"error":"[^"]*"' | cut -d'"' -f4)
    echo "   Error: $ERROR"
    exit 1
else
    echo "   ✅ GetPublicKey succeeded"
    PUBKEY=$(echo "$PUBKEY_RESPONSE" | grep -o '"result":"[^"]*"' | cut -d'"' -f4)
    echo "   Public key: $PUBKEY"
fi

echo ""
echo "═══════════════════════════════════════════════"
echo "         ✅ ALL CHECKS PASSED!"
echo "═══════════════════════════════════════════════"
echo ""
echo "Pleb_Signer is configured correctly."
echo "Now try posting with Doh - it should work!"
echo ""
