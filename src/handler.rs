use std::{sync::Arc, collections::HashMap};

use serenity::{
    async_trait,
    model::prelude::{
        command::Command,
        interaction::{
            Interaction,
        }, Ready, ResumedEvent, GuildId, UserId, ChannelId, MessageId,
    },
    prelude::{Context, EventHandler},
};
use tracing::{
    info, instrument,
    log::{debug, error},
};

use crate::{
    commands::{SlashCommand}, config::AppConfig,
};

pub struct BotHandler {
    pub commands: Vec<Arc<dyn SlashCommand>>,
    pub app_config: AppConfig,
    pub settings_file_path: String,
}

pub struct Configuration {
    pub observed_users: UserId,
    pub send_channels: HashMap<GuildId, ChannelId>,
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
                file_path: self.settings_file_path.clone(),
            };

            for command in self
                .commands
                .iter()
                .filter(|x| x.name() == command_interaction.data.name)
            {
                if let Err(why) = command
                    .dispatch(&command_interaction, &ctx, &conf)
                    .await
                {
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

    async fn message_delete(
        &self,
        ctx: Context,
        channel_id: ChannelId,
        deleted_message_id: MessageId,
        guild_id: Option<GuildId>,
    ) {
        if let Some(guild_id) = guild_id {
            let message = match channel_id.message(&ctx, deleted_message_id).await {
                Ok(message) => message,
                Err(why) => {
                    error!("Error while fetching message: {}", why);
                    
                    return;
                }
            };

            if let Some(target_channel) = self.app_config.deleted_message_send_channels.get(&guild_id) {
                if let Err(why) = target_channel
                    .send_message(&ctx, |create_message| create_message.content(message.content)).await {
                    error!("Failed to send deleted message to channel, '{}': {}", target_channel.0, why);
                }
            }
        }
    }
}