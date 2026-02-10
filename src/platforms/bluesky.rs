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
        let mut platform = self.clone();
        
        // Login if not already authenticated
        if platform.access_token.is_none() {
            platform.login().await?;
        }
        
        let token = platform.access_token.as_ref()
            .ok_or_else(|| anyhow!("Not authenticated"))?;
        
        // Create post record
        let now = chrono::Utc::now().to_rfc3339();
        let record = PostRecord {
            text: post.text.clone(),
            created_at: now,
            record_type: "app.bsky.feed.post".to_string(),
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
