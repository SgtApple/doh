use anyhow::{anyhow, Result};
use reqwest::multipart;
use serde::{Deserialize, Serialize};
use std::path::Path;

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

    pub async fn post(&self, text: String, image_paths: Vec<String>) -> Result<String> {
        eprintln!("[Mastodon] Starting post");
        eprintln!("[Mastodon] Text length: {}", text.len());
        eprintln!("[Mastodon] Image count: {}", image_paths.len());

        // Upload images if any
        let mut media_ids = Vec::new();
        for (i, path) in image_paths.iter().enumerate() {
            eprintln!("[Mastodon] Uploading image {}: {}", i + 1, path);
            match self.upload_media(path).await {
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

    async fn upload_media(&self, path: &str) -> Result<String> {
        eprintln!("[Mastodon] Reading file: {}", path);
        
        let file_path = Path::new(path);
        if !file_path.exists() {
            return Err(anyhow!("File does not exist: {}", path));
        }

        let file_bytes = tokio::fs::read(file_path).await?;
        let file_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("image.jpg");

        eprintln!("[Mastodon] File size: {} bytes", file_bytes.len());

        let part = multipart::Part::bytes(file_bytes)
            .file_name(file_name.to_string())
            .mime_str("image/jpeg")?;

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
