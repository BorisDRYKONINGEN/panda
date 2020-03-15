//! Session

use crate::{
    error::Result,
    http::{HttpClient, DISCORD_URL},
    models::{
        channel::{Channel, Message},
        user::User,
    },
};

use std::sync::atomic::{AtomicBool, Ordering};

use async_std::sync::Mutex;
use isahc::ResponseExt;
use serde::Serialize;

/// The struct of the current session of the bot.
pub struct Session {
    id: Mutex<String>,
    pub(crate) http: HttpClient,

    #[allow(dead_code)]
    pub(crate) state: (),

    is_resumable: AtomicBool,
}

impl Session {
    pub(crate) fn new(token: String) -> Self {
        Session {
            id: Mutex::new("".into()),
            http: HttpClient::new(token),
            state: (),
            is_resumable: AtomicBool::new(true),
        }
    }

    /// Set the value to resumable field
    pub(crate) fn set_resumable(&self, b: bool) {
        self.is_resumable.store(b, Ordering::Relaxed);
    }

    /// Get the value of resumable field
    pub(crate) fn is_resumable(&self) -> bool {
        self.is_resumable.load(Ordering::Relaxed)
    }

    /// Set the value to id field
    pub(crate) async fn set_id(&self, id: String) {
        let mut session_id = self.id.lock().await;
        *session_id = id;
    }

    /// Get the value to id field
    pub(crate) async fn id(&self) -> String {
        let session_id = self.id.lock().await;
        session_id.clone()
    }

    // *******************************************************************************
    // * HTTP METHODS
    // *******************************************************************************

    /// Get a channel by ID. Returns a [`Channel`] object, it will fail if the ID it's invalid
    ///
    /// [`Channel`]: ../../panda/models/channel/struct.Channel.html
    pub async fn get_channel(&self, channel_id: impl AsRef<str>) -> Result<Channel> {
        // Parse URL
        let uri = format!("{}/channels/{}", DISCORD_URL, channel_id.as_ref());

        // Create RateLimit Key
        let rt_key = format!("channels:{}", channel_id.as_ref());

        let mut res = self.http.get(uri, rt_key).await?;

        // If an error wasn't returned, it's safe to unwrap
        Ok(res.json().unwrap())
    }

    /// Update a channel's settings. Requires the **MANAGE_CHANNELS** permission for the guild.
    /// Returns a [`Channel`] on success. It's recommended to use [`MessageEdit`] builder.
    /// Fires a [`ChannelUpdate`] event.
    ///
    /// [`Channel`]: ../../panda/models/channel/struct.Channel.html
    /// [`MessageEdit`]: ../../panda/utils/builder/struct.MessageEdit.html
    /// [`ChannelUpdate`]: ../../panda/models/gateway/events/struct.ChannelUpdate.html
    pub async fn edit_channel(&self, channel_id: impl AsRef<str>, body: impl Serialize) -> Result<Channel> {
        // Parse URL
        let uri = format!("{}/channels/{}", DISCORD_URL, channel_id.as_ref());

        // Create RateLimit Key
        let rt_key = format!("channels:{}", channel_id.as_ref());

        let body = serde_json::to_string(&body).unwrap();
        let mut res = self.http.patch(uri, rt_key, body).await?;

        // If an error wasn't returned, it's safe to unwrap
        Ok(res.json().unwrap())
    }

    /// Delete a channel, or close a private message. Requires the **MANAGE_CHANNELS** permission
    /// for the guild. Returns a [`Channel`] on success.
    /// Fires a [`ChannelDelete`] event.
    ///
    /// [`Channel`]: ../../panda/models/channel/struct.Channel.html
    /// [`ChannelDelete`]: ../../panda/models/gateway/events/struct.ChannelDelete.html
    pub async fn delete_channel(&self, channel_id: impl AsRef<str>) -> Result<Channel> {
        // Parse URL
        let uri = format!("{}/channels/{}", DISCORD_URL, channel_id.as_ref());

        // Create RateLimit Key
        let rt_key = format!("channels:{}", channel_id.as_ref());

        let mut res = self.http.delete(uri, rt_key).await?;

        // If an error wasn't returned, it's safe to unwrap
        Ok(res.json().unwrap())
    }

