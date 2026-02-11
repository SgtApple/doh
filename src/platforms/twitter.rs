// SPDX-License-Identifier: MIT

//! X/Twitter platform adapter using OAuth 1.0a

use super::{Platform, Post, PostResult};
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use base64::Engine;

pub struct TwitterPlatform {
    consumer_key: String,
    consumer_secret: String,
    access_token: String,
    access_secret: String,
}

#[derive(Deserialize)]
struct TwitterMediaResponse {
    media_id_string: String,
}

#[derive(Serialize)]
struct TweetRequest {
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    media: Option<MediaAttachment>,
}

#[derive(Serialize)]
struct MediaAttachment {
    media_ids: Vec<String>,
}

impl TwitterPlatform {
    pub fn new(
        consumer_key: String,
        consumer_secret: String,
        access_token: String,
        access_secret: String,
    ) -> Self {
        Self {
            consumer_key,
            consumer_secret,
            access_token,
            access_secret,
        }
    }
    
    fn generate_oauth_header(
        &self,
        method: &str,
        url: &str,
        params: Option<&HashMap<String, String>>,
    ) -> String {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let nonce: String = format!("{:x}", timestamp);
        
        let mut oauth_params = HashMap::new();
        oauth_params.insert("oauth_consumer_key".to_string(), self.consumer_key.clone());
        oauth_params.insert("oauth_token".to_string(), self.access_token.clone());
        oauth_params.insert("oauth_signature_method".to_string(), "HMAC-SHA1".to_string());
        oauth_params.insert("oauth_timestamp".to_string(), timestamp.to_string());
        oauth_params.insert("oauth_nonce".to_string(), nonce);
        oauth_params.insert("oauth_version".to_string(), "1.0".to_string());
        
        // Create signature
        let signature = self.generate_signature(method, url, &oauth_params, params);
        oauth_params.insert("oauth_signature".to_string(), signature);
        
        // Build OAuth header
        let mut header_parts: Vec<String> = oauth_params
            .iter()
            .map(|(k, v)| format!("{}=\"{}\"", k, Self::percent_encode(v)))
            .collect();
        header_parts.sort();
        
        format!("OAuth {}", header_parts.join(", "))
    }
    
    fn generate_signature(
        &self,
        method: &str,
        url: &str,
        oauth_params: &HashMap<String, String>,
        additional_params: Option<&HashMap<String, String>>,
    ) -> String {
        use hmac::{Hmac, Mac};
        use sha1::Sha1;
        
        // Combine all parameters
        let mut all_params = oauth_params.clone();
        if let Some(params) = additional_params {
            all_params.extend(params.clone());
        }
        
        // Sort parameters
        let mut sorted_params: Vec<_> = all_params.iter().collect();
        sorted_params.sort_by(|a, b| a.0.cmp(b.0));
        
        // Create parameter string
        let param_string = sorted_params
            .iter()
            .map(|(k, v)| format!("{}={}", Self::percent_encode(k), Self::percent_encode(v)))
            .collect::<Vec<_>>()
            .join("&");
        
        // Create signature base string
        let base_string = format!(
            "{}&{}&{}",
            method.to_uppercase(),
            Self::percent_encode(url),
            Self::percent_encode(&param_string)
        );
        
        // Create signing key
        let signing_key = format!(
            "{}&{}",
            Self::percent_encode(&self.consumer_secret),
            Self::percent_encode(&self.access_secret)
        );
        
        // Generate signature
        type HmacSha1 = Hmac<Sha1>;
        let mut mac = HmacSha1::new_from_slice(signing_key.as_bytes()).unwrap();
        mac.update(base_string.as_bytes());
        let result = mac.finalize();
        base64::engine::general_purpose::STANDARD.encode(result.into_bytes())
    }
    
    fn percent_encode(s: &str) -> String {
        s.chars()
            .map(|c| match c {
                'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '.' | '_' | '~' => c.to_string(),
                _ => format!("%{:02X}", c as u8),
            })
            .collect()
    }
}

impl Platform for TwitterPlatform {
    fn name(&self) -> &'static str {
        "X"
    }
    
    async fn is_authenticated(&self) -> bool {
        !self.consumer_key.is_empty()
            && !self.consumer_secret.is_empty()
            && !self.access_token.is_empty()
            && !self.access_secret.is_empty()
    }
    
    async fn post(&self, post: &Post) -> Result<PostResult> {
        use crate::image_utils;
        
        // Upload images first if any
        let mut media_ids = Vec::new();
        for (i, image_bytes) in post.images.iter().enumerate() {
            eprintln!("[Twitter] Uploading image {} ({} bytes)", i + 1, image_bytes.len());
            
            // Process image - Twitter supports up to 5MB
            let processor = image_utils::ImageProcessor::new()
                .with_max_size(5_000_000); // 5MB
            let processed_bytes = processor.process(image_bytes)?;
            
            match self.upload_media(&processed_bytes).await {
                Ok(media_id) => {
                    eprintln!("[Twitter] Image {} uploaded: {}", i + 1, media_id);
                    media_ids.push(media_id);
                }
                Err(e) => {
                    eprintln!("[Twitter] Failed to upload image {}: {}", i + 1, e);
                    return Ok(PostResult::Error {
                        message: format!("Failed to upload image: {}", e),
                    });
                }
            }
        }
        
        let url = "https://api.twitter.com/2/tweets";
        
        let tweet = TweetRequest {
            text: post.text.clone(),
            media: if media_ids.is_empty() {
                None
            } else {
                Some(MediaAttachment { media_ids })
            },
        };
        
        let oauth_header = self.generate_oauth_header("POST", url, None);
        
        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .header("Authorization", oauth_header)
            .header("Content-Type", "application/json")
            .json(&tweet)
            .send()
            .await?;
        
        if response.status().is_success() {
            Ok(PostResult::Success {
                url: Some("Posted to X".to_string()),
            })
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Ok(PostResult::Error {
                message: format!("Failed to post: {}", error_text),
            })
        }
    }
}

impl TwitterPlatform {
    async fn upload_media(&self, image_bytes: &[u8]) -> Result<String> {
        use crate::image_utils;
        
        let url = "https://upload.twitter.com/1.1/media/upload.json";
        let mime_type = image_utils::get_mime_type(image_bytes)?;
        
        let oauth_header = self.generate_oauth_header("POST", url, None);
        
        let part = reqwest::multipart::Part::bytes(image_bytes.to_vec())
            .mime_str(&mime_type)?;
        
        let form = reqwest::multipart::Form::new()
            .part("media", part);
        
        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .header("Authorization", oauth_header)
            .multipart(form)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("Failed to upload media: {}", error_text));
        }
        
        let media_response: TwitterMediaResponse = response.json().await?;
        Ok(media_response.media_id_string)
    }
}
