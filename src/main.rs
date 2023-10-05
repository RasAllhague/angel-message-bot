use std::{
    collections::HashMap,
    path::PathBuf,
    sync::Arc,
};

use commands::{config::ConfigCommand, CommandError, SlashCommand};
use config::{AppConfig, EnvironmentConfigurations};
use handler::BotHandler;
use serenity::{model::prelude::UserId, prelude::*};
use tracing::{instrument, log::error};

mod commands;
mod config;
mod handler;
mod message_storage;

#[tokio::main]
#[instrument]
async fn main() -> Result<(), CommandError> {
    tracing_subscriber::fmt::init();

    let env_config = EnvironmentConfigurations::from_env();

    if !env_config.config_path.exists() {
        let app_config = AppConfig {
            deleted_message_send_channels: HashMap::new(),
            observed_user_id: UserId(714599597829390459),
            message_storage_path: PathBuf::from("./message_storage.json"),
        };

        app_config.save(&env_config.config_path).await?;
    }

    let app_config = AppConfig::load(&env_config.config_path).await?;

    let mut commands: Vec<Arc<dyn SlashCommand>> = Vec::new();
    commands.push(Arc::new(ConfigCommand));

    let intents = GatewayIntents::default()
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGES;
    let mut client = Client::builder(env_config.bot_token, intents)
        .event_handler(BotHandler::new(
            &commands,
            app_config,
            &env_config.config_path,
        ))
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }

    Ok(())
}
