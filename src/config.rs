// SPDX-License-Identifier: MIT

use cosmic::cosmic_config::{self, cosmic_config_derive::CosmicConfigEntry, CosmicConfigEntry};

#[derive(Debug, Clone, CosmicConfigEntry, Eq, PartialEq)]
#[version = 1]
pub struct Config {
    pub post_to_x: bool,
    pub post_to_bluesky: bool,
    pub post_to_nostr: bool,
    pub post_to_mastodon: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            post_to_x: false,
            post_to_bluesky: false,
            post_to_nostr: false,
            post_to_mastodon: false,
        }
    }
}
