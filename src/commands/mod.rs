mod parser;

use async_trait::async_trait;
use serenity::{
    builder::CreateApplicationCommands,
    model::prelude::interaction::application_command::ApplicationCommandInteraction,
    prelude::Context,
};

use crate::handler::Configuration;

use self::parser::ParserError;

#[async_trait]
pub trait SlashCommand: Send + Sync {
    fn register<'a>(
        &'a self,
        commands: &'a mut CreateApplicationCommands,
    ) -> &mut CreateApplicationCommands;
    async fn dispatch(
        &self,
        command: &ApplicationCommandInteraction,
        ctx: &Context,
        configuration: &Configuration,
    ) -> Result<(), CommandError>;
    fn name(&self) -> String;
}

#[derive(Debug)]
pub enum CommandError {
    Parser(ParserError),
    Serenity(serenity::Error),
    IO(std::io::Error),
}

impl From<ParserError> for CommandError {
    fn from(value: ParserError) -> Self {
        CommandError::Parser(value)
    }
}

impl From<serenity::Error> for CommandError {
    fn from(value: serenity::Error) -> Self {
        CommandError::Serenity(value)
    }
}

impl From<std::io::Error> for CommandError {
    fn from(value: std::io::Error) -> Self {
        CommandError::IO(value)
    }
}
