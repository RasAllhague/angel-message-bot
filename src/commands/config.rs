use async_trait::async_trait;
use serenity::{prelude::*, model::prelude::{interaction::{application_command::ApplicationCommandInteraction, InteractionResponseType}, command::CommandOptionType}, builder::{CreateApplicationCommands, CreateApplicationCommand}};
use tracing::info;

use crate::{handler::Configuration, commands::parser::OptionParser, config::AppConfig};

use super::{SlashCommand, CommandError};

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
    
        if let None = command.guild_id {
            return Ok(());
        }

        channels.insert(command.guild_id.unwrap(), channel_id);

        let app_config = AppConfig {
            deleted_message_send_channels: channels,
            observed_user_id: configuration.observed_users,
        };

        info!("Updating config for guild: {:?}", command.guild_id);
        app_config.save(&configuration.file_path).await?;
        info!("Updated config for guild: {:?}", command.guild_id);

        command
            .edit_original_interaction_response(ctx, |m| {
                m.content("Updated!")
            })
            .await?;

        Ok(())
    }

    fn name(&self) -> String {
        todo!()
    }
}

impl ConfigCommand {
    fn build(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name(COMMAND_NAME)
            .description("Command for nuking an entire channel with a timeout nuke.")
            .create_option(|sub_command| {
                sub_command
                    .name("minutes-in-past")
                    .description("Timeframe of messages.")
                    .kind(CommandOptionType::Channel)
                    .required(true)
            })
    }
}