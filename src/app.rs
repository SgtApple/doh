// SPDX-License-Identifier: MIT

use crate::config::Config;
use crate::credentials::Credentials;
use crate::post_manager::PostManager;
use cosmic::cosmic_config::{self, CosmicConfigEntry};
use cosmic::iced::{window::Id, Limits, Subscription};
use cosmic::iced::widget::text_editor::{Content, Action};
use cosmic::iced_winit::commands::popup::{destroy_popup, get_popup};
use cosmic::prelude::*;
use cosmic::widget;
use futures_util::SinkExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformType {
    X,
    BlueSky,
    Nostr,
    Mastodon,
}

/// The application model stores app-specific state used to describe its interface and
/// drive its logic.
pub struct AppModel {
    /// Application state which is managed by the COSMIC runtime.
    core: cosmic::Core,
    /// The popup id.
    popup: Option<Id>,
    /// Configuration data that persists between application runs.
    config: Config,
    /// Credentials loaded from keyring
    credentials: Credentials,
    
    // UI State
    /// Current view
    current_view: ViewState,
    /// Text content to post
    post_text: String,
    /// Text editor content for multiline post input
    text_editor_content: Content,
    /// Emoji picker visibility
    show_emoji_picker: bool,
    /// Selected image paths
    image_paths: Vec<std::path::PathBuf>,
    /// Platform selection
    post_to_x: bool,
    post_to_bluesky: bool,
    post_to_nostr: bool,
    post_to_mastodon: bool,
    /// Posting status
    posting: bool,
    status_message: String,
    
    // Settings UI State
    // X/Twitter
    twitter_consumer_key: String,
    twitter_consumer_secret: String,
    twitter_access_token: String,
    twitter_access_secret: String,
    // BlueSky
    bluesky_handle: String,
    bluesky_password: String,
    // Nostr
    nostr_nsec: String,
    nostr_relays: String,
    nostr_use_pleb_signer: bool,
    nostr_blossom_server: String,
    // Mastodon
    mastodon_instance_url: String,
    mastodon_access_token: String,
    
    // Collapsible section state
    twitter_section_expanded: bool,
    bluesky_section_expanded: bool,
    nostr_section_expanded: bool,
    mastodon_section_expanded: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewState {
    Main,
    Settings,
}

/// Messages emitted by the application and its widgets.
#[derive(Debug, Clone)]
pub enum Message {
    TogglePopup,
    PopupClosed(Id),
    SubscriptionChannel,
    UpdateConfig(Config),
    
    // UI Messages
    TextChanged(String),
    TextEditorAction(Action),
    ToggleEmojiPicker,
    InsertEmoji(String),
    SelectImages,
    ImagesSelected(Vec<std::path::PathBuf>),
    TogglePlatform(PlatformType, bool),
    PostClicked,
    PostCompleted(Vec<(String, bool, String)>), // (platform, success, message)
    
    // Settings
    ShowSettings,
    ShowMain,
    SaveCredentials,
    
    // Credential editing
    TwitterConsumerKeyChanged(String),
    TwitterConsumerSecretChanged(String),
    TwitterAccessTokenChanged(String),
    TwitterAccessSecretChanged(String),
    BlueSkyHandleChanged(String),
    BlueSkyPasswordChanged(String),
    NostrNsecChanged(String),
    NostrRelaysChanged(String),
    NostrTogglePlebSigner(bool),
    NostrBlossomServerChanged(String),
    MastodonInstanceUrlChanged(String),
    MastodonAccessTokenChanged(String),
    
    // Section toggles
    ToggleTwitterSection,
    ToggleBlueSkySection,
    ToggleNostrSection,
    ToggleMastodonSection,
}

/// Create a COSMIC application from the app model
impl cosmic::Application for AppModel {
    /// The async executor that will be used to run your application's commands.
    type Executor = cosmic::executor::Default;

    /// Data that your application receives to its init method.
    type Flags = ();

    /// Messages which the application and its widgets will emit.
    type Message = Message;

    /// Unique identifier in RDNN (reverse domain name notation) format.
    const APP_ID: &'static str = "com.sgtapple.doh";

    fn core(&self) -> &cosmic::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }

