// SPDX-License-Identifier: MIT

//! Platform adapters for social media services

use anyhow::Result;

pub mod nostr;
pub mod bluesky;
pub mod twitter;
pub mod mastodon;

/// Represents a post with text and optional images
#[derive(Debug, Clone)]
pub struct Post {
    pub text: String,
    pub images: Vec<Vec<u8>>,
}

/// Result of posting to a platform
#[derive(Debug, Clone)]
pub enum PostResult {
    Success { url: Option<String> },
    Error { message: String },
}

/// Abstract platform adapter trait
pub trait Platform: Send + Sync {
    /// Get the platform name
    fn name(&self) -> &'static str;
    
    /// Check if the platform is authenticated/ready
    fn is_authenticated(&self) -> impl std::future::Future<Output = bool> + Send;
    
    /// Post content to the platform
    fn post(&self, post: &Post) -> impl std::future::Future<Output = Result<PostResult>> + Send;
}
