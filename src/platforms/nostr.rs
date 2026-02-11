// SPDX-License-Identifier: MIT

//! Nostr platform adapter with dual authentication support

use super::{Platform, Post, PostResult};
use anyhow::{Result, anyhow};
use nostr_sdk::prelude::*;
use serde::Deserialize;

pub enum NostrAuth {
    /// Direct private key
    Nsec(String),
    /// Use Pleb_Signer via D-Bus
    PlebSigner,
}

pub struct NostrPlatform {
    auth: NostrAuth,
    relays: Vec<String>,
    image_host_url: Option<String>,
}

impl NostrPlatform {
    pub fn new(auth: NostrAuth, relays: Vec<String>, image_host_url: Option<String>) -> Self {
        Self {
            auth,
            relays,
            image_host_url,
        }
    }
    
    async fn get_keys(&self) -> Result<Keys> {
        match &self.auth {
            NostrAuth::Nsec(nsec_str) => {
                eprintln!("[Nostr] Parsing nsec key (length: {})", nsec_str.len());
                if nsec_str.is_empty() {
                    eprintln!("[Nostr] Error: nsec key is empty!");
                    return Err(anyhow!("nsec key is empty"));
                }
                
                // Try to parse as nsec
                match Keys::parse(nsec_str) {
                    Ok(keys) => {
                        eprintln!("[Nostr] Successfully parsed nsec key");
                        Ok(keys)
                    }
                    Err(e) => {
                        eprintln!("[Nostr] Failed to parse nsec key: {}", e);
                        eprintln!("[Nostr] Hint: nsec should start with 'nsec1'");
                        Err(anyhow!("Invalid nsec key: {}. Must start with 'nsec1'", e))
                    }
                }
            }
            NostrAuth::PlebSigner => {
                eprintln!("[Nostr] Using Pleb_Signer for authentication");
                
                // For Pleb_Signer, we can't get Keys directly
                // We'll need to create a dummy Keys object for the client
                // The actual signing will be done by Pleb_Signer
                
                // Just generate a temporary key for the client
                // (we won't use it for signing)
                Ok(Keys::generate())
            }
        }
    }
    
    async fn post_with_nsec(&self, post: &Post) -> Result<PostResult> {
        use crate::image_utils;
        
        // Get keys
        let keys = match self.get_keys().await {
            Ok(k) => {
                eprintln!("[Nostr] Keys loaded successfully");
                k
            }
            Err(e) => {
                eprintln!("[Nostr] Failed to load keys: {}", e);
                return Ok(PostResult::Error { 
                    message: format!("Failed to load keys: {}", e)
                });
            }
        };
        
        // Create client
        eprintln!("[Nostr] Creating client...");
        let client = Client::new(keys);
        
        // Add relays
        eprintln!("[Nostr] Adding {} relays...", self.relays.len());
        for relay_url in &self.relays {
            eprintln!("[Nostr] Adding relay: {}", relay_url);
            match client.add_relay(relay_url).await {
                Ok(_) => eprintln!("[Nostr] Relay added: {}", relay_url),
                Err(e) => eprintln!("[Nostr] Failed to add relay {}: {}", relay_url, e),
            }
        }
        
        eprintln!("[Nostr] Connecting to relays...");
        client.connect().await;
        
        // Give relays a moment to connect
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        
        // Upload images if any
        let mut image_urls = Vec::new();
        if !post.images.is_empty() {
            if let Some(blossom_url) = &self.image_host_url {
                eprintln!("[Nostr] Uploading {} images to Blossom", post.images.len());
                for (i, image_bytes) in post.images.iter().enumerate() {
                    // Strip EXIF data for privacy (Blossom requirement)
                    let processor = image_utils::ImageProcessor::new()
                        .with_exif_stripping();
                    
                    let processed_bytes = match processor.process(image_bytes) {
                        Ok(bytes) => {
                            eprintln!("[Nostr] Image {} processed ({} -> {} bytes)", 
                                i + 1, image_bytes.len(), bytes.len());
                            bytes
                        }
                        Err(e) => {
                            eprintln!("[Nostr] Failed to process image {}: {}", i + 1, e);
                            continue;
                        }
                    };
                    
                    match self.upload_to_blossom(&processed_bytes, blossom_url, &client).await {
                        Ok(url) => {
                            eprintln!("[Nostr] Image {} uploaded: {}", i + 1, url);
                            image_urls.push(url);
                        }
                        Err(e) => {
                            eprintln!("[Nostr] Failed to upload image {}: {}", i + 1, e);
                        }
                    }
                }
            } else {
                eprintln!("[Nostr] No Blossom server configured, skipping image upload");
            }
        }
        
        // Create text note with image URLs
        let mut content = post.text.clone();
        for url in &image_urls {
            content.push_str(&format!("\n{}", url));
        }
        
        eprintln!("[Nostr] Creating event...");
        let builder = EventBuilder::text_note(&content, vec![]);
        
        eprintln!("[Nostr] Sending event to relays...");
        match client.send_event_builder(builder).await {
            Ok(event_id) => {
                eprintln!("[Nostr] Event sent successfully: {:?}", event_id);
                Ok(PostResult::Success { url: None })
            }
            Err(e) => {
                eprintln!("[Nostr] Failed to send event: {}", e);
                Ok(PostResult::Error {
                    message: format!("Failed to send event: {}", e)
                })
            }
        }
    }
    
