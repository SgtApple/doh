# Doh! 💬

A beautiful multi-platform social media posting applet for the COSMIC desktop environment.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Platform](https://img.shields.io/badge/platform-Linux-lightgrey.svg)
![Desktop](https://img.shields.io/badge/desktop-COSMIC-orange.svg)

## ✨ Features

- **Multi-Platform Posting**: Post to multiple social networks simultaneously
- **COSMIC Integration**: Native panel applet with elegant UI
- **Secure Storage**: Credentials stored safely in system keyring
- **Platform Support**:
  - 🐦 **X (Twitter)** - Full OAuth 1.0a support
  - 🦋 **BlueSky** - Simple handle + password authentication
  - ⚡ **Nostr** - Direct nsec or Pleb_Signer integration
  - 🐘 **Mastodon** - Works with any instance
- **Easy Configuration**: Collapsible settings sections for each platform
- **Image Support**: Attach images to your posts
- **Character Counter**: Real-time feedback on post length
- **Status Feedback**: See success/failure for each platform

## 🚀 Quick Start

### Installation from .deb

Download the latest `.deb` package from [Releases](https://github.com/sgtapple/doh/releases) and install:

```bash
sudo dpkg -i doh_0.1.0_amd64.deb
```

### Building from Source

**Prerequisites:**
- Rust toolchain (1.70+)
- COSMIC desktop environment
- System dependencies:
  ```bash
  sudo apt install libdbus-1-dev pkg-config libssl-dev
  ```

**Build:**
```bash
git clone https://github.com/sgtapple/doh.git
cd doh
cargo build --release
sudo cp target/release/doh /usr/bin/doh
sudo cp com.sgtapple.doh.desktop /usr/share/applications/
sudo cp resources/icon.svg /usr/share/icons/hicolor/scalable/apps/com.sgtapple.doh.svg
```

## ⚙️ Configuration

Launch Doh from the COSMIC panel and click the settings icon.

### X (Twitter) 🐦

1. Get API credentials from [developer.twitter.com](https://developer.twitter.com/)
2. Enter Consumer Key, Consumer Secret, Access Token, and Access Token Secret

### BlueSky 🦋

1. Create app password at [bsky.app/settings/app-passwords](https://bsky.app/settings/app-passwords)
2. Enter your handle and app password

### Nostr ⚡

**Option 1:** Direct nsec key  
**Option 2:** [Pleb_Signer](https://github.com/PlebOne/Pleb_Signer) via D-Bus

### Mastodon 🐘

1. Go to your instance → Preferences → Development → New Application
2. Grant `write:statuses` and `write:media` scopes
3. Copy Access Token
4. Enter Instance URL and Access Token

## 🎯 Usage

1. **Launch** Doh from the COSMIC panel
2. **Configure** platforms in Settings
3. **Write** your post
4. **Select** which platforms to post to
5. **Click** "Post" - your message goes everywhere!

## 🛠️ Development

### Technologies

- **UI**: [libcosmic](https://github.com/pop-os/libcosmic)
- **Language**: Rust 2024
- **Storage**: System keyring
- **Nostr**: [nostr-sdk](https://github.com/rust-nostr/nostr)
- **D-Bus**: [zbus](https://github.com/dbus2/zbus)

### Building

```bash
cargo build --release
```

## 📜 License

MIT License

## 🙏 Acknowledgments

- [COSMIC Desktop](https://github.com/pop-os/cosmic-epoch) by System76
- [Pleb_Signer](https://github.com/PlebOne/Pleb_Signer)
- All open source libraries used

---

**Post once, share everywhere! 🚀**