    /// Initializes the application with any given flags and startup commands.
    fn init(
        core: cosmic::Core,
        _flags: Self::Flags,
    ) -> (Self, Task<cosmic::Action<Self::Message>>) {
        // Load credentials from keyring
        let credentials = Credentials::load().unwrap_or_default();
        
        // Initialize UI fields from credentials
        let twitter_consumer_key = credentials.twitter_consumer_key.clone().unwrap_or_default();
        let twitter_consumer_secret = credentials.twitter_consumer_secret.clone().unwrap_or_default();
        let twitter_access_token = credentials.twitter_access_token.clone().unwrap_or_default();
        let twitter_access_secret = credentials.twitter_access_secret.clone().unwrap_or_default();
        let bluesky_handle = credentials.bluesky_handle.clone().unwrap_or_default();
        let bluesky_password = credentials.bluesky_app_password.clone().unwrap_or_default();
        let nostr_nsec = credentials.nostr_nsec.clone().unwrap_or_default();
        let nostr_relays = credentials.nostr_relays.join(", ");
        let nostr_use_pleb_signer = credentials.nostr_use_pleb_signer;
        let nostr_blossom_server = credentials.nostr_image_host_url.clone().unwrap_or_default();
        let mastodon_instance_url = credentials.mastodon_instance_url.clone().unwrap_or_default();
        let mastodon_access_token = credentials.mastodon_access_token.clone().unwrap_or_default();
        
        // Load config
        let config = cosmic_config::Config::new(Self::APP_ID, Config::VERSION)
            .map(|context| match Config::get_entry(&context) {
                Ok(config) => config,
                Err((_errors, config)) => config,
            })
            .unwrap_or_default();
        
        // Construct the app model with the runtime's core.
        let app = AppModel {
            core,
            credentials,
            popup: None,
            current_view: ViewState::Main,
            post_text: String::new(),
            text_editor_content: Content::new(),
            show_emoji_picker: false,
            image_paths: Vec::new(),
            post_to_x: config.post_to_x,
            post_to_bluesky: config.post_to_bluesky,
            post_to_nostr: config.post_to_nostr,
            post_to_mastodon: config.post_to_mastodon,
            posting: false,
            status_message: String::new(),
            config,
            twitter_consumer_key,
            twitter_consumer_secret,
            twitter_access_token,
            twitter_access_secret,
            bluesky_handle,
            bluesky_password,
            nostr_nsec,
            nostr_relays,
            nostr_use_pleb_signer,
            nostr_blossom_server,
            mastodon_instance_url,
            mastodon_access_token,
            twitter_section_expanded: false,
            bluesky_section_expanded: false,
            nostr_section_expanded: false,
            mastodon_section_expanded: false,
        };

        (app, Task::none())
    }

    fn on_close_requested(&self, id: Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    /// Describes the interface based on the current state of the application model.
    ///
    /// The applet's button in the panel will be drawn using the main view method.
    /// This view should emit messages to toggle the applet's popup window, which will
    /// be drawn using the `view_window` method.
    fn view(&self) -> Element<'_, Self::Message> {
        self.core
            .applet
            .icon_button("com.sgtapple.doh")
            .on_press(Message::TogglePopup)
            .into()
    }

    /// The applet's popup window will be drawn using this view method. If there are
    /// multiple poups, you may match the id parameter to determine which popup to
    /// create a view for.
    fn view_window(&self, _id: Id) -> Element<'_, Self::Message> {
        let content = match self.current_view {
            ViewState::Main => self.view_main(),
            ViewState::Settings => self.view_settings(),
        };
        