    async fn upload_to_blossom(&self, image_bytes: &[u8], blossom_url: &str, _client: &Client) -> Result<String> {
        use crate::image_utils;
        use sha2::{Sha256, Digest};
        use base64::Engine;
        
        // Calculate SHA-256 hash
        let mut hasher = Sha256::new();
        hasher.update(image_bytes);
        let hash = format!("{:x}", hasher.finalize());
        
        let mime_type = image_utils::get_mime_type(image_bytes)?;
        
        // Create auth event for upload
        let upload_url = format!("{}/upload", blossom_url.trim_end_matches('/'));
        let timestamp = Timestamp::now();
        
        let auth_tags = vec![
            Tag::custom(TagKind::Custom(std::borrow::Cow::Borrowed("t")), vec!["upload"]),
            Tag::custom(TagKind::Custom(std::borrow::Cow::Borrowed("x")), vec![&hash]),
            Tag::custom(TagKind::Custom(std::borrow::Cow::Borrowed("expiration")), 
                vec![&(timestamp.as_u64() + 600).to_string()]), // 10 min expiration
        ];
        
        // Create and sign auth event
        let keys = self.get_keys().await?;
        let auth_event = EventBuilder::new(Kind::Custom(24242), "", auth_tags)
            .sign_with_keys(&keys)?;
        
        let engine = base64::engine::general_purpose::STANDARD;
        let auth_header = engine.encode(auth_event.as_json());
        
        // Upload to Blossom
        let http_client = reqwest::Client::new();
        let response = http_client
            .put(&upload_url)
            .header("Authorization", format!("Nostr {}", auth_header))
            .header("Content-Type", &mime_type)
            .body(image_bytes.to_vec())
            .send()
            .await?;
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("Blossom upload failed: {}", error_text));
        }
        
        #[derive(Deserialize)]
        struct BlossomResponse {
            url: String,
        }
        
        let blossom_response: BlossomResponse = response.json().await?;
        Ok(blossom_response.url)
    }
    
    async fn post_with_pleb_signer(&self, post: &Post) -> Result<PostResult> {
        use crate::image_utils;
        
        eprintln!("[Nostr] Posting via Pleb_Signer...");
        
        // Get pubkey from Pleb_Signer
        let _pubkey_hex = match get_pleb_signer_pubkey().await {
            Ok(pk) => pk,
            Err(e) => {
                eprintln!("[Nostr] Failed to get pubkey from Pleb_Signer: {}", e);
                return Ok(PostResult::Error {
                    message: format!("Failed to get pubkey: {}", e)
                });
            }
        };
        
        // Upload images if any
        let mut image_urls = Vec::new();
        if !post.images.is_empty() {
            if let Some(blossom_url) = &self.image_host_url {
                eprintln!("[Nostr] Uploading {} images to Blossom", post.images.len());
                
                // For PlebSigner, we need a temporary client for Blossom uploads
                let temp_keys = Keys::generate();
                let temp_client = Client::new(temp_keys);
                
                for (i, image_bytes) in post.images.iter().enumerate() {
                    // Strip EXIF data for privacy (Blossom requirement)
                    let processor = image_utils::ImageProcessor::new()
                        .with_exif_stripping();
                    
                    let processed_bytes = match processor.process(image_bytes) {
                        Ok(bytes) => {
                            eprintln!("[Nostr] Image {} processed ({} -> {} bytes)", 
                                i + 1, image_bytes.len(), bytes.len());
                            bytes
                        }
                        Err(e) => {
                            eprintln!("[Nostr] Failed to process image {}: {}", i + 1, e);
                            continue;
                        }
                    };
                    
                    match self.upload_to_blossom(&processed_bytes, blossom_url, &temp_client).await {
                        Ok(url) => {
                            eprintln!("[Nostr] Image {} uploaded: {}", i + 1, url);
                            image_urls.push(url);
                        }
                        Err(e) => {
                            eprintln!("[Nostr] Failed to upload image {}: {}", i + 1, e);
                        }
                    }
                }
            } else {
                eprintln!("[Nostr] No Blossom server configured, skipping image upload");
            }
        }
        
        // Create content with image URLs
        let mut content = post.text.clone();
        for url in &image_urls {
            content.push_str(&format!("\n{}", url));
        }
        
        // Create unsigned event
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let unsigned_event = serde_json::json!({
            "kind": 1,
            "content": content,
            "tags": [],
            "created_at": timestamp
        });
        
        eprintln!("[Nostr] Requesting signature from Pleb_Signer...");
        let signed_event_json = match sign_event_with_pleb_signer(&unsigned_event.to_string()).await {
            Ok(se) => se,
            Err(e) => {
                eprintln!("[Nostr] Failed to sign event with Pleb_Signer: {}", e);
                return Ok(PostResult::Error {
                    message: format!("Failed to sign event: {}", e)
                });
            }
        };
        
        eprintln!("[Nostr] Event signed successfully");
        
        // Parse signed event
        let signed_event: nostr_sdk::Event = match serde_json::from_str(&signed_event_json) {
            Ok(e) => e,
            Err(e) => {
                eprintln!("[Nostr] Failed to parse signed event: {}", e);
                return Ok(PostResult::Error {
                    message: format!("Failed to parse signed event: {}", e)
                });
            }
        };
        
        // Create client without keys (we'll send pre-signed event)
        let keys = match self.get_keys().await {
            Ok(k) => k,
            Err(e) => {
                return Ok(PostResult::Error {
                    message: format!("Failed to create client: {}", e)
                });
            }
        };
        let client = Client::new(keys);
        
        // Add relays
        eprintln!("[Nostr] Adding {} relays...", self.relays.len());
        for relay_url in &self.relays {
            eprintln!("[Nostr] Adding relay: {}", relay_url);
            match client.add_relay(relay_url).await {
                Ok(_) => eprintln!("[Nostr] Relay added: {}", relay_url),
                Err(e) => eprintln!("[Nostr] Failed to add relay {}: {}", relay_url, e),
            }
        }
        
        eprintln!("[Nostr] Connecting to relays...");
        client.connect().await;
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        
        // Send pre-signed event
        eprintln!("[Nostr] Sending signed event to relays...");
        match client.send_event(signed_event).await {
            Ok(event_id) => {
                eprintln!("[Nostr] Event sent successfully: {:?}", event_id);
                Ok(PostResult::Success { url: None })
            }
            Err(e) => {
                eprintln!("[Nostr] Failed to send event: {}", e);
                Ok(PostResult::Error {
                    message: format!("Failed to send event: {}", e)
                })
            }
        }
    }
}