    /// Returns a Vec<[`Message`]> of a channel. If operating on a guild channel, this endpoint
    /// requires the **VIEW_CHANNEL** permission to be present on the current user.
    ///
    /// [`Channel`]: ../../panda/models/channel/struct.Channel.html
    pub async fn get_messages_around(
        &self,
        channel_id: impl AsRef<str>,
        msg_id: impl AsRef<str>,
        limit: u8,
    ) -> Result<Vec<Message>> {
        // Parse URL
        let uri = format!(
            "{}/channels/{}/messages?around={}&limit={}",
            DISCORD_URL,
            channel_id.as_ref(),
            msg_id.as_ref(),
            limit
        );

        // Create RateLimit Key
        let rt_key = format!("channels:{}", channel_id.as_ref());

        let mut res = self.http.get(uri, rt_key).await?;

        // If an error wasn't returned, it's safe to unwrap
        Ok(res.json().unwrap())
    }

    /// Returns a Vec<[`Message`]> of a channel. If operating on a guild channel, this endpoint
    /// requires the **VIEW_CHANNEL** permission to be present on the current user.
    ///
    /// [`Channel`]: ../../panda/models/channel/struct.Channel.html
    pub async fn get_messages_before(
        &self,
        channel_id: impl AsRef<str>,
        msg_id: impl AsRef<str>,
        limit: u8,
    ) -> Result<Vec<Message>> {
        // Parse URL
        let uri = format!(
            "{}/channels/{}/messages?before={}&limit={}",
            DISCORD_URL,
            channel_id.as_ref(),
            msg_id.as_ref(),
            limit
        );

        // Create RateLimit Key
        let rt_key = format!("channels:{}", channel_id.as_ref());

        let mut res = self.http.get(uri, rt_key).await?;

        Ok(res.json().unwrap())
    }

    /// Returns a Vec<[`Message`]> of a channel. If operating on a guild channel, this endpoint
    /// requires the **VIEW_CHANNEL** permission to be present on the current user.
    ///
    /// [`Channel`]: ../../panda/models/channel/struct.Channel.html
    pub async fn get_messages_after(
        &self,
        channel_id: impl AsRef<str>,
        msg_id: impl AsRef<str>,
        limit: u8,
    ) -> Result<Vec<Message>> {
        // Format uri
        let uri = format!(
            "{}/channels/{}/messages?after={}&limit={}",
            DISCORD_URL,
            channel_id.as_ref(),
            msg_id.as_ref(),
            limit
        );

        // Create RateLimit Key
        let rt_key = format!("channels:{}", channel_id.as_ref());

        let mut res = self.http.get(uri, rt_key).await?;

        Ok(res.json().unwrap())
    }

    /// Returns a specific [`Message`] in the channel. If operating on a guild channel, this endpoint
    /// requires the **READ_MESSAGE_HISTORY** permission to be present on the current user.
    ///
    /// [`Message`]: ../../panda/models/channel/struct.Message.html
    pub async fn get_message(&self, channel_id: impl AsRef<str>, msg_id: impl AsRef<str>) -> Result<Message> {
        let uri = format!(
            "{}/channel/{}/messages/{}",
            DISCORD_URL,
            channel_id.as_ref(),
            msg_id.as_ref()
        );

        // Create RateLimit Key
        let rt_key = format!("channels:{}", channel_id.as_ref());

        let mut res = self.http.get(uri, rt_key).await?;

        Ok(res.json().unwrap())
    }

    /// Creates a new message, and returns the [`Message`]. This will also trigger
    /// [`MessageCreate`] event
    ///
    /// [`Message`]: ../../panda/models/channel/struct.Message.html
    /// [`MessageCreate`]: ../../panda/models/gateway/events/struct.MessageCreate.html
    pub async fn send_message(&self, channel_id: impl AsRef<str>, content: impl AsRef<str>) -> Result<Message> {
        let uri = format!("{}/channels/{}/messages", DISCORD_URL, channel_id.as_ref());

        let msg = serde_json::json!({
            "content": content.as_ref(),
            "tts": "false"
        });

        // Create RateLimit Key
        let rt_key = format!("channels:{}", channel_id.as_ref());

        let msg = serde_json::to_string(&msg).unwrap();

        let mut res = self.http.post(uri, rt_key, msg).await?;

        // If an error wasn't returned, it's safe to unwrap
        Ok(res.json().unwrap())
    }

    /// Add a reaction to a [`Message`], it needs the [`Channel`] ID, and [`Message`] ID
    ///
    /// [`Channel`]: ../../panda/models/channel/struct.channel.html
    /// [`Message`]: ../../panda/models/channel/struct.Message.html
    pub async fn add_reaction(
        &self,
        channel_id: impl AsRef<str>,
        message_id: impl AsRef<str>,
        emoji: impl AsRef<str>,
    ) -> Result<()> {
        // Encode emoji
        let emoji = encode(emoji.as_ref());

        // Parse URL
        let uri = format!(
            "{}/channels/{}/messages/{}/reactions/{}/@me",
            DISCORD_URL,
            channel_id.as_ref(),
            message_id.as_ref(),
            emoji
        );

        // Create RateLimit Key
        let rt_key = format!("channel:{}:emoji", channel_id.as_ref());

        let _res = self.http.put(uri, rt_key).await?;

        Ok(())
    }

