# Pleb_Signer Setup Required!

## Issue Found

When I tested Pleb_Signer, it returned:
```json
{"success":false,"error":"No keys configured"}
```

**You need to add a Nostr key to Pleb_Signer first!**

## How to Add a Key to Pleb_Signer

### Option 1: Generate New Key in Pleb_Signer

1. **Open Pleb_Signer** (click icon in system tray)
2. **Go to Keys tab**
3. **Click "Generate New Key"** or "Add Key"
4. **Give it a name** (e.g., "Main")
5. **Set as active** (check mark or star icon)

### Option 2: Import Existing nsec Key

1. **Open Pleb_Signer**
2. **Go to Keys tab**
3. **Click "Import Key"**
4. **Paste your nsec key** (starts with `nsec1...`)
5. **Give it a name**
6. **Set as active**

## Verify Key is Active

Run this to check:
```bash
dbus-send --session --print-reply \
  --dest=com.plebsigner.Signer \
  /com/plebsigner/Signer \
  com.plebsigner.Signer1.GetPublicKey
```

**Should return** (success):
```
string "{"success":true,"result":"\"YOUR_PUBKEY_HEX\"",...}"
```

**Not** (error):
```
string "{"success":false,"error":"No keys configured"}"
```

## After Adding Key

1. **Restart Doh** or click the panel icon
2. **Type a message**
3. **Enable Nostr toggle**
4. **Click Post**
5. **Pleb_Signer popup should appear!**

## Watch Logs

```bash
journalctl --user -f | grep '\[Nostr\]'
```

Should see:
- `[Credentials] Loaded from keyring: {...nostr_use_pleb_signer:true...}`
- `[Nostr] Using Pleb_Signer for authentication`
- `[Nostr/PlebSigner] GetPublicKey response: {"success":true...}`
- `[Nostr] Requesting signature from Pleb_Signer...`
- **Pleb_Signer popup!**
- `[Nostr] Event signed successfully`

## Troubleshooting

### Pleb_Signer not in system tray
- Launch it from applications menu
- Search for "Pleb Signer" or "pleb-signer"

### Can't find "Add Key" button
- Check the Keys/Identities tab
- Look for + button or "New" button
- Check Pleb_Signer documentation

### Still says "No keys configured"
- Make sure the key is marked as "Active"
- Check Pleb_Signer status in system tray
- Try restarting Pleb_Signer

## Current Status

✅ Credentials saved correctly with Pleb_Signer enabled  
✅ D-Bus connection working  
❌ **Need to add key to Pleb_Signer** ← DO THIS NOW  

Once you add a key to Pleb_Signer, everything should work!