impl Platform for NostrPlatform {
    fn name(&self) -> &'static str {
        "Nostr"
    }
    
    async fn is_authenticated(&self) -> bool {
        match &self.auth {
            NostrAuth::Nsec(key) => !key.is_empty() && Keys::parse(key).is_ok(),
            NostrAuth::PlebSigner => {
                check_pleb_signer_available().await
            }
        }
    }
    
    async fn post(&self, post: &Post) -> Result<PostResult> {
        eprintln!("[Nostr] Starting post attempt...");
        
        match &self.auth {
            NostrAuth::Nsec(_) => {
                // Direct posting with nsec key
                self.post_with_nsec(post).await
            }
            NostrAuth::PlebSigner => {
                // Posting via Pleb_Signer
                self.post_with_pleb_signer(post).await
            }
        }
    }
}

#[derive(Deserialize)]
struct PlebSignerResponse {
    success: bool,
    result: Option<String>,
    error: Option<String>,
}

async fn check_pleb_signer_available() -> bool {
    // Check if Pleb_Signer is running via D-Bus
    match zbus::Connection::session().await {
        Ok(conn) => {
            let proxy = zbus::fdo::DBusProxy::new(&conn).await;
            if let Ok(proxy) = proxy {
                let name = zbus::names::BusName::from_static_str("com.plebsigner.Signer").unwrap();
                proxy.name_has_owner(name).await.unwrap_or(false)
            } else {
                false
            }
        }
        Err(_) => false,
    }
}

