# Installing Doh! Applet

## Prerequisites

- COSMIC Desktop Environment (Alpha 4 or later)
- Rust toolchain
- System keyring (usually included with COSMIC)

## Build

```bash
cd /home/sgtapple/Projects/doh/doh
cargo build --release
```

## Install

### 1. Install the binary

```bash
sudo install -Dm755 target/release/doh /usr/bin/doh
```

### 2. Install the icon

```bash
sudo install -Dm644 resources/icon.svg /usr/share/icons/hicolor/scalable/apps/com.sgtapple.doh.svg
```

### 3. Install the desktop file

```bash
sudo install -Dm644 resources/app.desktop /usr/share/applications/com.sgtapple.doh.desktop
```

### 4. Update icon cache

```bash
sudo gtk-update-icon-cache -f -t /usr/share/icons/hicolor
```

## Add to Panel

1. **Open COSMIC Settings**
2. **Go to Desktop → Panel**
3. **Click "Configure Applets"**
4. **Find "Doh" in the list**
5. **Click the "+" button to add it**

The applet will appear in your panel with a speech bubble icon!

## Usage

1. **Click the Doh icon** in the panel to open the popup
2. **First time**: Click the settings gear icon to configure accounts
3. **Enter credentials** for each platform you want to use:
   - **X/Twitter**: Get API keys from developer.twitter.com
   - **BlueSky**: Create app password at bsky.app/settings/app-passwords
   - **Nostr**: Enter your nsec key or enable Pleb Signer
   - **Threads**: Get access token from Meta developers portal
4. **Click "Save Credentials"**
5. **Go back** to main view
6. **Type your post**, select platforms, and click "Post"!

## Uninstall

```bash
sudo rm /usr/bin/doh
sudo rm /usr/share/icons/hicolor/scalable/apps/com.sgtapple.doh.svg
sudo rm /usr/share/applications/com.sgtapple.doh.desktop
sudo gtk-update-icon-cache -f -t /usr/share/icons/hicolor
```

## Troubleshooting

### Applet doesn't appear in panel

- Make sure you installed all files correctly
- Update icon cache: `sudo gtk-update-icon-cache -f -t /usr/share/icons/hicolor`
- Restart COSMIC: Log out and log back in

### "Failed to save credentials" error

- Make sure your system keyring is unlocked
- COSMIC typically uses `secret-service` backend
- Test keyring: `secret-tool store --label='test' service test username test`

### Icon doesn't show

- Verify icon is installed: `ls /usr/share/icons/hicolor/scalable/apps/com.sgtapple.doh.svg`
- Update icon cache again
- Try refreshing COSMIC panel settings

### Posting fails

- Check credentials are correct
- View logs: `journalctl --user -f | grep doh`
- Test with one platform first (BlueSky is easiest)

## Development Mode

For testing without installing:

```bash
# Run directly
cargo run --release

# Or with logging
RUST_LOG=debug cargo run --release
```

Note: When running directly (not as applet), it won't integrate with the panel.
