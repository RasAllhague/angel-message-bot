use std::{collections::HashMap, path::PathBuf, sync::Arc};

use chrono::{Days, Utc};
use serenity::{
    async_trait,
    model::prelude::{
        command::Command, interaction::Interaction, ChannelId, GuildId, Message, MessageId, Ready,
        ResumedEvent, UserId,
    },
    prelude::{Context, EventHandler},
};
use tracing::{
    info, instrument,
    log::{debug, error, warn},
};

use crate::{commands::SlashCommand, config::AppConfig, message_storage::MessageStorage};

pub struct BotHandler {
    pub commands: Vec<Arc<dyn SlashCommand>>,
    pub app_config: AppConfig,
    pub settings_file_path: String,
}

pub struct Configuration {
    pub observed_users: UserId,
    pub send_channels: HashMap<GuildId, ChannelId>,
    pub message_storage_path: PathBuf,
    pub file_path: String,
}

impl BotHandler {
    pub fn new(
        commands: &[Arc<dyn SlashCommand>],
        app_config: AppConfig,
        file_path: &str,
    ) -> BotHandler {
        BotHandler {
            commands: commands.into(),
            app_config: app_config,
            settings_file_path: String::from(file_path),
        }
    }
}

#[async_trait]
impl EventHandler for BotHandler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command_interaction) = interaction {
            debug!("Received command interaction: {:#?}", command_interaction);

            if command_interaction.guild_id.is_none() {
                return;
            }

            let conf = Configuration {
                observed_users: self.app_config.observed_user_id.clone(),
                send_channels: self.app_config.deleted_message_send_channels.clone(),
                message_storage_path: self.app_config.message_storage_path.clone(),
                file_path: self.settings_file_path.clone(),
            };

            for command in self
                .commands
                .iter()
                .filter(|x| x.name() == command_interaction.data.name)
            {
                if let Err(why) = command.dispatch(&command_interaction, &ctx, &conf).await {
                    error!("Error during command interaction: {:?}", why);
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let guild_commands =
            Command::set_global_application_commands(&ctx.http, |command_builder| {
                for command in &self.commands {
                    command.register(command_builder);
                }

                command_builder
            })
            .await;

        if let Err(why) = guild_commands {
            error!("Failed to create slash commands. {}", why);
        }
    }

    #[instrument(skip(self, _ctx))]
    async fn resume(&self, _ctx: Context, resume: ResumedEvent) {
        debug!("Resumed; trace: {:?}", resume.trace);
    }

    async fn message(&self, _: Context, new_message: Message) {
        if new_message.author.id != self.app_config.observed_user_id {
            return;
        }

        let mut message_storage =
            match MessageStorage::load(&self.app_config.message_storage_path).await {
                Ok(storage) => storage,
                Err(why) => {
                    error!("Failed to load message storage: {:?}", why);
                    return;
                }
            };

        message_storage.messages.push((Utc::now(), new_message));

        for index in 0..message_storage.messages.len() {
            let (created, _) = message_storage.messages[index].clone();

            let now = Utc::now();

            if let Some(subdate) = now.checked_sub_days(Days::new(2)) {
                if created < subdate {
                    message_storage.messages.remove(index);
                    info!("C:{created}/S:{subdate}");
                }
            } else {
                message_storage.messages.remove(index);
                info!("Fall2");
            }
        }

        if let Err(why) = message_storage
            .save(&self.app_config.message_storage_path)
            .await
        {
            error!("Failed to save message storage: {:?}", why);
        };
    }

    async fn message_delete(
        &self,
        ctx: Context,
        _: ChannelId,
        deleted_message_id: MessageId,
        guild_id: Option<GuildId>,
    ) {
        if let Some(guild_id) = guild_id {
            let message_storage =
                match MessageStorage::load(&self.app_config.message_storage_path).await {
                    Ok(storage) => storage,
                    Err(why) => {
                        error!("Failed to load message storage: {:?}", why);
                        return;
                    }
                };

            let message = match message_storage
                .messages
                .iter()
                .filter(|(_, m)| m.id == deleted_message_id)
                .next()
            {
                Some(m) => m,
                None => {
                    warn!("Failed to get message out of storage. Message not found.");
                    return;
                }
            };

            if let Some(target_channel) =
                self.app_config.deleted_message_send_channels.get(&guild_id)
            {
                if let Err(why) = target_channel
                    .send_message(&ctx, |create_message| {
                        create_message.content(&message.1.content)
                    })
                    .await
                {
                    error!(
                        "Failed to send deleted message to channel, '{}': {}",
                        target_channel.0, why
                    );
                }
            }
        }
    }
}