async fn get_pleb_signer_pubkey() -> Result<String> {
    let conn = zbus::Connection::session().await?;
    
    let msg = conn.call_method(
        Some("com.plebsigner.Signer"),
        "/com/plebsigner/Signer",
        Some("com.plebsigner.Signer1"),
        "GetPublicKey",
        &(),
    ).await.map_err(|e| anyhow!("D-Bus call failed: {}", e))?;
    
    let response: String = msg.body().deserialize()?;
    
    eprintln!("[Nostr/PlebSigner] GetPublicKey response: {}", response);
    
    let parsed: PlebSignerResponse = serde_json::from_str(&response)?;
    
    if !parsed.success {
        return Err(anyhow!("Pleb_Signer error: {}", parsed.error.unwrap_or_else(|| "Unknown error".to_string())));
    }
    
    let result_json = parsed.result.ok_or_else(|| anyhow!("No result in response"))?;
    
    // Result is double-encoded JSON string
    let pubkey: String = serde_json::from_str(&result_json)?;
    
    Ok(pubkey)
}

async fn sign_event_with_pleb_signer(event_json: &str) -> Result<String> {
    let conn = zbus::Connection::session().await?;
    
    let app_id = "com.sgtapple.doh";
    let msg = conn.call_method(
        Some("com.plebsigner.Signer"),
        "/com/plebsigner/Signer",
        Some("com.plebsigner.Signer1"),
        "SignEvent",
        &(event_json, app_id),
    ).await.map_err(|e| anyhow!("D-Bus call failed: {}", e))?;
    
    let response: String = msg.body().deserialize()?;
    
    eprintln!("[Nostr/PlebSigner] SignEvent response: {}", response);
    
    let parsed: PlebSignerResponse = serde_json::from_str(&response)?;
    
    if !parsed.success {
        return Err(anyhow!("Pleb_Signer error: {}", parsed.error.unwrap_or_else(|| "Unknown error".to_string())));
    }
    
    let result_json = parsed.result.ok_or_else(|| anyhow!("No result in response"))?;
    
    // Result is double-encoded JSON string containing the signed event
    let signed_event_json: String = serde_json::from_str(&result_json)?;
    
    Ok(signed_event_json)
}
