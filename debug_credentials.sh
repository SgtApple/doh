#!/bin/bash
# Debug helper - manually check and set credentials

echo "=== Doh Credentials Debug ==="
echo ""

echo "1. Checking if credentials exist in keyring:"
CREDS=$(secret-tool lookup service com.sgtapple.doh type credentials 2>&1)
if [ -z "$CREDS" ]; then
    echo "   ❌ NO CREDENTIALS FOUND"
    echo ""
    echo "2. Let's test saving manually..."
    
    TEST_CREDS='{"nostr_use_pleb_signer":true,"nostr_relays":[],"twitter_consumer_key":null,"twitter_consumer_secret":null,"twitter_access_token":null,"twitter_access_secret":null,"bluesky_handle":null,"bluesky_app_password":null,"nostr_nsec":null,"nostr_image_host_url":null,"threads_access_token":null,"threads_user_id":null}'
    
    echo "$TEST_CREDS" | secret-tool store --label="Doh Credentials" service com.sgtapple.doh type credentials
    
    echo "   Saved test credentials with Pleb_Signer enabled"
    echo ""
    echo "3. Verifying save..."
    VERIFY=$(secret-tool lookup service com.sgtapple.doh type credentials 2>&1)
    if [ -n "$VERIFY" ]; then
        echo "   ✅ Credentials saved successfully!"
        echo "   Content: $VERIFY"
    else
        echo "   ❌ Failed to save"
    fi
else
    echo "   ✅ Credentials found:"
    echo "   $CREDS"
    echo ""
    echo "   Checking Pleb_Signer status..."
    if echo "$CREDS" | grep -q '"nostr_use_pleb_signer":true'; then
        echo "   ✅ Pleb_Signer is ENABLED"
    else
        echo "   ❌ Pleb_Signer is DISABLED"
    fi
fi

echo ""
echo "4. Checking if Pleb_Signer is running:"
if dbus-send --session --print-reply --dest=org.freedesktop.DBus /org/freedesktop/DBus org.freedesktop.DBus.ListNames 2>/dev/null | grep -q "com.plebsigner.Signer"; then
    echo "   ✅ Pleb_Signer is running on D-Bus"
else
    echo "   ❌ Pleb_Signer is NOT running"
fi

echo ""
echo "5. Now try:"
echo "   - Restart doh: kill <PID> && /usr/bin/doh"
echo "   - Try posting with Nostr enabled"
echo "   - Watch logs: journalctl --user -f | grep '\[Nostr\]'"
