# Mastodon Setup Guide for Doh!

## Quick Setup

### 1. Create an Application on Your Mastodon Instance

1. Log into your Mastodon instance (e.g., mastodon.social, fosstodon.org, etc.)

2. Go to: **Preferences** → **Development** → **Your applications**

3. Click **"New application"**

4. Fill in the details:
   - **Application name**: Doh! (or whatever you prefer)
   - **Scopes**: At minimum, check:
     - ✓ `write:statuses` (required to post)
     - ✓ `write:media` (required to upload images)
   - **Redirect URI**: Can leave default or use `urn:ietf:wg:oauth:2.0:oob`

5. Click **"Submit"**

### 2. Get Your Access Token

After creating the application:

1. Click on your newly created app in the list

2. You'll see your credentials:
   - **Client key** (Client ID)
   - **Client secret**
   - **Your access token** ← This is what you need!

3. Copy the **Access Token** - it's a long string like:
   ```
   abcd1234efgh5678ijkl9012mnop3456qrst7890uvwx1234yz
   ```

### 3. Configure in Doh!

1. Open Doh! (run `~/bin/doh`)

2. Click **Settings** (gear icon)

3. Click **▶ Mastodon** to expand the section

4. Enter:
   - **Instance URL**: Your instance URL with https://
     - Examples:
       - `https://mastodon.social`
       - `https://fosstodon.org`
       - `https://mas.to`
     - ⚠️  Include `https://` and no trailing slash
   
   - **Access Token**: Paste the token you copied

5. Click **"Save Credentials"**

6. Return to main view

7. The **Mastodon ✓** toggle should now appear!

## Testing

1. Type a test message
2. Enable **Mastodon** toggle
3. Click **"Post"**
4. Check your Mastodon profile - the post should appear!

## Popular Mastodon Instances

- **mastodon.social** - General purpose, largest instance
- **fosstodon.org** - FOSS and tech focused
- **mas.to** - General purpose, well-moderated
- **techhub.social** - Tech enthusiasts
- **mstdn.social** - Another general instance

## Token Security

- Your access token is stored securely in your system keyring
- Never share your access token with anyone
- If compromised, regenerate it from Preferences → Development
- Each app gets its own token

## Troubleshooting

**"Failed to post status: 401"**
- Invalid or expired access token
- Regenerate token from your Mastodon app settings

**"Failed to post status: 422"**
- Your post might be too long (Mastodon has character limits)
- Or missing required scopes - ensure `write:statuses` is enabled

**"Not configured" in Doh!**
- Check that you entered the Instance URL correctly
- Ensure access token is pasted without extra spaces
- Click "Save Credentials" after entering

**"Connection failed"**
- Check your instance URL format: `https://instance.domain`
- No trailing slash
- Must start with `https://`

## Features

✅ Post text status updates
✅ Upload images with posts
✅ Works with ANY Mastodon-compatible instance
✅ Secure token storage
✅ Multi-platform posting (combine with X, BlueSky, Nostr, Threads!)

## Notes

- Character limits vary by instance (typically 500 chars)
- Image limits vary by instance (typically 4 images)
- Access tokens don't expire unless you regenerate them
- You can create multiple apps for different purposes

## Example Workflow

1. Write your post in Doh!
2. Select multiple platforms (e.g., Mastodon + BlueSky + Nostr)
3. Click Post
4. Your content appears on all selected platforms!

---

**Enjoy federated social media posting with Doh!** 🐘
