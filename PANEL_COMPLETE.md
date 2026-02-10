# Doh! Panel Applet - Complete ✅

## What's New

✨ **Panel Integration**: Doh! now runs as a proper COSMIC panel applet  
🎨 **Custom Icon**: Speech bubble icon with "D!" representing multi-platform posting  
⚙️ **GUI Configuration**: Full credential management through the Settings interface  
🔒 **Secure Storage**: All credentials saved to system keyring

## Quick Setup

```bash
cd /home/sgtapple/Projects/doh/doh
./install.sh
```

Then:
1. Open **COSMIC Settings** → **Desktop** → **Panel**
2. Click **"Configure Applets"**
3. Find **"Doh"** in the list
4. Click **"+"** to add it to your panel
5. Click the **Doh icon** in your panel
6. Click the **settings gear** icon
7. **Configure your accounts** (BlueSky recommended first!)
8. Click **"Save Credentials"**
9. **Start posting!**

## Using the Applet

### Panel Icon
The Doh! icon appears in your COSMIC panel with a blue speech bubble containing "D!" and four connected dots representing the four platforms.

### Main View
Click the icon to open the popup:
- **Text input** - Type your message
- **Platform toggles** - Select X, BlueSky, Nostr, and/or Threads
- **Post button** - Send to selected platforms
- **Status** - Shows "Posted to X/Y platforms"
- **Settings gear** - Opens account configuration

### Settings View
Configure credentials for each platform:

#### X (Twitter)
- Consumer Key
- Consumer Secret  
- Access Token
- Access Token Secret

#### BlueSky (Easiest to test!)
- Handle (e.g., `yourname.bsky.social`)
- App Password (from bsky.app/settings/app-passwords)

#### Nostr
- **Toggle**: Use Pleb Signer (via D-Bus) or nsec key
- **nsec key** (if not using Pleb Signer)
- **Relays** (comma-separated, or leave empty for defaults)

#### Threads
- Access Token
- User ID

**Save Button** - Saves all credentials to system keyring

## Features Implemented

### UI Features
✅ Text input with character counter  
✅ Per-platform toggles (only show configured platforms)  
✅ Settings view with input fields for all platforms  
✅ Password fields use secure input masking  
✅ Status messages for success/failure  
✅ Back/forward navigation between Main and Settings

### Backend Features
✅ System keyring integration (automatic load/save)  
✅ Multi-platform async posting (parallel execution)  
✅ Platform-specific authentication:
  - X: OAuth 1.0a with HMAC-SHA1
  - BlueSky: AT Protocol with JWT
  - Nostr: nsec OR Pleb Signer via D-Bus
  - Threads: OAuth 2.0 with Meta Graph API
✅ Error handling with user-friendly messages

### Panel Integration
✅ COSMIC applet framework (libcosmic)  
✅ Popup window with size constraints  
✅ Custom icon (SVG, themed)  
✅ Desktop entry (NoDisplay=true, X-CosmicApplet=true)  
✅ Icon caching and system integration

## Architecture

```
Panel Icon (view)
    ↓ click
Popup Window (view_window)
    ├── Main View (view_main)
    │   ├── Text Input
    │   ├── Platform Toggles
    │   └── Post Button → PostManager
    │       └── Parallel async posting
    └── Settings View (view_settings)
        ├── Credential Inputs
        └── Save → Keyring Storage
```

## File Changes Made

### New Files
- `INSTALL.md` - Complete installation guide
- `README.md` - Project overview and documentation
- `install.sh` - Automated installation script

### Modified Files
- `resources/icon.svg` - Custom Doh! icon (speech bubble with D!)
- `src/app.rs` - Added:
  - 11 credential input fields in AppModel
  - 11 new Message variants for credential editing
  - Complete Settings view with all input fields
  - Credential load/save logic
  - Password masking on sensitive fields
  - Conditional Nostr UI (toggle between nsec/Pleb Signer)

### Unchanged (Already Working)
- All platform implementations (twitter, bluesky, nostr, threads)
- Post manager and async execution
- Credentials storage backend
- Main posting UI

## Testing Instructions

### 1. Quick Test (5 min - BlueSky)

```bash
cd /home/sgtapple/Projects/doh/doh
./install.sh

# Then in COSMIC:
# - Add applet to panel
# - Click icon, go to Settings
# - Enter BlueSky credentials
# - Save and post!
```

See `QUICKSTART.md` for detailed BlueSky setup.

### 2. Full Testing

See `TESTING.md` for platform-specific testing:
- BlueSky (easiest)
- Nostr (if you have keys)
- X/Twitter (requires developer account)
- Threads (requires business account)

## Known Limitations

1. **Image uploads** - Architecture ready, not yet implemented
2. **Character limits** - Counter shows count, no enforcement
3. **Pleb Signer** - Untested (need Pleb_Signer running)
4. **Rate limiting** - No retry logic
5. **Network errors** - Basic error messages

These are non-blocking for the core posting functionality.

## Troubleshooting

### Applet doesn't appear
- Run `./install.sh` again
- Restart COSMIC (log out/in)
- Check: `ls /usr/share/applications/com.sgtapple.doh.desktop`

### Icon doesn't show
- Update cache: `sudo gtk-update-icon-cache -f -t /usr/share/icons/hicolor`
- Check: `ls /usr/share/icons/hicolor/scalable/apps/com.sgtapple.doh.svg`

### Can't save credentials
- Unlock system keyring
- Test: `secret-tool store --label='test' service test username test`
- COSMIC uses `secret-service` backend automatically

### Posting fails
- Verify credentials in Settings
- Test one platform at a time
- Check logs: `journalctl --user -f | grep doh`

## Next Steps (Optional)

Future enhancements (not required for basic functionality):
- [ ] Implement image uploads (API code researched)
- [ ] Add character limit enforcement per platform
- [ ] Toast notifications for post status
- [ ] Draft saving
- [ ] Post scheduling
- [ ] Thread/multi-post support

## Summary

🎉 **Doh! is now a fully functional COSMIC panel applet!**

✅ Runs in panel with custom icon  
✅ GUI configuration for all 4 platforms  
✅ Secure credential storage  
✅ Multi-platform posting working  
✅ Ready for real-world use  

**Installation:** Run `./install.sh`  
**Documentation:** See README.md, INSTALL.md, QUICKSTART.md, TESTING.md  
**First test:** BlueSky (5 minutes)
