// SPDX-License-Identifier: MIT

use anyhow::{anyhow, Result};
use reqwest::multipart;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct MastodonPlatform {
    instance_url: String,
    access_token: String,
}

#[derive(Debug, Serialize)]
struct StatusPayload {
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    media_ids: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct MediaUploadResponse {
    id: String,
}

#[derive(Debug, Deserialize)]
struct StatusResponse {
    id: String,
    url: Option<String>,
}

impl MastodonPlatform {
    pub fn new(instance_url: String, access_token: String) -> Self {
        eprintln!("[Mastodon] Creating platform with instance: {}", instance_url);
        Self {
            instance_url: instance_url.trim_end_matches('/').to_string(),
            access_token,
        }
    }

    pub async fn post(&self, text: String, images: &[Vec<u8>]) -> Result<String> {
        eprintln!("[Mastodon] Starting post");
        eprintln!("[Mastodon] Text length: {}", text.len());
        eprintln!("[Mastodon] Image count: {}", images.len());

        // Upload images if any
        let mut media_ids = Vec::new();
        for (i, image_bytes) in images.iter().enumerate() {
            eprintln!("[Mastodon] Uploading image {} ({} bytes)", i + 1, image_bytes.len());
            match self.upload_media(image_bytes).await {
                Ok(media_id) => {
                    eprintln!("[Mastodon] Image {} uploaded successfully: {}", i + 1, media_id);
                    media_ids.push(media_id);
                }
                Err(e) => {
                    eprintln!("[Mastodon] Failed to upload image {}: {}", i + 1, e);
                    return Err(anyhow!("Failed to upload image: {}", e));
                }
            }
        }

        // Post status
        let client = reqwest::Client::new();
        let url = format!("{}/api/v1/statuses", self.instance_url);

        eprintln!("[Mastodon] Posting to: {}", url);

        let payload = StatusPayload {
            status: text,
            media_ids: if media_ids.is_empty() {
                None
            } else {
                Some(media_ids)
            },
        };

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .json(&payload)
            .send()
            .await?;

        let status_code = response.status();
        eprintln!("[Mastodon] Response status: {}", status_code);

        if !status_code.is_success() {
            let error_text = response.text().await?;
            eprintln!("[Mastodon] Error response: {}", error_text);
            return Err(anyhow!("Failed to post status: {} - {}", status_code, error_text));
        }

        let status_response: StatusResponse = response.json().await?;
        let post_url = status_response.url.unwrap_or_else(|| {
            format!("{}/web/statuses/{}", self.instance_url, status_response.id)
        });

        eprintln!("[Mastodon] Post successful: {}", post_url);
        Ok(post_url)
    }

    async fn upload_media(&self, image_bytes: &[u8]) -> Result<String> {
        use crate::image_utils;
        
        eprintln!("[Mastodon] Processing image ({} bytes)", image_bytes.len());
        
        let mime_type = image_utils::get_mime_type(image_bytes)?;
        let file_name = format!("image.{}", 
            if mime_type.contains("png") { "png" } 
            else if mime_type.contains("gif") { "gif" }
            else if mime_type.contains("webp") { "webp" }
            else { "jpg" }
        );

        eprintln!("[Mastodon] MIME type: {}, filename: {}", mime_type, file_name);

        let part = multipart::Part::bytes(image_bytes.to_vec())
            .file_name(file_name)
            .mime_str(&mime_type)?;

        let form = multipart::Form::new().part("file", part);

        let client = reqwest::Client::new();
        let url = format!("{}/api/v2/media", self.instance_url);

        eprintln!("[Mastodon] Uploading to: {}", url);

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .multipart(form)
            .send()
            .await?;

        let status_code = response.status();
        eprintln!("[Mastodon] Upload response status: {}", status_code);

        if !status_code.is_success() {
            let error_text = response.text().await?;
            eprintln!("[Mastodon] Upload error: {}", error_text);
            return Err(anyhow!("Failed to upload media: {} - {}", status_code, error_text));
        }

        let media_response: MediaUploadResponse = response.json().await?;
        eprintln!("[Mastodon] Media ID: {}", media_response.id);
        
        Ok(media_response.id)
    }
}
