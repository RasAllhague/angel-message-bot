use std::sync::Arc;

use commands::SlashCommand;
use config::AppConfigurations;
use handler::BotHandler;
use serenity::prelude::*;
use tracing::{instrument, log::error};

mod commands;
mod utils;
mod config;
mod handler;

#[tokio::main]
#[instrument]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = AppConfigurations::from_env();
    let mut commands: Vec<Arc<dyn SlashCommand>> = Vec::new();

    let intents = GatewayIntents::default() | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MESSAGES;
    let mut client = Client::builder(config.bot_token, intents)
        .event_handler(BotHandler::new(&commands))
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