        self.core.applet.popup_container(content).into()
    }

    /// Register subscriptions for this application.
    ///
    /// Subscriptions are long-lived async tasks running in the background which
    /// emit messages to the application through a channel. They may be conditionally
    /// activated by selectively appending to the subscription batch, and will
    /// continue to execute for the duration that they remain in the batch.
    fn subscription(&self) -> Subscription<Self::Message> {
        struct MySubscription;

        Subscription::batch(vec![
            // Create a subscription which emits updates through a channel.
            Subscription::run_with_id(
                std::any::TypeId::of::<MySubscription>(),
                cosmic::iced::stream::channel(4, move |mut channel| async move {
                    _ = channel.send(Message::SubscriptionChannel).await;

                    futures_util::future::pending().await
                }),
            ),
            // Watch for application configuration changes.
            self.core()
                .watch_config::<Config>(Self::APP_ID)
                .map(|update| {
                    // for why in update.errors {
                    //     tracing::error!(?why, "app config error");
                    // }

                    Message::UpdateConfig(update.config)
                }),
        ])
    }

    /// Handles messages emitted by the application and its widgets.
    ///
    /// Tasks may be returned for asynchronous execution of code in the background
    /// on the application's async runtime. The application will not exit until all
    /// tasks are finished.
    fn update(&mut self, message: Self::Message) -> Task<cosmic::Action<Self::Message>> {
        match message {
            Message::SubscriptionChannel => {
                // For example purposes only.
            }
            Message::UpdateConfig(config) => {
                self.config = config;
            }
            Message::TextChanged(text) => {
                self.post_text = text;
            }
            Message::TextEditorAction(action) => {
                self.text_editor_content.perform(action);
                self.post_text = self.text_editor_content.text();
            }
            Message::ToggleEmojiPicker => {
                self.show_emoji_picker = !self.show_emoji_picker;
            }
            Message::InsertEmoji(emoji) => {
                // Append emoji to the end of the text
                let current_text = self.text_editor_content.text();
                let new_text = format!("{}{}", current_text, emoji);
                self.text_editor_content = Content::with_text(&new_text);
                self.post_text = new_text;
                self.show_emoji_picker = false;
            }
            Message::SelectImages => {
                return Task::future(
                    async {
                        let paths = rfd::AsyncFileDialog::new()
                            .add_filter("images", &["png", "jpg", "jpeg", "gif", "webp"])
                            .set_title("Select Images")
                            .pick_files()
                            .await
                            .map(|files| files.into_iter().map(|f| f.path().to_path_buf()).collect())
                            .unwrap_or_default();
                        cosmic::Action::App(Message::ImagesSelected(paths))
                    }
                );
            }
            Message::ImagesSelected(paths) => {
                self.image_paths = paths;
            }
            Message::TogglePlatform(platform, enabled) => {
                match platform {
                    PlatformType::X => self.post_to_x = enabled,
                    PlatformType::BlueSky => self.post_to_bluesky = enabled,
                    PlatformType::Nostr => self.post_to_nostr = enabled,
                    PlatformType::Mastodon => self.post_to_mastodon = enabled,
                }
                
                // Update config to persist platform selections
                self.config.post_to_x = self.post_to_x;
                self.config.post_to_bluesky = self.post_to_bluesky;
                self.config.post_to_nostr = self.post_to_nostr;
                self.config.post_to_mastodon = self.post_to_mastodon;
                
                // Save config
                if let Ok(config_helper) = cosmic_config::Config::new(Self::APP_ID, Config::VERSION) {
                    let _ = self.config.write_entry(&config_helper);
                }
            }
            Message::PostClicked => {
                if self.post_text.is_empty() {
                    self.status_message = "Please enter some text".to_string();
                    return Task::none();
                }
                
                let any_platform_selected = self.post_to_x 
                    || self.post_to_bluesky 
                    || self.post_to_nostr 
                    || self.post_to_mastodon;
                
                if !any_platform_selected {
                    self.status_message = "Please select at least one platform".to_string();
                    return Task::none();
                }
                
                self.posting = true;
                self.status_message = "Posting...".to_string();
                
                // Collect selected platforms
                let mut platforms = Vec::new();
                if self.post_to_x { platforms.push("X".to_string()); }
                if self.post_to_bluesky { platforms.push("BlueSky".to_string()); }
                if self.post_to_nostr { platforms.push("Nostr".to_string()); }
                if self.post_to_mastodon { platforms.push("Mastodon".to_string()); }
                
                let text = self.post_text.clone();
                let image_paths = self.image_paths.clone();
                let credentials = self.credentials.clone();
                
                return Task::future(async move {
                    // Load images from paths
                    let mut images = Vec::new();
                    for path in image_paths {
                        match std::fs::read(&path) {
                            Ok(bytes) => {
                                eprintln!("[App] Loaded image: {} ({} bytes)", path.display(), bytes.len());
                                images.push(bytes);
                            }
                            Err(e) => {
                                eprintln!("[App] Failed to load image {}: {}", path.display(), e);
                            }
                        }
                    }
                    
                    let manager = PostManager::new(credentials);
                    let results = manager.post(text, images, platforms).await;
                    cosmic::Action::App(Message::PostCompleted(results))
                });
            }
            Message::PostCompleted(results) => {
                self.posting = false;
                let success_count = results.iter().filter(|(_, success, _)| *success).count();
                let total = results.len();
                self.status_message = format!(
                    "Posted to {}/{} platforms. {}",
                    success_count,
                    total,
                    if success_count < total { "Check logs for errors." } else { "" }
                );
                
                // Clear input box if all posts were successful
                if success_count == total && total > 0 {
                    self.post_text.clear();
                    self.text_editor_content = Content::new();
                    self.image_paths.clear();
                }
            }
            Message::ShowSettings => {
                self.current_view = ViewState::Settings;
            }
            Message::ShowMain => {
                self.current_view = ViewState::Main;
            }
            Message::SaveCredentials => {
                eprintln!("[GUI] SaveCredentials button clicked!");
                // Update credentials from UI fields
                self.credentials.twitter_consumer_key = if self.twitter_consumer_key.is_empty() {
                    None
                } else {
                    Some(self.twitter_consumer_key.clone())
                };
                self.credentials.twitter_consumer_secret = if self.twitter_consumer_secret.is_empty() {
                    None
                } else {
                    Some(self.twitter_consumer_secret.clone())
                };
                self.credentials.twitter_access_token = if self.twitter_access_token.is_empty() {
                    None
                } else {
                    Some(self.twitter_access_token.clone())
                };
                self.credentials.twitter_access_secret = if self.twitter_access_secret.is_empty() {
                    None
                } else {
                    Some(self.twitter_access_secret.clone())
                };
                self.credentials.bluesky_handle = if self.bluesky_handle.is_empty() {
                    None
                } else {
                    Some(self.bluesky_handle.clone())
                };
                self.credentials.bluesky_app_password = if self.bluesky_password.is_empty() {
                    None
                } else {
                    Some(self.bluesky_password.clone())
                };
                self.credentials.nostr_nsec = if self.nostr_nsec.is_empty() {
                    None
                } else {
                    Some(self.nostr_nsec.clone())
                };
                self.credentials.nostr_relays = if self.nostr_relays.is_empty() {
                    Vec::new()
                } else {
                    self.nostr_relays.split(',').map(|s| s.trim().to_string()).collect()
                };
                self.credentials.nostr_use_pleb_signer = self.nostr_use_pleb_signer;
                self.credentials.nostr_image_host_url = if self.nostr_blossom_server.is_empty() {
                    None
                } else {
                    Some(self.nostr_blossom_server.clone())
                };
                self.credentials.mastodon_instance_url = if self.mastodon_instance_url.is_empty() {
                    None
                } else {
                    Some(self.mastodon_instance_url.clone())
                };
                self.credentials.mastodon_access_token = if self.mastodon_access_token.is_empty() {
                    None
                } else {
                    Some(self.mastodon_access_token.clone())
                };
                
                eprintln!("[GUI] About to call credentials.save()");
                eprintln!("[GUI] nostr_use_pleb_signer = {}", self.credentials.nostr_use_pleb_signer);
                eprintln!("[GUI] nostr_nsec = {:?}", self.credentials.nostr_nsec);
                if let Err(e) = self.credentials.save() {
                    eprintln!("[GUI] Save FAILED: {}", e);
                    self.status_message = format!("Failed to save credentials: {}", e);
                } else {
                    eprintln!("[GUI] Save SUCCESS!");
                    self.status_message = "Credentials saved!".to_string();
                }
            }
            Message::TwitterConsumerKeyChanged(value) => {
                self.twitter_consumer_key = value;
            }
            Message::TwitterConsumerSecretChanged(value) => {
                self.twitter_consumer_secret = value;
            }
            Message::TwitterAccessTokenChanged(value) => {
                self.twitter_access_token = value;
            }
            Message::TwitterAccessSecretChanged(value) => {
                self.twitter_access_secret = value;
            }
            Message::BlueSkyHandleChanged(value) => {
                self.bluesky_handle = value;
            }
            Message::BlueSkyPasswordChanged(value) => {
                self.bluesky_password = value;
            }
            Message::NostrNsecChanged(value) => {
                self.nostr_nsec = value;
            }
            Message::NostrRelaysChanged(value) => {
                self.nostr_relays = value;
            }
            Message::NostrTogglePlebSigner(value) => {
                self.nostr_use_pleb_signer = value;
            }
            Message::NostrBlossomServerChanged(value) => {
                self.nostr_blossom_server = value;
            }
            Message::ToggleTwitterSection => {
                self.twitter_section_expanded = !self.twitter_section_expanded;
            }
            Message::ToggleBlueSkySection => {
                self.bluesky_section_expanded = !self.bluesky_section_expanded;
            }
            Message::ToggleNostrSection => {
                self.nostr_section_expanded = !self.nostr_section_expanded;
            }
            Message::MastodonInstanceUrlChanged(value) => {
                self.mastodon_instance_url = value;
            }
            Message::MastodonAccessTokenChanged(value) => {
                self.mastodon_access_token = value;
            }
            Message::ToggleMastodonSection => {
                self.mastodon_section_expanded = !self.mastodon_section_expanded;
            }
            Message::TogglePopup => {
                return if let Some(p) = self.popup.take() {
                    destroy_popup(p)
                } else {
                    let new_id = Id::unique();
                    self.popup.replace(new_id);
                    let mut popup_settings = self.core.applet.get_popup_settings(
                        self.core.main_window_id().unwrap(),
                        new_id,
                        None,
                        None,
                        None,
                    );
                    popup_settings.positioner.size_limits = Limits::NONE
                        .max_width(450.0)
                        .min_width(400.0)
                        .min_height(400.0)
                        .max_height(800.0);
                    get_popup(popup_settings)
                }
            }
            Message::PopupClosed(id) => {
                if self.popup.as_ref() == Some(&id) {
                    self.popup = None;
                }
            }
        }
        Task::none()
    }

    fn style(&self) -> Option<cosmic::iced_runtime::Appearance> {
        Some(cosmic::applet::style())
    }
}

