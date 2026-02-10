# Debugging Posting Failures

## Issue Reported
User configured BlueSky and Nostr accounts via GUI but posting failed on both platforms.
Also reported: Cannot paste credentials into input fields.

## Investigation Steps Taken

### 1. Added Debug Logging
Added `eprintln!` statements to track:
- Credential loading from keyring
- Credential saving to keyring  
- Platform posting attempts with full error messages

### 2. Issues Identified

#### A. Paste Functionality
The `.password()` modifier on text_input widgets should NOT block paste in libcosmic/iced. This is likely a system clipboard issue or COSMIC-specific behavior.

**Workaround for user**: Try:
- Ctrl+Shift+V (alternative paste)
- Middle-click paste (Linux)
- Typing credentials manually (for now)

#### B. Credentials May Not Be Saving
When checking keyring with `secret-tool search service com.sgtapple.doh`, no results found.

**Possible causes**:
1. Keyring not unlocked
2. Permission issue with secret-service
3. save() method failing silently (though status should show error)

### 3. Debug Output Added

**In credentials.rs**:
- Load: Prints JSON loaded from keyring OR "No entry found"
- Save: Prints JSON being saved AND success/failure

**In post_manager.rs**:
- BlueSky: Prints success/error/exception with full details
- Nostr: Prints success/error/exception with full details

### 4. Files Modified

- `src/credentials.rs` - Added eprintln! for load/save operations
- `src/post_manager.rs` - Added eprintln! for posting results, changed to `mut platform`
- Binary rebuilt and installed

## Next Steps for User

### Test 1: Check if Credentials Are Saving

1. Open the Doh applet
2. Go to Settings
3. Enter BlueSky credentials:
   - Handle: `yourname.bsky.social`
   - Password: your app password
4. Click "Save Credentials"
5. Check terminal/logs:
   ```bash
   journalctl --user -f | grep Credentials
   ```
   Should see: `[Credentials] Saving to keyring: {...}`

### Test 2: Check Credentials in Keyring

```bash
secret-tool search service com.sgtapple.doh
```

Should show the saved JSON.

### Test 3: Test Posting with Debug Output

```bash
# Kill any running doh instances
pkill -9 doh

# Run with debug output
RUST_LOG=debug /usr/bin/doh 2>&1 | grep -E "\[Credentials\]|\[BlueSky\]|\[Nostr\]"
```

Then try posting and watch for error messages.

### Test 4: Paste Workarounds

If paste doesn't work with Ctrl+V, try:
- **Ctrl+Shift+V**
- **Middle-click** (select text, then middle-click in field)
- **Type manually** (for initial testing)

## Expected Debug Output

### On App Start:
```
[Credentials] Loaded from keyring: {"bluesky_handle":"user.bsky.social","bluesky_app_password":"xxxx-xxxx-xxxx-xxxx",...}
```
OR
```
[Credentials] No entry found in keyring, using defaults
```

### On Save:
```
[Credentials] Saving to keyring: {"bluesky_handle":"user.bsky.social",...}
[Credentials] Successfully saved to keyring
```

### On Post:
```
[BlueSky] Success: Posted successfully
```
OR
```
[BlueSky] Exception: Invalid credentials
```

## Possible Fixes Needed

### If Credentials Not Saving:
1. Check keyring backend: `echo $DESKTOP_SESSION`
2. Check secret-service: `systemctl --user status gnome-keyring-daemon`
3. Unlock keyring: Run any app that uses keyring first

### If Paste Not Working:
Need to investigate libcosmic text_input widget:
- Check if there's an `.enable_paste()` or similar method
- Check if it's a Wayland clipboard issue
- May need to file issue with libcosmic project

### If Posting Fails After Credentials Save:
Check specific error in debug output:
- **"Not authenticated"** → Login failing
- **"Invalid credentials"** → Wrong password
- **"Network error"** → Connectivity issue
- **"Invalid nsec key"** → Nostr key format wrong

## Code Changes Summary

1. **credentials.rs**: Added debug prints for load/save
2. **post_manager.rs**: Added debug prints for posting, made platforms mutable
3. **Rebuild & reinstall**: Updated /usr/bin/doh

## User Action Required

Please run the debug tests above and report:
1. What you see when you click "Save Credentials"
2. Output of `secret-tool search service com.sgtapple.doh`
3. Any `[Credentials]` or `[BlueSky]` messages in logs when posting
4. Exact error message shown in the UI

This will help identify if it's:
- Keyring issue (not saving)
- Authentication issue (wrong credentials)  
- Network issue (can't reach server)
- Paste issue (separate UI problem)