    /// Remove a own reaction to a [`Message`], it needs the [`Channel`] ID, and [`Message`] ID
    ///
    /// [`Channel`]: ../../panda/models/channel/struct.channel.html
    /// [`Message`]: ../../panda/models/channel/struct.Message.html
    pub async fn remove_own_reaction(
        &self,
        channel_id: impl AsRef<str>,
        message_id: impl AsRef<str>,
        emoji: impl AsRef<str>,
    ) -> Result<()> {
        // Encode emoji
        let emoji = encode(emoji.as_ref());

        // Parse URL
        let uri = format!(
            "{}/channels/{}/messages/{}/reactions/{}/@me",
            DISCORD_URL,
            channel_id.as_ref(),
            message_id.as_ref(),
            emoji
        );

        // Create RateLimit Key
        let rt_key = format!("channel:{}:emoji", channel_id.as_ref());

        let _res = self.http.delete(uri, rt_key).await?;

        Ok(())
    }

    /// Remove an [`User`] reaction to a [`Message`], it needs the [`Channel`] ID, [`Message`] ID
    /// and [`User`] ID.
    ///
    /// [`Channel`]: ../../panda/models/channel/struct.channel.html
    /// [`Message`]: ../../panda/models/channel/struct.Message.html
    /// [`User`]: ../../panda/models/user/struct.User.html
    pub async fn remove_user_reaction(
        &self,
        channel_id: impl AsRef<str>,
        message_id: impl AsRef<str>,
        user: impl AsRef<str>,
        emoji: impl AsRef<str>,
    ) -> Result<()> {
        // Encode emoji
        let emoji = encode(emoji.as_ref());

        // Parse URL
        let uri = format!(
            "{}/channels/{}/messages/{}/reactions/{}/{}",
            DISCORD_URL,
            channel_id.as_ref(),
            message_id.as_ref(),
            emoji,
            user.as_ref()
        );

        // Create RateLimit Key
        let rt_key = format!("channel:{}:emoji", channel_id.as_ref());

        let _res = self.http.delete(uri, rt_key).await?;

        Ok(())
    }

    /// Get all [`User`]s that reacted with given emoji to a [`Message`],
    /// it needs the [`Channel`] ID, [`Message`] ID
    ///
    /// [`Channel`]: ../../panda/models/channel/struct.channel.html
    /// [`Message`]: ../../panda/models/channel/struct.Message.html
    /// [`User`]: ../../panda/models/user/struct.User.html
    pub async fn get_reactions(
        &self,
        channel_id: impl AsRef<str>,
        message_id: impl AsRef<str>,
        emoji: impl AsRef<str>,
    ) -> Result<Vec<User>> {
        // Encode emoji
        let emoji = encode(emoji.as_ref());

        // Parse URL
        let uri = format!(
            "{}/channels/{}/messages/{}/reactions/{}",
            DISCORD_URL,
            channel_id.as_ref(),
            message_id.as_ref(),
            emoji,
        );

        // Create RateLimit Key
        let rt_key = format!("channel:{}:emoji", channel_id.as_ref());

        let mut res = self.http.get(uri, rt_key).await?;

        Ok(res.json().unwrap())
    }

    /// Deletes all reactions on a [`Message`]. This endpoint requires the **MANAGE_MESSAGES**
    /// permission to be present on the current user. Fires a [`MessageReactionRemoveAll`].
    ///
    /// [`Message`]: ../../panda/models/channel/struct.Message.html
    /// [`MessageReactionRemoveAll`]: ../../panda/models/gateway/events/struct.MessageReactionRemoveAll.html
    pub async fn remove_all_reactions(&self, channel_id: impl AsRef<str>, message_id: impl AsRef<str>) -> Result<()> {
        // Parse URL
        let uri = format!(
            "{}/channels/{}/messages/{}/reactions",
            DISCORD_URL,
            channel_id.as_ref(),
            message_id.as_ref(),
        );

        // Create RateLimit Key
        let rt_key = format!("channel:{}:emoji", channel_id.as_ref());

        let _res = self.http.delete(uri, rt_key).await?;

        Ok(())
    }