impl AppModel {
    fn view_main(&self) -> widget::Column<'_, Message> {
        let char_count = self.post_text.chars().count();
        
        let mut content_list = widget::column()
            .padding(10)
            .spacing(10)
            .push(
                widget::row()
                    .spacing(10)
                    .push(widget::text::body("Post to social media").size(18))
                    .push(widget::horizontal_space())
                    .push(
                        widget::button::icon(widget::icon::from_name("preferences-system-symbolic"))
                            .on_press(Message::ShowSettings)
                    )
            )
            .push(
                widget::text_editor(&self.text_editor_content)
                    .placeholder("What's happening?")
                    .height(100)
                    .on_action(Message::TextEditorAction)
            )
            .push(
                widget::row()
                    .spacing(10)
                    .push(
                        widget::button::icon(widget::icon::from_name("face-smile-symbolic"))
                            .on_press(Message::ToggleEmojiPicker)
                    )
                    .push(widget::text::caption(format!("{} characters", char_count)))
            )
            .push(
                widget::button::text("Add Images")
                    .on_press(Message::SelectImages)
            )
            .push(
                widget::text::caption(format!("{} image(s) selected", self.image_paths.len()))
            )
            .push(widget::divider::horizontal::default())
            .push(widget::text::body("Post to:"));
        
