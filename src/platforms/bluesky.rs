// SPDX-License-Identifier: MIT

//! BlueSky platform adapter using AT Protocol

use super::{Platform, Post, PostResult};
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

pub struct BlueSkyPlatform {
    handle: String,
    app_password: String,
    access_token: Option<String>,
}

#[derive(Serialize)]
struct LoginRequest {
    identifier: String,
    password: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct LoginResponse {
    #[serde(rename = "accessJwt")]
    access_jwt: String,
    #[serde(rename = "refreshJwt")]
    refresh_jwt: String,
    handle: String,
    did: String,
}

#[derive(Serialize)]
struct CreatePostRequest {
    repo: String,
    collection: String,
    record: PostRecord,
}

#[derive(Serialize)]
struct PostRecord {
    text: String,
    #[serde(rename = "createdAt")]
    created_at: String,
    #[serde(rename = "$type")]
    record_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    embed: Option<ImagesEmbed>,
}

#[derive(Serialize)]
struct ImagesEmbed {
    #[serde(rename = "$type")]
    embed_type: String,
    images: Vec<ImageRef>,
}

#[derive(Serialize)]
struct ImageRef {
    alt: String,
    image: BlobRef,
}

#[derive(Serialize, Deserialize)]
struct BlobRef {
    #[serde(rename = "$type")]
    blob_type: String,
    #[serde(rename = "ref")]
    reference: BlobReference,
    #[serde(rename = "mimeType")]
    mime_type: String,
    size: usize,
}

#[derive(Serialize, Deserialize)]
struct BlobReference {
    #[serde(rename = "$link")]
    link: String,
}

#[derive(Deserialize)]
struct UploadBlobResponse {
    blob: BlobRef,
}

impl BlueSkyPlatform {
    pub fn new(handle: String, app_password: String) -> Self {
        Self {
            handle,
            app_password,
            access_token: None,
        }
    }
    
    async fn login(&mut self) -> Result<()> {
        let client = reqwest::Client::new();
        let response = client
            .post("https://bsky.social/xrpc/com.atproto.server.createSession")
            .json(&LoginRequest {
                identifier: self.handle.clone(),
                password: self.app_password.clone(),
            })
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(anyhow!("Login failed: {}", response.status()));
        }
        
        let login_response: LoginResponse = response.json().await?;
        self.access_token = Some(login_response.access_jwt);
        Ok(())
    }
}

impl Platform for BlueSkyPlatform {
    fn name(&self) -> &'static str {
        "BlueSky"
    }
    
    async fn is_authenticated(&self) -> bool {
        !self.handle.is_empty() && !self.app_password.is_empty()
    }
    
    async fn post(&self, post: &Post) -> Result<PostResult> {
        use crate::image_utils;
        
        let mut platform = self.clone();
        
        // Login if not already authenticated
        if platform.access_token.is_none() {
            platform.login().await?;
        }
        
        let token = platform.access_token.as_ref()
            .ok_or_else(|| anyhow!("Not authenticated"))?;
        
        // Upload images if any (max 4 images, 1MB each)
        let mut image_refs = Vec::new();
        for (i, image_bytes) in post.images.iter().enumerate().take(4) {
            eprintln!("[BlueSky] Processing image {} ({} bytes)", i + 1, image_bytes.len());
            
            // Compress to 1MB max
            let processor = image_utils::ImageProcessor::new()
                .with_max_size(1_000_000) // 1MB
                .with_max_dimension(2000); // Max resolution
            
            let processed_bytes = match processor.process(image_bytes) {
                Ok(bytes) => {
                    eprintln!("[BlueSky] Image {} processed to {} bytes", i + 1, bytes.len());
                    bytes
                }
                Err(e) => {
                    eprintln!("[BlueSky] Failed to process image {}: {}", i + 1, e);
                    return Ok(PostResult::Error {
                        message: format!("Failed to process image: {}", e),
                    });
                }
            };
            
            match platform.upload_blob(&processed_bytes, token).await {
                Ok(blob_ref) => {
                    eprintln!("[BlueSky] Image {} uploaded successfully", i + 1);
                    image_refs.push(ImageRef {
                        alt: String::new(),
                        image: blob_ref,
                    });
                }
                Err(e) => {
                    eprintln!("[BlueSky] Failed to upload image {}: {}", i + 1, e);
                    return Ok(PostResult::Error {
                        message: format!("Failed to upload image: {}", e),
                    });
                }
            }
        }
        
        // Create post record
        let now = chrono::Utc::now().to_rfc3339();
        let record = PostRecord {
            text: post.text.clone(),
            created_at: now,
            record_type: "app.bsky.feed.post".to_string(),
            embed: if image_refs.is_empty() {
                None
            } else {
                Some(ImagesEmbed {
                    embed_type: "app.bsky.embed.images".to_string(),
                    images: image_refs,
                })
            },
        };
        
        let request = CreatePostRequest {
            repo: platform.handle.clone(),
            collection: "app.bsky.feed.post".to_string(),
            record,
        };
        
        let client = reqwest::Client::new();
        let response = client
            .post("https://bsky.social/xrpc/com.atproto.repo.createRecord")
            .header("Authorization", format!("Bearer {}", token))
            .json(&request)
            .send()
            .await?;
            
        if response.status().is_success() {
            Ok(PostResult::Success { url: None })
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Ok(PostResult::Error { 
                message: format!("Failed to post: {}", error_text) 
            })
        }
    }
}

impl BlueSkyPlatform {
    async fn upload_blob(&self, image_bytes: &[u8], token: &str) -> Result<BlobRef> {
        use crate::image_utils;
        
        let mime_type = image_utils::get_mime_type(image_bytes)?;
        
        let client = reqwest::Client::new();
        let response = client
            .post("https://bsky.social/xrpc/com.atproto.repo.uploadBlob")
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", &mime_type)
            .body(image_bytes.to_vec())
            .send()
            .await?;
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("Failed to upload blob: {}", error_text));
        }
        
        let upload_response: UploadBlobResponse = response.json().await?;
        Ok(upload_response.blob)
    }
}

// Make BlueSkyPlatform cloneable for the mut self workaround
impl Clone for BlueSkyPlatform {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
            app_password: self.app_password.clone(),
            access_token: self.access_token.clone(),
        }
    }
}
