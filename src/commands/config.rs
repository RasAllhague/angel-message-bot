use async_trait::async_trait;
use serenity::{
    builder::{CreateApplicationCommand, CreateApplicationCommands},
    model::prelude::{
        command::CommandOptionType,
        interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
    },
    prelude::*,
};
use tracing::info;

use crate::{commands::parser::OptionParser, config::AppConfig, handler::Configuration};

use super::{CommandError, SlashCommand};

static COMMAND_NAME: &str = "config";

pub struct ConfigCommand;

#[async_trait]
impl SlashCommand for ConfigCommand {
    fn register<'a>(
        &'a self,
        commands: &'a mut CreateApplicationCommands,
    ) -> &mut CreateApplicationCommands {
        commands.create_application_command(|command| Self::build(command));

        commands
    }

    async fn dispatch(
        &self,
        command: &ApplicationCommandInteraction,
        ctx: &Context,
        configuration: &Configuration,
    ) -> Result<(), CommandError> {
        command
            .create_interaction_response(ctx, |m| {
                m.kind(InteractionResponseType::DeferredChannelMessageWithSource)
            })
            .await?;

        let channel_id = OptionParser::parse_channel_id(&command.data.options, 0)?;

        let mut channels = configuration.send_channels.clone();

        let guild_id = match command.guild_id {
            Some(g) => g,
            None => return Err(CommandError::NoGuildId),
        };

        if let Some(old_channel_id) = channels.insert(guild_id, channel_id) {
            info!("Changing channel id from '{old_channel_id}' to '{channel_id}' for guild '{guild_id}'.");
        }

        let app_config = AppConfig {
            deleted_message_send_channels: channels,
            observed_user_id: configuration.observed_users,
            message_storage_path: configuration.message_storage_path.clone(),
        };

        info!("Updating config for guild: {guild_id}");
        app_config.save(&configuration.file_path).await?;
        info!("Updated config for guild: {guild_id}");

        command
            .edit_original_interaction_response(ctx, |m| m.content("Updated!"))
            .await?;

        Ok(())
    }

    fn name(&self) -> String {
        String::from(COMMAND_NAME)
    }
}

impl ConfigCommand {
    fn build(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name(COMMAND_NAME)
            .description("Command for setting configurations.")
            .create_option(|sub_command| {
                sub_command
                    .name("target-channel")
                    .description("Channel where to send his deleted messages.")
                    .kind(CommandOptionType::Channel)
                    .required(true)
            })
    }
}
