use serenity::model::prelude::{
    interaction::application_command::{CommandDataOption, CommandDataOptionValue},
    ChannelId,
};

#[derive(Debug)]
pub enum ParserError {
    ChannelId(String),
}

pub struct OptionParser;

impl OptionParser {
    pub fn parse_channel_id(
        options: &[CommandDataOption],
        index: usize,
    ) -> Result<ChannelId, ParserError> {
        if let Some(option) = options.get(index) {
            if let Some(value) = option.resolved.as_ref() {
                if let CommandDataOptionValue::Channel(data) = value {
                    return Ok(data.id);
                }
            }
        }

        Err(ParserError::ChannelId(format!(
            "No ChannelId option was found at index {}!",
            index
        )))
    }
}
