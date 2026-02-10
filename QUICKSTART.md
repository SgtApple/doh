# 🚀 Doh! Quick Start - 5 Minute Test

## Fastest Way to Test (BlueSky)

### 1. Get BlueSky App Password (2 minutes)

1. Go to https://bsky.app/settings/app-passwords
2. Click "Add App Password"
3. Name it: "Doh Test"
4. Click "Create"
5. **COPY THE PASSWORD** (looks like: `xxxx-xxxx-xxxx-xxxx`)

### 2. Set Up Credentials (1 minute)

```bash
cd /home/sgtapple/Projects/doh/doh
python3 setup_credentials.py
```

**Answer the prompts:**
- Configure BlueSky? **y**
- Handle: **YOUR_HANDLE.bsky.social** (e.g., alice.bsky.social)
- App password: **paste the password from step 1**
- Configure Nostr? **n**
- Configure X/Twitter? **n**
- Configure Threads? **n**

✅ Credentials saved!

### 3. Run & Test (2 minutes)

```bash
./test.sh
```

**OR**

```bash
just run
```

### 4. Post Your First Message

In the Doh! window:

1. Type: **"Testing Doh! applet 🚀"**
2. Toggle **BlueSky** ON (should show ✓ next to it)
3. Click **"Post"**
4. Wait for status message: "Posted to 1/1 platforms"

### 5. Verify It Worked

Go to your BlueSky profile and see your post!
- https://bsky.app/profile/YOUR_HANDLE.bsky.social

🎉 **Success!** You've posted to BlueSky using Doh!

---

## Test Multiple Platforms

Once BlueSky works, add more:

### Add Nostr (if you have a key)

```bash
python3 setup_credentials.py
```

- Configure Nostr? **y**
- Use Pleb_Signer? **n**
- Nostr nsec key: **nsec1yourkey...**
- Use default relays? **y**

Now you can post to both BlueSky and Nostr at once!

### Verify Multi-Platform

1. Run: `./test.sh`
2. Type a message
3. Enable **both BlueSky AND Nostr**
4. Click Post
5. Check both:
   - BlueSky: https://bsky.app
   - Nostr: https://primal.net (search for your npub)

---

## Troubleshooting

### "Not configured" next to platform

**Fix:**
```bash
python3 setup_credentials.py
# Re-enter credentials for that platform
```

### Applet won't start

**Fix:**
```bash
just build-release
./test.sh
```

### Can't find credentials

**Check:**
```bash
secret-tool lookup application com.sgtapple.doh type credentials
```

If nothing shows, run `python3 setup_credentials.py` again.

---

## What You Built

A fully functional social media posting applet that:
- ✅ Posts to 4 platforms (BlueSky, Nostr, X, Threads)
- ✅ Secure credential storage
- ✅ Multi-platform simultaneous posting
- ✅ COSMIC desktop integration

**Congrats!** 🎉
