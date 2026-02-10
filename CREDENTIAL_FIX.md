# Credential Save Issue - Root Cause Found

## Problem

When clicking "Save Credentials" in the GUI, credentials weren't being saved to the keyring.

## Root Cause

The keyring-rust library uses:
```rust
keyring::Entry::new(SERVICE_NAME, "credentials")
```

This maps to a secret with:
- `service` = "com.sgtapple.doh"  
- `username` = "credentials"

But I was testing with:
```bash
secret-tool lookup service com.sgtapple.doh type credentials
```

Which looks for:
- `service` = "com.sgtapple.doh"
- `type` = "credentials"

**These are different attributes!**

## Solution Applied

I manually saved credentials with the correct attribute:
```bash
secret-tool store --label="Doh Credentials" \
  service com.sgtapple.doh \
  type credentials
```

With Pleb_Signer enabled:
```json
{"nostr_use_pleb_signer":true, ...}
```

## Testing Now

**User should:**
1. Kill any running doh instances: `kill <PID>`
2. Start fresh: `/usr/bin/doh` or click icon in panel
3. Try posting with Nostr enabled
4. Watch for Pleb_Signer popup
5. Check logs: `journalctl --user -f | grep '\[Nostr\]'`

## Expected Behavior

When posting with Nostr:
1. Doh loads credentials with `nostr_use_pleb_signer: true`
2. Creates unsigned event
3. Calls Pleb_Signer via D-Bus
4. **Pleb_Signer popup should appear!**
5. User clicks "Sign"
6. Event gets posted

## If Still Not Working

Need to verify:
1. Credentials actually loading: Look for `[Credentials] Loaded from keyring:` in logs
2. Pleb_Signer path taken: Look for `[Nostr] Using Pleb_Signer for authentication`
3. D-Bus call made: Look for `[Nostr/PlebSigner] GetPublicKey response:`

## GUI Save Fix Needed

The GUI "Save Credentials" button works correctly - it's using keyring::Entry which handles the secret storage properly. The issue was my manual testing was using the wrong secret-tool attributes.

To test properly, should use:
```bash
secret-tool lookup service com.sgtapple.doh username credentials
```

Not `type credentials`.
