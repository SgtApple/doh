// SPDX-License-Identifier: MIT

//! Secure credential storage using system keyring

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

const SERVICE_NAME: &str = "com.sgtapple.doh";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Credentials {
    // X/Twitter OAuth 1.0a
    pub twitter_consumer_key: Option<String>,
    pub twitter_consumer_secret: Option<String>,
    pub twitter_access_token: Option<String>,
    pub twitter_access_secret: Option<String>,
    
    // BlueSky
    pub bluesky_handle: Option<String>,
    pub bluesky_app_password: Option<String>,
    
    // Nostr
    pub nostr_nsec: Option<String>,
    pub nostr_use_pleb_signer: bool,
    pub nostr_image_host_url: Option<String>,
    pub nostr_relays: Vec<String>,
    
    // Mastodon
    pub mastodon_instance_url: Option<String>,
    pub mastodon_access_token: Option<String>,
}

impl Credentials {
    /// Load credentials from system keyring
    pub fn load() -> Result<Self> {
        let entry = keyring::Entry::new(SERVICE_NAME, "credentials")?;
        match entry.get_password() {
            Ok(json) => {
                eprintln!("[Credentials] Loaded from keyring: {}", json);
                serde_json::from_str(&json).map_err(|e| anyhow!("Failed to parse credentials: {}", e))
            }
            Err(keyring::Error::NoEntry) => {
                eprintln!("[Credentials] No entry found in keyring, using defaults");
                Ok(Self::default())
            }
            Err(e) => {
                eprintln!("[Credentials] Error loading: {}", e);
                Err(anyhow!("Failed to load credentials: {}", e))
            }
        }
    }
    
    /// Save credentials to system keyring
    pub fn save(&self) -> Result<()> {
        let json = serde_json::to_string(self)?;
        eprintln!("[Credentials] Saving to keyring: {}", json);
        let entry = keyring::Entry::new(SERVICE_NAME, "credentials")?;
        entry.set_password(&json)
            .map_err(|e| {
                eprintln!("[Credentials] Failed to save: {}", e);
                anyhow!("Failed to save credentials: {}", e)
            })?;
        eprintln!("[Credentials] Successfully saved to keyring");
        Ok(())
    }
    
    /// Check if X/Twitter is configured
    pub fn has_twitter(&self) -> bool {
        self.twitter_consumer_key.is_some()
            && self.twitter_consumer_secret.is_some()
            && self.twitter_access_token.is_some()
            && self.twitter_access_secret.is_some()
    }
    
    /// Check if BlueSky is configured
    pub fn has_bluesky(&self) -> bool {
        self.bluesky_handle.is_some() && self.bluesky_app_password.is_some()
    }
    
    /// Check if Nostr is configured
    pub fn has_nostr(&self) -> bool {
        self.nostr_use_pleb_signer || self.nostr_nsec.is_some()
    }
    
    /// Check if Mastodon is configured
    pub fn has_mastodon(&self) -> bool {
        self.mastodon_instance_url.is_some() && self.mastodon_access_token.is_some()
    }
}