        // Add platform toggles with status indicators
        if self.credentials.has_twitter() {
            content_list = content_list.push(
                widget::settings::item(
                    "X (Twitter) ✓",
                    widget::toggler(self.post_to_x)
                        .on_toggle(|v| Message::TogglePlatform(PlatformType::X, v)),
                )
            );
        } else {
            content_list = content_list.push(
                widget::text::caption("X (Twitter) - Not configured")
            );
        }
        
        if self.credentials.has_bluesky() {
            content_list = content_list.push(
                widget::settings::item(
                    "BlueSky ✓",
                    widget::toggler(self.post_to_bluesky)
                        .on_toggle(|v| Message::TogglePlatform(PlatformType::BlueSky, v)),
                )
            );
        } else {
            content_list = content_list.push(
                widget::text::caption("BlueSky - Not configured")
            );
        }
        
        if self.credentials.has_nostr() {
            content_list = content_list.push(
                widget::settings::item(
                    "Nostr ✓",
                    widget::toggler(self.post_to_nostr)
                        .on_toggle(|v| Message::TogglePlatform(PlatformType::Nostr, v)),
                )
            );
        } else {
            content_list = content_list.push(
                widget::text::caption("Nostr - Not configured")
            );
        }
        
        if self.credentials.has_mastodon() {
            content_list = content_list.push(
                widget::settings::item(
                    "Mastodon ✓",
                    widget::toggler(self.post_to_mastodon)
                        .on_toggle(|v| Message::TogglePlatform(PlatformType::Mastodon, v)),
                )
            );
        } else {
            content_list = content_list.push(
                widget::text::caption("Mastodon - Not configured")
            );
        }
        
