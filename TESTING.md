# Testing Doh! - Step-by-Step Guide

## Quick Test (BlueSky - Easiest)

BlueSky is the easiest platform to test because it requires minimal setup.

### Step 1: Get BlueSky App Password

1. **Go to BlueSky**: https://bsky.app
2. **Log in** to your account (or create one - it's free!)
3. **Go to Settings** → **App Passwords**: https://bsky.app/settings/app-passwords
4. **Create new app password**:
   - Name it: "Doh Testing"
   - Click "Create App Password"
   - **Copy the password** (you won't see it again!)

### Step 2: Configure Credentials

```bash
cd /home/sgtapple/Projects/doh/doh
python3 setup_credentials.py
```

**Follow the prompts:**
```
Configure BlueSky? (y/n): y
BlueSky handle (e.g., user.bsky.social): YOUR_HANDLE.bsky.social
BlueSky app password: [paste the password you copied]

Configure Nostr? (y/n): n
Configure X/Twitter? (y/n): n
Configure Threads? (y/n): n
```

### Step 3: Run the Applet

```bash
just run
```

**Expected behavior:**
- A window should open (or if on COSMIC, the applet appears in the panel)
- You'll see the posting interface

### Step 4: Make a Test Post

1. **Click the Doh! icon** (if it's in the panel) or use the window
2. **Type a test message**: "Testing Doh! applet - ignore this post 🚀"
3. **Enable only BlueSky** (toggle it on)
4. **Click "Post"**
5. **Wait** for the status message

**Expected result:**
- Status should show: "Posted to 1/1 platforms"
- Or specific success/error message

### Step 5: Verify on BlueSky

1. **Go to your BlueSky profile**: https://bsky.app/profile/YOUR_HANDLE.bsky.social
2. **Check your posts** - you should see your test post!

---

## Testing Other Platforms

### Testing Nostr (Medium Difficulty)

**Requirements:**
- A Nostr private key (nsec)
- OR Pleb_Signer installed

**Setup:**

```bash
python3 setup_credentials.py
```

```
Configure Nostr? (y/n): y
Use Pleb_Signer? (y/n): n
Nostr nsec key: nsec1your_private_key_here
Use default relays? (y/n): y
```

**Test:**
1. Run applet: `just run`
2. Enable Nostr toggle
3. Post test message
4. Verify on any Nostr client (Damus, Amethyst, Primal, etc.)

**Verify:**
- Check your Nostr profile on https://primal.net or https://iris.to
- Search for your public key (npub) to see the post

---

### Testing X/Twitter (Hard - Requires API Access)

**Requirements:**
- Twitter Developer Account
- App created in developer portal
- API keys (Consumer Key/Secret, Access Token/Secret)

**Setup:**

1. **Get API keys** from https://developer.twitter.com/en/portal/dashboard
2. Run credential setup:

```bash
python3 setup_credentials.py
```

```
Configure X/Twitter? (y/n): y
Consumer Key (API Key): YOUR_CONSUMER_KEY
Consumer Secret (API Secret): YOUR_CONSUMER_SECRET
Access Token: YOUR_ACCESS_TOKEN
Access Token Secret: YOUR_ACCESS_SECRET
```

**Test:**
1. Run applet
2. Enable X toggle
3. Post test message
4. Check your Twitter profile

**Note**: Twitter API requires app approval and may have restrictions.

---

### Testing Threads (Hard - Requires Business Account)

**Requirements:**
- Instagram business/creator account
- Meta Developer account
- Threads API access approved

**Setup:**

This is the most complex because it requires:
1. Meta developer app
2. OAuth 2.0 flow to get access token
3. Threads API access (may require approval)

**For now, skip Threads testing unless you have enterprise access.**

---

## Testing Multi-Platform Posting

Once you have 2+ platforms configured:

### Step 1: Configure Multiple Platforms

```bash
python3 setup_credentials.py
```

Configure at least BlueSky and Nostr (easiest combo).

### Step 2: Post to Both

1. Run applet
2. Type: "Multi-platform test post from Doh! 🚀"
3. **Enable both BlueSky AND Nostr**
4. Click "Post"

### Step 3: Verify Both

- **BlueSky**: Check https://bsky.app/profile/YOUR_HANDLE
- **Nostr**: Check https://primal.net or your Nostr client

**Expected:** Same message appears on both platforms!

---

## Debugging Tips

### Check Credentials Saved

```bash
# Try to retrieve credentials
secret-tool lookup application com.sgtapple.doh type credentials
```

**Expected:** JSON with your credentials

### Run with Debug Output

```bash
cd /home/sgtapple/Projects/doh/doh
RUST_LOG=debug cargo run
```

**Expected:** Detailed logging of what's happening

### Check Build

```bash
just check
```

**Expected:** No errors, maybe some warnings

### Test Individual Platform

Modify the post in the applet to test one platform at a time:
1. Enable only ONE platform
2. Post
3. Check status message
4. Verify on that platform

---

## Common Issues

### "Not configured" for platform

**Problem:** Platform toggle is grayed out or shows "Not configured"

**Solution:**
```bash
# Re-run setup
python3 setup_credentials.py

# Or check credentials were saved
secret-tool lookup application com.sgtapple.doh type credentials
```

### Applet won't start

**Problem:** Error when running `just run`

**Solution:**
```bash
# Check build
cargo check

# Try debug build
cargo run
```

### Post shows "Failed to post"

**Problem:** Status shows error for specific platform

**BlueSky Solutions:**
- Verify you used APP PASSWORD, not main password
- Check handle format: `username.bsky.social`
- Try logging in to BlueSky web to verify account works

**Nostr Solutions:**
- Verify nsec key format (should start with `nsec1`)
- Check relay connectivity
- Try with default relays first

---

## Recommended Testing Order

1. ✅ **BlueSky** (easiest, 5 minutes)
   - Free account
   - Simple app password
   - Instant verification

2. ✅ **Nostr** (easy if you have a key, 10 minutes)
   - Generate key or use existing
   - Multiple clients to verify
   - Decentralized (may take a moment to propagate)

3. ⚠️ **X/Twitter** (hard, requires API access)
   - Need developer account
   - API approval process
   - Rate limits

4. ⚠️ **Threads** (hardest, requires business account)
   - Complex OAuth
   - Business account required
   - May need API approval

---

## Success Indicators

### ✅ Working Correctly

- Applet launches without errors
- UI shows all configured platforms with checkmarks
- Can type message
- Post button is clickable
- After posting, see "Posted to X/X platforms"
- Posts appear on platform websites/apps

### ❌ Not Working

- Applet crashes on launch
- Platforms show "Not configured" when they should be configured
- Post button grayed out
- Status shows "Failed" for all platforms
- No posts appear on platforms

---

## Quick Verification Script

Create a test script:

```bash
#!/bin/bash
# test_doh.sh

echo "=== Testing Doh! Applet ==="

echo "1. Checking build..."
cd /home/sgtapple/Projects/doh/doh
if cargo check --quiet; then
    echo "   ✅ Build OK"
else
    echo "   ❌ Build failed"
    exit 1
fi

echo "2. Checking credentials..."
if secret-tool lookup application com.sgtapple.doh type credentials > /dev/null 2>&1; then
    echo "   ✅ Credentials found"
else
    echo "   ❌ No credentials - run: python3 setup_credentials.py"
    exit 1
fi

echo "3. Checking which platforms configured..."
CREDS=$(secret-tool lookup application com.sgtapple.doh type credentials)

if echo "$CREDS" | grep -q '"bluesky_handle".*[^null]'; then
    echo "   ✅ BlueSky configured"
fi

if echo "$CREDS" | grep -q '"nostr_nsec".*[^null]'; then
    echo "   ✅ Nostr configured"
fi

echo ""
echo "Ready to test! Run: just run"
```

```bash
chmod +x test_doh.sh
./test_doh.sh
```

---

## Need Help?

If something doesn't work:

1. **Check the build**: `cargo check`
2. **Check credentials**: `secret-tool lookup application com.sgtapple.doh type credentials`
3. **Run with logging**: `RUST_LOG=debug cargo run`
4. **Start simple**: Test BlueSky only first
5. **Verify platform**: Log in to the platform website to ensure account works

---

**Ready to test? Start with BlueSky - it's the easiest!**

```bash
cd /home/sgtapple/Projects/doh/doh
python3 setup_credentials.py  # Configure BlueSky
just run                       # Test it!
```
