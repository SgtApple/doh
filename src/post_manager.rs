// SPDX-License-Identifier: MIT

//! Post manager for coordinating multi-platform posting

use crate::credentials::Credentials;
use crate::platforms::{Platform, Post, PostResult};
use crate::platforms::nostr::{NostrPlatform, NostrAuth};
use crate::platforms::bluesky::BlueSkyPlatform;
use crate::platforms::twitter::TwitterPlatform;
use crate::platforms::mastodon::MastodonPlatform;
use anyhow::Result;

pub struct PostManager {
    credentials: Credentials,
}

impl PostManager {
    pub fn new(credentials: Credentials) -> Self {
        Self { credentials }
    }
    
    pub async fn post(
        &self,
        text: String,
        images: Vec<Vec<u8>>,
        platforms: Vec<String>,
    ) -> Vec<(String, bool, String)> {
        let post = Post { text, images };
        let mut results = Vec::new();
        
        for platform_name in platforms {
            let result = match platform_name.as_str() {
                "Nostr" => self.post_nostr(&post).await,
                "BlueSky" => self.post_bluesky(&post).await,
                "X" => self.post_twitter(&post).await,
                "Mastodon" => self.post_mastodon(&post).await,
                _ => continue,
            };
            
            results.push(result);
        }
        
        results
    }
    
    async fn post_nostr(&self, post: &Post) -> (String, bool, String) {
        if !self.credentials.has_nostr() {
            return ("Nostr".to_string(), false, "Not configured".to_string());
        }
        
        let auth = if self.credentials.nostr_use_pleb_signer {
            NostrAuth::PlebSigner
        } else {
            NostrAuth::Nsec(self.credentials.nostr_nsec.clone().unwrap_or_default())
        };
        
        let relays = if self.credentials.nostr_relays.is_empty() {
            vec![
                "wss://relay.primal.net".to_string(),
                "wss://relay.damus.io".to_string(),
                "wss://relay.pleb.one".to_string(),
            ]
        } else {
            self.credentials.nostr_relays.clone()
        };
        
        let mut platform = NostrPlatform::new(
            auth,
            relays,
            self.credentials.nostr_image_host_url.clone(),
        );
        
        match platform.post(post).await {
            Ok(PostResult::Success { url }) => {
                eprintln!("[Nostr] Success: {}", url.as_ref().unwrap_or(&"Posted successfully".to_string()));
                ("Nostr".to_string(), true, url.unwrap_or_else(|| "Posted successfully".to_string()))
            }
            Ok(PostResult::Error { message }) => {
                eprintln!("[Nostr] Error: {}", message);
                ("Nostr".to_string(), false, message)
            }
            Err(e) => {
                eprintln!("[Nostr] Exception: {}", e);
                ("Nostr".to_string(), false, format!("Error: {}", e))
            }
        }
    }
    
    async fn post_bluesky(&self, post: &Post) -> (String, bool, String) {
        if !self.credentials.has_bluesky() {
            return ("BlueSky".to_string(), false, "Not configured".to_string());
        }
        
        let mut platform = BlueSkyPlatform::new(
            self.credentials.bluesky_handle.clone().unwrap(),
            self.credentials.bluesky_app_password.clone().unwrap(),
        );
        
        match platform.post(post).await {
            Ok(PostResult::Success { url }) => {
                eprintln!("[BlueSky] Success: {}", url.as_ref().unwrap_or(&"Posted successfully".to_string()));
                ("BlueSky".to_string(), true, url.unwrap_or_else(|| "Posted successfully".to_string()))
            }
            Ok(PostResult::Error { message }) => {
                eprintln!("[BlueSky] Error: {}", message);
                ("BlueSky".to_string(), false, message)
            }
            Err(e) => {
                eprintln!("[BlueSky] Exception: {}", e);
                ("BlueSky".to_string(), false, format!("Error: {}", e))
            }
        }
    }
    
    async fn post_twitter(&self, post: &Post) -> (String, bool, String) {
        if !self.credentials.has_twitter() {
            return ("X".to_string(), false, "Not configured".to_string());
        }
        
        let platform = TwitterPlatform::new(
            self.credentials.twitter_consumer_key.clone().unwrap(),
            self.credentials.twitter_consumer_secret.clone().unwrap(),
            self.credentials.twitter_access_token.clone().unwrap(),
            self.credentials.twitter_access_secret.clone().unwrap(),
        );
        
        match platform.post(post).await {
            Ok(PostResult::Success { url }) => {
                ("X".to_string(), true, url.unwrap_or_else(|| "Posted successfully".to_string()))
            }
            Ok(PostResult::Error { message }) => {
                ("X".to_string(), false, message)
            }
            Err(e) => {
                ("X".to_string(), false, format!("Error: {}", e))
            }
        }
    }
    
    async fn post_mastodon(&self, post: &Post) -> (String, bool, String) {
        if !self.credentials.has_mastodon() {
            return ("Mastodon".to_string(), false, "Not configured".to_string());
        }
        
        let platform = MastodonPlatform::new(
            self.credentials.mastodon_instance_url.clone().unwrap(),
            self.credentials.mastodon_access_token.clone().unwrap(),
        );
        
        // Convert image Vec<u8> to temp files for now (simplified)
        // In production, you'd want better handling
        let image_paths: Vec<String> = Vec::new(); // TODO: Handle images properly
        
        match platform.post(post.text.clone(), image_paths).await {
            Ok(url) => {
                eprintln!("[Mastodon] Success: {}", url);
                ("Mastodon".to_string(), true, url)
            }
            Err(e) => {
                eprintln!("[Mastodon] Error: {}", e);
                ("Mastodon".to_string(), false, format!("Error: {}", e))
            }
        }
    }
}