        content_list = content_list.push(widget::divider::horizontal::default());
        
        let button_element: Element<'_, Message> = if self.posting {
            widget::button::text("Posting...").into()
        } else {
            widget::button::suggested("Post")
                .on_press(Message::PostClicked)
                .into()
        };
        
        content_list = content_list.push(button_element);
        
        if !self.status_message.is_empty() {
            content_list = content_list.push(widget::text::caption(&self.status_message));
        }
        
        // Add emoji picker modal if visible
        if self.show_emoji_picker {
            content_list = content_list.push(self.view_emoji_picker());
        }
        
        content_list
    }
    
    fn view_emoji_picker(&self) -> Element<'_, Message> {
        // Create a grid of emoji buttons
        let emojis_list = emojis::Group::SmileysAndEmotion.emojis()
            .chain(emojis::Group::PeopleAndBody.emojis())
            .chain(emojis::Group::AnimalsAndNature.emojis())
            .chain(emojis::Group::FoodAndDrink.emojis())
            .chain(emojis::Group::TravelAndPlaces.emojis())
            .chain(emojis::Group::Activities.emojis())
            .chain(emojis::Group::Objects.emojis())
            .chain(emojis::Group::Symbols.emojis())
            .chain(emojis::Group::Flags.emojis())
            .take(100); // Limit to first 100 emojis for performance
        
        let mut emoji_grid = widget::column().spacing(5).padding(10);
        let mut current_row = widget::row().spacing(5);
        let mut count = 0;
        
        for emoji in emojis_list {
            current_row = current_row.push(
                widget::button::text(emoji.as_str())
                    .on_press(Message::InsertEmoji(emoji.as_str().to_string()))
            );
            count += 1;
            if count % 8 == 0 {
                emoji_grid = emoji_grid.push(current_row);
                current_row = widget::row().spacing(5);
            }
        }
        
        // Add any remaining emojis
        if count % 8 != 0 {
            emoji_grid = emoji_grid.push(current_row);
        }
        
        widget::container(
            widget::column()
                .spacing(10)
                .push(
                    widget::row()
                        .spacing(10)
                        .push(widget::text::body("Select Emoji"))
                        .push(widget::horizontal_space())
                        .push(
                            widget::button::icon(widget::icon::from_name("window-close-symbolic"))
                                .on_press(Message::ToggleEmojiPicker)
                        )
                )
                .push(widget::scrollable(emoji_grid).height(200))
        )
        .padding(10)
        .into()
    }
    
    fn view_settings(&self) -> widget::Column<'_, Message> {
        let mut content = widget::column()
            .padding(10)
            .spacing(10)
            .push(
                widget::row()
                    .spacing(10)
                    .push(
                        widget::button::icon(widget::icon::from_name("go-previous-symbolic"))
                            .on_press(Message::ShowMain)
                    )
                    .push(widget::text::body("Account Settings").size(18))
            )
            .push(widget::divider::horizontal::default());
        
        // X/Twitter Section (Collapsible)
        content = content.push(
            widget::row()
                .spacing(8)
                .push(
                    widget::button::text(if self.twitter_section_expanded { "▼" } else { "▶" })
                        .on_press(Message::ToggleTwitterSection)
                )
                .push(widget::text::heading("X (Twitter)"))
        );
        
        if self.twitter_section_expanded {
            content = content
                .push(
                    widget::text_input("Consumer Key", &self.twitter_consumer_key)
                        .on_input(Message::TwitterConsumerKeyChanged)
                )
                .push(
                    widget::text_input("Consumer Secret", &self.twitter_consumer_secret)
                        .on_input(Message::TwitterConsumerSecretChanged)
                        .password()
                )
                .push(
                    widget::text_input("Access Token", &self.twitter_access_token)
                        .on_input(Message::TwitterAccessTokenChanged)
                )
                .push(
                    widget::text_input("Access Token Secret", &self.twitter_access_secret)
                        .on_input(Message::TwitterAccessSecretChanged)
                        .password()
                )
                .push(widget::text::caption("Get API keys from developer.twitter.com"));
        }
        content = content.push(widget::divider::horizontal::default());
        
        // BlueSky Section (Collapsible)
        content = content.push(
            widget::row()
                .spacing(8)
                .push(
                    widget::button::text(if self.bluesky_section_expanded { "▼" } else { "▶" })
                        .on_press(Message::ToggleBlueSkySection)
                )
                .push(widget::text::heading("BlueSky"))
        );
        
        if self.bluesky_section_expanded {
            content = content
                .push(
                    widget::text_input("Handle (e.g., user.bsky.social)", &self.bluesky_handle)
                        .on_input(Message::BlueSkyHandleChanged)
                )
                .push(
                    widget::text_input("App Password", &self.bluesky_password)
                        .on_input(Message::BlueSkyPasswordChanged)
                        .password()
                )
                .push(widget::text::caption("Create app password at bsky.app/settings/app-passwords"));
        }
        content = content.push(widget::divider::horizontal::default());
        
        // Nostr Section (Collapsible)
        content = content.push(
            widget::row()
                .spacing(8)
                .push(
                    widget::button::text(if self.nostr_section_expanded { "▼" } else { "▶" })
                        .on_press(Message::ToggleNostrSection)
                )
                .push(widget::text::heading("Nostr"))
        );
        
        if self.nostr_section_expanded {
            content = content.push(
                widget::settings::item(
                    "Use Pleb Signer (via D-Bus)",
                    widget::toggler(self.nostr_use_pleb_signer)
                        .on_toggle(Message::NostrTogglePlebSigner),
                )
            );
            
            if !self.nostr_use_pleb_signer {
                content = content
                    .push(
                        widget::text_input("nsec key", &self.nostr_nsec)
                            .on_input(Message::NostrNsecChanged)
                            .password()
                    )
                    .push(widget::text::caption("Your private Nostr key (nsec1...)"));
            }
            
            content = content
                .push(
                    widget::text_input("Relays (comma-separated)", &self.nostr_relays)
                        .on_input(Message::NostrRelaysChanged)
                )
                .push(widget::text::caption("Leave empty for defaults: relay.primal.net, relay.damus.io, relay.pleb.one"))
                .push(
                    widget::text_input("Blossom Server URL", &self.nostr_blossom_server)
                        .on_input(Message::NostrBlossomServerChanged)
                )
                .push(widget::text::caption("URL for uploading images (e.g., https://blossom.primal.net)"));
        }
        content = content.push(widget::divider::horizontal::default());
        
        // Mastodon Section (Collapsible)
        content = content.push(
            widget::row()
                .spacing(8)
                .push(
                    widget::button::text(if self.mastodon_section_expanded { "▼" } else { "▶" })
                        .on_press(Message::ToggleMastodonSection)
                )
                .push(widget::text::heading("Mastodon"))
        );
        
        if self.mastodon_section_expanded {
            content = content
                .push(
                    widget::text_input("Instance URL (e.g., https://mastodon.social)", &self.mastodon_instance_url)
                        .on_input(Message::MastodonInstanceUrlChanged)
                )
                .push(
                    widget::text_input("Access Token", &self.mastodon_access_token)
                        .on_input(Message::MastodonAccessTokenChanged)
                        .password()
                )
                .push(widget::text::caption("Get token from your instance: Preferences → Development → New Application"));
        }
        content = content.push(widget::divider::horizontal::default());
        
        // Save button - Always visible
        content = content
            .push(
                widget::button::suggested("Save Credentials")
                    .on_press(Message::SaveCredentials)
            );
        
        if !self.status_message.is_empty() {
            content = content.push(widget::text::caption(&self.status_message));
        }
        
        content
    }
}