    // TODO: ADD THIS EVENT(NEW)
    /// Deletes all reactions on a [`Message`]. This endpoint requires the **MANAGE_MESSAGES**
    /// permission to be present on the current user. Fires a [`MessageReactionRemoveEmoji`].
    ///
    /// [`Message`]: ../../panda/models/channel/struct.Message.html
    /// [`MessageReactionRemoveEmoji`]: ../../panda/models/gateway/events/struct.MessageReactionRemoveEmoji.html
    pub async fn remove_all_emoji_reactions(
        &self,
        channel_id: impl AsRef<str>,
        message_id: impl AsRef<str>,
        emoji: impl AsRef<str>,
    ) -> Result<()> {
        let emoji = encode(emoji.as_ref());
        // Parse URL
        let uri = format!(
            "{}/channels/{}/messages/{}/reactions/{}",
            DISCORD_URL,
            channel_id.as_ref(),
            message_id.as_ref(),
            emoji
        );

        // Create RateLimit Key
        let rt_key = format!("channel:{}:emoji", channel_id.as_ref());

        let _res = self.http.delete(uri, rt_key).await?;

        Ok(())
    }

    /// Edits message, and returns the [`Message`]. This will also trigger
    /// [`MessageUpdate`] event
    ///
    /// [`Message`]: ../../panda/models/channel/struct.Message.html
    /// [`MessageUpdate`]: ../../panda/models/gateway/events/struct.MessageUpdate.html
    pub async fn edit_message(&self, channel_id: impl AsRef<str>, content: impl AsRef<str>) -> Result<Message> {
        // TODO: Make this functional
        let uri = format!("{}/channels/{}/messages", DISCORD_URL, channel_id.as_ref());

        let msg = serde_json::json!({
            "content": content.as_ref(),
            "tts": "false"
        });

        // Create RateLimit Key
        let rt_key = format!("channels:{}", channel_id.as_ref());

        let msg = serde_json::to_string(&msg).unwrap();

        let mut res = self.http.post(uri, rt_key, msg).await?;

        // If an error wasn't returned, it's safe to unwrap
        Ok(res.json().unwrap())
    }

    /// Delete a [`Message`], This will also trigger [`MessageDelete`] event
    ///
    /// [`Message`]: ../../panda/models/channel/struct.Message.html
    /// [`MessageDelete`]: ../../panda/models/gateway/events/struct.MessageDelete.html
    pub async fn delete_message(&self, channel_id: impl AsRef<str>, message_id: impl AsRef<str>) -> Result<()> {
        // Parse URL
        let uri = format!(
            "{}/channels/{}/messages/{}",
            DISCORD_URL,
            channel_id.as_ref(),
            message_id.as_ref()
        );

        // Create RateLimit Key
        let rt_key = format!("channels:{}", channel_id.as_ref());

        let _res = self.http.delete(uri, rt_key).await?;

        // If an error wasn't returned, it's safe to unwrap
        Ok(())
    }

    /// Delete a a bulk of [`Message`] (2 - 100), This will also trigger [`MessageDeleteBulk`] event.
    ///
    /// [`Message`]: ../../panda/models/channel/struct.Message.html
    /// [`MessageDelete`]: ../../panda/models/gateway/events/struct.MessageDelete.html
    pub async fn delete_many_messages(&self, channel_id: impl AsRef<str>, messages: &[&str]) -> Result<()> {
        // Parse URL
        let uri = format!("{}/channels/{}/messages/bulk-delete", DISCORD_URL, channel_id.as_ref(),);

        let body = serde_json::json!({ "messages": messages });
        let msg = serde_json::to_string(&body).unwrap();

        // Create RateLimit Key
        let rt_key = format!("channels:{}", channel_id.as_ref());

        let _res = self.http.post(uri, rt_key, msg).await?;

        Ok(())
    }

    /// Edit the channel permission overwrites for a user or role in a channel. Only usable
    /// for guild channels. Requires the **MANAGE_ROLES** permission.
    ///
    /// [`Message`]: ../../panda/models/channel/struct.Message.html
    /// [`MessageDelete`]: ../../panda/models/gateway/events/struct.MessageDelete.html
    pub async fn edit_channel_permissions(&self, channel_id: impl AsRef<str>) -> Result<()> {
        // Parse URL
        let uri = format!("{}/channels/{}/permissions/{}", DISCORD_URL, channel_id.as_ref(), "");

        // Create RateLimit Key
        let rt_key = format!("channels:{}", channel_id.as_ref());

        let _res = self.http.get(uri, rt_key).await?;

        Ok(())
    }
}

/// Used to encode emoji as a valid char in URL
fn encode(data: &str) -> String {
    let mut escaped = String::new();
    for b in data.as_bytes().iter() {
        match *b as char {
            // Accepted characters
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => escaped.push(*b as char),

            // Everything else is percent-encoded
            b => escaped.push_str(format!("%{:02X}", b as u32).as_str()),
        };
    }
    return escaped;
}
