# Nostr Posting Troubleshooting Guide

## Common Nostr Issues

### 1. nsec Key Format

**Correct format**: `nsec1...` (followed by many characters)

Example: `nsec1abcd1234efgh5678...` (actual keys are much longer)

**Common mistakes**:
- Using npub (public key) instead of nsec (private key)
- Missing the "nsec1" prefix
- Extra whitespace or newlines
- Truncated key (too short)

### 2. How to Get Your nsec Key

If you don't have one yet:

**Using nak (command line)**:
```bash
# Install nak if needed
go install github.com/fiatjaf/nak@latest

# Generate new key
nak key generate

# This will output:
# Private key (nsec): nsec1...
# Public key (npub): npub1...
```

**Using Nostr clients**:
- **Damus** (iOS): Settings → Keys → Export nsec (copy it)
- **Primal** (Web): Settings → Keys → Show Private Key
- **Alby** (Extension): Settings → Keys → Export

⚠️ **SECURITY WARNING**: Your nsec is like a password. Never share it or post it publicly!

### 3. Relay Configuration

**Default relays** (used if you leave the field empty):
- wss://relay.damus.io
- wss://relay.nostr.band
- wss://nos.lol

**Custom relays** (comma-separated):
```
wss://relay.damus.io, wss://nostr.wine, wss://relay.snort.social
```

### 4. Testing Your nsec Key

To verify your nsec key is valid:

```bash
# Install nak if you haven't
go install github.com/fiatjaf/nak@latest

# Decode your key
nak key decode YOUR_NSEC_KEY_HERE

# Should output hex key and npub
```

### 5. Running Doh with Debug Output

To see exactly what's happening with Nostr:

```bash
# Kill any running instances
pkill doh

# Run with full debug output
RUST_LOG=debug /usr/bin/doh 2>&1 | grep -E "\[Nostr\]"
```

Then try posting and look for:
- `[Nostr] Keys loaded successfully` ✅
- `[Nostr] Failed to load keys` ❌
- `[Nostr] Event sent successfully` ✅
- `[Nostr] Failed to send event` ❌

### 6. Debug Messages Explained

**If you see**:
```
[Nostr] Error: nsec key is empty!
```
→ You didn't enter an nsec key. Go to Settings and enter one.

**If you see**:
```
[Nostr] Failed to parse nsec key
[Nostr] Hint: nsec should start with 'nsec1'
```
→ Your key format is wrong. Check it starts with `nsec1` and is complete.

**If you see**:
```
[Nostr] Failed to add relay: ...
```
→ Relay connection issue. Try different relays or check internet connection.

**If you see**:
```
[Nostr] Failed to send event: ...
```
→ Event was created but couldn't be sent. Check relay connections.

### 7. Quick Test Checklist

- [ ] nsec key starts with `nsec1`
- [ ] nsec key is complete (no truncation)
- [ ] No extra spaces before/after the key
- [ ] Pleb Signer toggle is OFF (unless you have Pleb Signer running)
- [ ] Clicked "Save Credentials"
- [ ] Saw "Credentials saved!" message
- [ ] Internet connection working
- [ ] At least one relay reachable

### 8. Viewing Your Post

After successful post, check:
- **Primal**: https://primal.net/home (login with your npub)
- **Damus**: Open app and check feed
- **Nostr Band**: https://nostr.band (search for your npub)

You need to convert nsec → npub to find your posts:
```bash
nak key public YOUR_NSEC_KEY_HERE
```

### 9. Pleb Signer Support

If you want to use Pleb Signer (more secure):
1. Install Pleb_Signer from https://github.com/PlebOne/Pleb_Signer
2. Make sure it's running
3. In Doh Settings, toggle "Use Pleb Signer"
4. Save credentials

Note: Pleb Signer integration is partially implemented but untested.

### 10. Still Not Working?

Run this debug command and paste the output:

```bash
pkill doh
RUST_LOG=debug /usr/bin/doh 2>&1 | tee /tmp/doh_debug.log &

# Then try posting through the UI
# After it fails, check the log:
cat /tmp/doh_debug.log | grep -E "\[Nostr\]|\[Credentials\]"
```

Common issues and fixes:
- **Empty nsec**: Enter your key in Settings
- **Wrong format**: Must start with `nsec1`
- **Can't paste**: Try Ctrl+Shift+V or middle-click, or type manually
- **Relays unreachable**: Try default relays (leave field empty)
- **Keys not saving**: Check keyring is unlocked

## Example Configuration

**Settings → Nostr Section**:

```
[ ] Use Pleb Signer (via D-Bus)

nsec key: nsec1abcd1234efgh5678ijkl9012mnop3456qrst7890uvwx1234yz567890
(your actual key will be longer)

Relays: (leave empty for defaults)
OR
wss://relay.damus.io, wss://relay.nostr.band, wss://nos.lol
```

Click **"Save Credentials"** - you should see "Credentials saved!"

Then go back to main view, type a message, enable Nostr toggle, and post!
