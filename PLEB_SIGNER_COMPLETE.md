# Pleb_Signer Integration - Complete! ✅

## What Was Implemented

Full Pleb_Signer support via D-Bus for secure Nostr signing.

### Features:

**1. D-Bus Integration**
- `GetPublicKey()` - Gets user's public key from Pleb_Signer
- `SignEvent()` - Signs events using Pleb_Signer (keys never exposed to Doh)
- Automatic detection if Pleb_Signer is running

**2. Dual Authentication Modes**
- ✅ **nsec key** (direct signing)
- ✅ **Pleb_Signer** (secure external signing via D-Bus)

**3. Debug Output**
All steps logged with `[Nostr]` and `[Nostr/PlebSigner]` prefixes.

## How It Works

### With Pleb_Signer Enabled:

1. User toggles "Use Pleb Signer" in Settings
2. Clicks "Save Credentials"
3. Types a post and clicks Post
4. Doh creates unsigned Nostr event
5. Doh calls Pleb_Signer via D-Bus to sign it
6. Pleb_Signer shows UI popup asking user to confirm
7. User approves → event gets signed
8. Doh sends signed event to relays
9. Success!

### Security Benefits:

- **Private keys never leave Pleb_Signer**
- **User confirms each post** (popup)
- **Keys stored in OS keyring** by Pleb_Signer
- **Doh never sees your nsec**

## Testing

### Verify Pleb_Signer is Running:

```bash
dbus-send --session --print-reply \
  --dest=com.plebsigner.Signer \
  /com/plebsigner/Signer \
  com.plebsigner.Signer1.IsReady
```

Should return: `boolean true`

### Test Posting with Doh:

1. Open Doh applet
2. Go to Settings
3. Nostr section:
   - ✅ Enable "Use Pleb Signer (via D-Bus)"
   - Leave nsec key empty
   - Relays: (leave empty or add custom)
4. Click "Save Credentials"
5. Go back to main view
6. Type a message
7. Enable Nostr toggle
8. Click "Post"
9. **Pleb_Signer popup should appear!**
10. Click "Sign" in Pleb_Signer
11. Watch terminal for success

### Debug Output to Watch For:

```bash
# Run in terminal to see logs:
journalctl --user -f | grep "\[Nostr"
```

**Expected messages:**
```
[Nostr] Starting post attempt...
[Nostr] Using Pleb_Signer for authentication
[Nostr] Posting via Pleb_Signer...
[Nostr/PlebSigner] GetPublicKey response: {...}
[Nostr] Requesting signature from Pleb_Signer...
[Nostr/PlebSigner] SignEvent response: {...}
[Nostr] Event signed successfully
[Nostr] Adding 3 relays...
[Nostr] Connecting to relays...
[Nostr] Sending signed event to relays...
[Nostr] Event sent successfully: ...
```

## Troubleshooting

### "Pleb_Signer not running"

**Check:**
```bash
ps aux | grep -i pleb
```

**Start Pleb_Signer:**
- It should auto-start with your session
- Or launch from applications menu
- Look for icon in system tray

### "D-Bus call failed"

Make sure Pleb_Signer is:
1. Running (check system tray)
2. Unlocked (enter password if needed)
3. Has at least one key configured

### "Failed to sign event"

- Pleb_Signer popup appeared but you clicked "Cancel"
- OR Pleb_Signer denied the request
- Check Pleb_Signer logs/settings

### No Popup Appears

- Pleb_Signer might be set to auto-approve
- Check Pleb_Signer settings for "com.sgtapple.doh"
- Look in system tray for notifications

## Comparison: nsec vs Pleb_Signer

| Feature | nsec key | Pleb_Signer |
|---------|----------|-------------|
| **Security** | Key in keyring | Key never exposed |
| **User Confirmation** | No | Yes (popup) |
| **Setup** | Paste nsec | Just toggle |
| **Speed** | Faster | Slightly slower |
| **Mobile** | Yes | Linux only |
| **Best for** | Quick posts | High security |

## Implementation Details

### D-Bus Service:
- **Service**: `com.plebsigner.Signer`
- **Object**: `/com/plebsigner/Signer`
- **Interface**: `com.plebsigner.Signer1`

### Methods Called:
```rust
// 1. Get public key
GetPublicKey() → String (JSON response)

// 2. Sign event
SignEvent(event_json: &str, app_id: &str) → String (JSON response)
```

### Response Format:
```json
{
  "success": true,
  "result": "\"<double-encoded-data>\"",
  "error": null
}
```

Note: Result is double-encoded JSON that needs parsing twice.

## Files Modified

- `src/platforms/nostr.rs`:
  - Added `get_pleb_signer_pubkey()` function
  - Added `sign_event_with_pleb_signer()` function
  - Added `post_with_pleb_signer()` method
  - Split `post()` into nsec vs Pleb_Signer paths
  - Extensive debug logging throughout

## Next Steps

1. **Test it!** Try posting with Pleb_Signer enabled
2. **Report results** - Did the popup appear? Did it post successfully?
3. **Check logs** - Use `journalctl --user -f | grep "\[Nostr"`

The implementation is complete and ready to use! 🎉
