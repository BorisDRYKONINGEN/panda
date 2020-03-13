//! # Panda Client

//modules
mod config;
mod handler;
mod session;

pub use config::Config;
use handler::EventHandler;
pub use session::Session;

use crate::{
    error::{DiscordError, Result},
    gateway::{heartbeat, GatewayConnection},
    models::gateway::{
        commands::Command,
        events::*,
        events::{DispatchEvent, Event},
    },
};

use async_std::{sync::Arc, task};
use futures::{sink::SinkExt, stream::StreamExt, FutureExt};
use std::future::Future;

/// This macro it's used to handle all dispatched events of handler::EventHandler
macro_rules! handle_event {
    ($client: ident, $kind: ident, $event: expr) => {
        if let Some(func) = &($client).handler.$kind {
            let session = $client.session.clone();
            task::spawn(func(session, $event));
        }
    };
}

/// This macro it's used to create all "on_EVENT" methods to add a event handler
macro_rules! on_event_fn {
    ($(#[$doc: meta])* $name: ident, $event: ident, $event_enum: ty) => {
        $(#[$doc])*
        pub fn $name<F, Fut>(&mut self, func: F)
        where
            F: Fn(Arc<Session>, $event_enum) -> Fut + Sync + Send + 'static,
            Fut: Future<Output=()> + Send + 'static
        {
            self.handler.$event = Some(Box::new(move |m, r| func(m, r).boxed() ))
        }
    };
}

/// Client it's the main struct of Discord library, it receives and handle all discord events
pub struct Client {
    handler: EventHandler,
    config: Config,
    token: String,
    // Session will be shared between tasks, and it will be passed to the handler events
    session: Arc<Session>,
    gateway: GatewayConnection,
}

impl Client {
    /// Create a new "discord" Client with the default configs
    pub async fn new(token: impl Into<String>) -> Result<Self> {
        // Create a new gateway connection
        let gateway = GatewayConnection::new().await?;

        // Add Bot prefix to the token if it doesn't have
        let mut token = token.into();
        if !token.starts_with("Bot ") {
            token.insert_str(0, "Bot ");
        }

        Ok(Self {
            handler: EventHandler::new(),
            config: Config::new_default(),
            token: token.clone(),
            session: Arc::new(Session::new(token)),
            gateway,
        })
    }

    /// Create a new "discord" Client with personalized configs
    pub fn new_with_config() {}

    /// Start the bot connection process
    pub async fn start(&mut self) -> Result<()> {
        // Send identify and spawn heartbeater
        self.clean_connect().await;

        // Connection loop
        loop {
            if let Some(event) = self.gateway.from_gateway.next().await {
                match event {
                    Event::Dispatch(d) => match d {
                        DispatchEvent::Ready(e) => {
                            // Save session id
                            let id = e.session_id.clone();
                            self.session.set_id(id).await;

                            handle_event!(self, ready, e);
                            // if let Some(f) = &self.handler.ready {
                            //     let session = self.session.clone();
                            //     task::spawn(f(session, e));
                            // }
                        }
                        // Channel
                        DispatchEvent::ChannelCreate(e) => {
                            handle_event!(self, channel_create, e);
                        }
                        DispatchEvent::ChannelUpdate(e) => {
                            handle_event!(self, channel_update, e);
                        }
                        DispatchEvent::ChannelDelete(e) => {
                            handle_event!(self, channel_delete, e);
                        }
                        DispatchEvent::ChannelPinsUpdate(e) => {
                            handle_event!(self, channel_pins_update, e);
                        }
                        // Guild
                        DispatchEvent::GuildCreate(e) => {
                            handle_event!(self, guild_create, e);
                        }
                        DispatchEvent::GuildUpdate(e) => {
                            handle_event!(self, guild_update, e);
                        }
                        DispatchEvent::GuildDelete(e) => {
                            handle_event!(self, guild_delete, e);
                        }
                        DispatchEvent::GuildBanAdd(e) => {
                            handle_event!(self, guild_ban_add, e);
                        }
                        DispatchEvent::GuildBanRemove(e) => {
                            handle_event!(self, guild_ban_remove, e);
                        }
                        DispatchEvent::GuildEmojisUpdate(e) => {
                            handle_event!(self, guild_emojis_update, e);
                        }
                        DispatchEvent::GuildIntegrationsUpdate(e) => {
                            handle_event!(self, guild_integrations_update, e);
                        }
                        DispatchEvent::GuildMemberAdd(e) => {
                            handle_event!(self, guild_member_add, e);
                        }
                        DispatchEvent::GuildMemberUpdate(e) => {
                            handle_event!(self, guild_member_update, e);
                        }
                        DispatchEvent::GuildMemberRemove(e) => {
                            handle_event!(self, guild_member_remove, e);
                        }
                        DispatchEvent::GuildMembersChunk(e) => {
                            handle_event!(self, guild_members_chunk, e);
                        }
                        DispatchEvent::GuildRoleCreate(e) => {
                            handle_event!(self, guild_role_create, e);
                        }
                        DispatchEvent::GuildRoleUpdate(e) => {
                            handle_event!(self, guild_role_update, e);
                        }
                        DispatchEvent::GuildRoleDelete(e) => {
                            handle_event!(self, guild_role_delete, e);
                        }
                        // Message
                        DispatchEvent::MessageCreate(e) => {
                            handle_event!(self, message_create, e);
                        }
                        DispatchEvent::MessageUpdate(e) => {
                            handle_event!(self, message_update, e);
                        }
                        DispatchEvent::MessageDelete(e) => {
                            handle_event!(self, message_delete, e);
                        }
                        DispatchEvent::MessageDeleteBulk(e) => {
                            handle_event!(self, message_delete_bulk, e);
                        }
                        DispatchEvent::MessageReactionAdd(e) => {
                            handle_event!(self, message_reaction_add, e);
                        }
                        DispatchEvent::MessageReactionRemove(e) => {
                            handle_event!(self, message_reaction_remove, e);
                        }
                        DispatchEvent::MessageReactionRemoveAll(e) => {
                            handle_event!(self, message_reaction_remove_all, e);
                        }
                        // Presences
                        DispatchEvent::PresenceUpdate(e) => {
                            handle_event!(self, presence_update, e);
                        }
                        DispatchEvent::TypingStart(e) => {
                            handle_event!(self, typing_start, e);
                        }
                        DispatchEvent::UserUpdate(e) => {
                            handle_event!(self, user_update, e);
                        }
                        _ => {}
                    },
                    Event::Reconnect => {
                        log::info!("Reconnected successfully!");
                    }
                    Event::InvalidSession(resumable) => {
                        self.session.set_resumable(resumable);
                    }
                    Event::HeartbeatACK => log::info!("HeartbeatACK received"),
                    Event::Close(e) => {
                        log::error!("Event CLOSE, reason: {}", e);

                        // Return if there are unrecoverable errors
                        match e {
                            DiscordError::AuthenticationFailed => return Err(e),
                            v => {
                                log::error!("{}", v);
                            } // TODO: Remove this
                        };

                        // If there was a recoverable error, try to reconnect
                        self.reconnect().await;
                    }
                    e => log::info!("Unhandled event received: {:?}", e),
                };
            }
        }

        //Ok(())
    }

    /// Makes all necessary to reconnect to gateway
    async fn reconnect(&mut self) {
        // Close channels
        if let Err(e) = self.gateway.close_channels() {
            log::error!(
                "Error when trying to close gateway channels at reconnect: {}",
                e
            );
        };

        // Reconnect and get last sequence received, needed to send a RESUME command
        let last_sequence = self.gateway.reconnect().await;

        // If session is resumable, send a RESUME command
        if self.session.is_resumable() {
            self.resume_connect(last_sequence).await;

        // Else send an IDENTIFY command, and start again
        } else {
            self.clean_connect().await;
            self.session.set_resumable(true);
        }
    }

    async fn clean_connect(&mut self) {
        // Create IDENTIFY
        let shard = [self.config.gateway_shard_id, self.config.gateway_num_shards];
        let identify = Command::new_identify(
            &self.token,
            self.config.gateway_large_treshold,
            self.config.gateway_guilds_subscriptions,
            shard,
        );

        // Send IDENTIFY, this should not fail
        self.gateway
            .to_gateway
            .send(identify)
            .await
            .expect("Could't send identify");

        // Spawn heartbeater
        self.spawn_heartbeater();
    }

    async fn resume_connect(&mut self, last_sequence: Option<u64>) {
        // Create a RESUME command
        let token = self.token.clone();
        let session_id = self.session.id().await;
        let resume = Command::new_resume(token, session_id, last_sequence);

        // Send RESUME, this should not fail
        self.gateway
            .to_gateway
            .send(resume)
            .await
            .expect("Couldn't send RESUME");

        // Spawn heartbeater
        self.spawn_heartbeater();
    }

    /// This function spawn a heartbeater that will be closed when
    /// the current gateway channel is close.
    fn spawn_heartbeater(&self) {
        let heartbeat_interval = self.gateway.heartbeat_interval;
        let to_gateway = self.gateway.to_gateway.clone();

        task::spawn(async move {
            heartbeat::heartbeater(heartbeat_interval, to_gateway).await;
            log::info!("spawn_heartbeater exited");
        });
    }

    // on_ready
    on_event_fn!(
        /// Set the handler function for [`Ready`] event
        ///
        /// [`Ready`]: ../models/gateway/events/struct.Ready.html
        on_ready,
        ready,
        Ready
    );

    // *******************************************************************************
    // * CHANNEL METHODS
    // *******************************************************************************

    // on_channel_create
    on_event_fn!(
        /// Set the handler function for [`ChannelCreate`] event
        ///
        /// [`ChannelCreate`]: ../models/gateway/events/struct.ChannelCreate.html
        on_channel_create,
        channel_create,
        ChannelCreate
    );

    // on_channel_update
    on_event_fn!(
        /// Set the handler function for [`ChannelUpdate`] event
        ///
        /// [`ChannelUpdate`]: ../models/gateway/events/struct.ChannelUpdate.html
        on_channel_update,
        channel_update,
        ChannelUpdate
    );

    // on_channel_delete
    on_event_fn!(
        /// Set the handler function for [`ChannelDelete`] event
        ///
        /// [`ChannelDelete`]: ../models/gateway/events/struct.ChannelDelete.html
        on_channel_delete,
        channel_delete,
        ChannelDelete
    );

    // on_channel_pins_update
    on_event_fn!(
        /// Set the handler function for [`ChannelPinsUpdate`] event
        ///
        /// [`ChannelPinsUpdate`]: ../models/gateway/events/struct.ChannelPinsUpdate.html
        on_channel_pins_update,
        channel_pins_update,
        ChannelPinsUpdate
    );

    // *******************************************************************************
    // * GUILD METHODS
    // *******************************************************************************

    // on_guild_create
    on_event_fn!(
        /// Set the handler function for [`GuildCreate`] event
        ///
        /// [`GuildCreate`]: ../models/gateway/events/struct.GuildCreate.html
        on_guild_create,
        guild_create,
        GuildCreate
    );

    // on_guild_update
    on_event_fn!(
        /// Set the handler function for [`GuildUpdate`] event
        ///
        /// [`GuildUpdate`]: ../models/gateway/events/struct.GuildUpdate.html
        on_guild_update,
        guild_update,
        GuildUpdate
    );

    // on_guild_delete
    on_event_fn!(
        /// Set the handler function for [`GuildDelete`] event
        ///
        /// [`GuildDelete`]: ../models/gateway/events/struct.GuildDelete.html
        on_guild_delete,
        guild_delete,
        GuildDelete
    );

    // on_guild_ban_add
    on_event_fn!(
        /// Set the handler function for [`GuildBanAdd`] event
        ///
        /// [`GuildBanAdd`]: ../models/gateway/events/struct.GuildBanAdd.html
        on_guild_ban_add,
        guild_ban_add,
        GuildBanAdd
    );

    // on_guild_ban_remove
    on_event_fn!(
        /// Set the handler function for [`GuildBanRemove`] event
        ///
        /// [`GuildBanRemove`]: ../models/gateway/events/struct.GuildBanRemove.html
        on_guild_ban_remove,
        guild_ban_remove,
        GuildBanRemove
    );

    // on_guild_emojis_update
    on_event_fn!(
        /// Set the handler function for [`GuildEmojisUpdate`] event
        ///
        /// [`GuildEmojisUpdate`]: ../models/gateway/events/struct.GuildEmojisUpdate.html
        on_guild_emojis_update,
        guild_emojis_update,
        GuildEmojisUpdate
    );

    // on_guild_integrations_update
    on_event_fn!(
        /// Set the handler function for [`GuildIntegrationsUpdate`] event
        ///
        /// [`GuildIntegrationsUpdate`]: ../models/gateway/events/struct.GuildIntegrationsUpdate.html
        on_guild_integrations_update,
        guild_integrations_update,
        GuildIntegrationsUpdate
    );

    // on_guild_member_add
    on_event_fn!(
        /// Set the handler function for [`GuildMemberAdd`] event
        ///
        /// [`GuildMemberAdd`]: ../models/gateway/events/struct.GuildMemberAdd.html
        on_guild_member_add,
        guild_member_add,
        GuildMemberAdd
    );

    // on_guild_member_update
    on_event_fn!(
        /// Set the handler function for [`GuildMemberUpdate`] event
        ///
        /// [`GuildMemberUpdate`]: ../models/gateway/events/struct.GuildMemberUpdate.html
        on_guild_member_update,
        guild_member_update,
        GuildMemberUpdate
    );

    // on_guild_member_remove
    on_event_fn!(
        /// Set the handler function for [`GuildMemberRemove`] event
        ///
        /// [`GuildMemberRemove`]: ../models/gateway/events/struct.GuildMemberRemove.html
        on_guild_member_remove,
        guild_member_remove,
        GuildMemberRemove
    );

    // on_guild_members_chunk
    on_event_fn!(
        /// Set the handler function for [`GuildMembersChunk`] event
        ///
        /// [`GuildMembersChunk`]: ../models/gateway/events/struct.GuildMembersChunk.html
        on_guild_members_chunk,
        guild_members_chunk,
        GuildMembersChunk
    );

    // on_guild_role_create
    on_event_fn!(
        /// Set the handler function for [`GuildRoleCreate`] event
        ///
        /// [`GuildRoleCreate`]: ../models/gateway/events/struct.GuildRoleCreate.html
        on_guild_role_create,
        guild_role_create,
        GuildRoleCreate
    );

    // on_guild_role_update
    on_event_fn!(
        /// Set the handler function for [`GuildRoleUpdate`] event
        ///
        /// [`GuildRoleUpdate`]: ../models/gateway/events/struct.GuildRoleUpdate.html
        on_guild_role_update,
        guild_role_update,
        GuildRoleUpdate
    );

    // on_guild_role_delete
    on_event_fn!(
        /// Set the handler function for [`GuildRoleDelete`] event
        ///
        /// [`GuildRoleDelete`]: ../models/gateway/events/struct.GuildRoleDelete.html
        on_guild_role_delete,
        guild_role_delete,
        GuildRoleDelete
    );

    // *******************************************************************************
    // * MESSAGE METHODS
    // *******************************************************************************

    // on_message_create
    on_event_fn!(
        /// Set the handler function for [`MessageCreate`] event
        ///
        /// [`MessageCreate`]: ../models/gateway/events/struct.MessageCreate.html
        on_message_create,
        message_create,
        MessageCreate
    );

    // on_message_update
    on_event_fn!(
        /// Set the handler function for [`MessageUpdate`] event
        ///
        /// [`MessageUpdate`]: ../models/gateway/events/struct.MessageUpdate.html
        on_message_update,
        message_update,
        MessageUpdate
    );

    // on_message_delete
    on_event_fn!(
        /// Set the handler function for [`MessageDelete`] event
        ///
        /// [`MessageDelete`]: ../models/gateway/events/struct.MessageDelete.html
        on_message_delete,
        message_delete,
        MessageDelete
    );

    // on_message_delete_bulk
    on_event_fn!(
        /// Set the handler function for [`MessageDeleteBulk`] event
        ///
        /// [`MessageDeleteBulk`]: ../models/gateway/events/struct.MessageDeleteBulk.html
        on_message_delete_bulk,
        message_delete_bulk,
        MessageDeleteBulk
    );

    // on_message_reaction_add
    on_event_fn!(
        /// Set the handler function for [`MessageReactionAdd`] event
        ///
        /// [`MessageReactionAdd`]: ../models/gateway/events/struct.MessageReactionAdd.html
        on_message_reaction_add,
        message_reaction_add,
        MessageReactionAdd
    );

    // on_message_reaction_remove
    on_event_fn!(
        /// Set the handler function for [`MessageReactionRemove`] event
        ///
        /// [`MessageReactionRemove`]: ../models/gateway/events/struct.MessageReactionRemove.html
        on_message_reaction_remove,
        message_reaction_remove,
        MessageReactionRemove
    );

    // on_message_reaction_remove_all
    on_event_fn!(
        /// Set the handler function for [`MessageReactionRemoveAll`] event
        ///
        /// [`MessageReactionRemoveAll`]: ../models/gateway/events/struct.MessageReactionRemoveAll.html
        on_message_reaction_remove_all,
        message_reaction_remove_all,
        MessageReactionRemoveAll
    );

    // *******************************************************************************
    // * PRESENCE METHODS
    // *******************************************************************************

    // on_presence_update
    on_event_fn!(
        /// Set the handler function for [`PresenceUpdate`] event
        ///
        /// [`PresenceUpdate`]: ../models/gateway/events/struct.PresenceUpdate.html
        on_presence_update,
        presence_update,
        PresenceUpdate
    );

    // on_typing_start
    on_event_fn!(
        /// Set the handler function for [`TypingStart`] event
        ///
        /// [`TypingStart`]: ../models/gateway/events/struct.TypingStart.html
        on_typing_start,
        typing_start,
        TypingStart
    );

    // on_user_update
    on_event_fn!(
        /// Set the handler function for [`UserUpdate`] event
        ///
        /// [`UserUpdate`]: ../models/gateway/events/struct.UserUpdate.html
        on_user_update,
        user_update,
        UserUpdate
    );
}
