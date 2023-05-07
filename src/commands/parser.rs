use chrono::{NaiveDate, NaiveDateTime};
use serenity::model::{
    prelude::{
        interaction::application_command::{CommandDataOption, CommandDataOptionValue},
        ChannelId,
    },
    user::User,
};

#[derive(Debug)]
pub enum ParserError {
    Date,
    User(String),
    ChannelId(String),
    Text(String),
}
pub struct UserInputParser;

impl UserInputParser {
    pub fn parse(options: &[CommandDataOption], index: usize) -> Result<User, ParserError> {
        if let Some(option) = options.get(index) {
            if let Some(value) = option.resolved.as_ref() {
                if let CommandDataOptionValue::User(data, _) = value {
                    return Ok(data.clone());
                }

                return Err(ParserError::User(String::from("No value found!")));
            }

            return Err(ParserError::User(String::from("No option found!")));
        }

        return Err(ParserError::User(format!(
            "No option found at index {}!",
            index
        )));
    }
}

pub struct DateInputParser;

impl DateInputParser {
    pub fn parse(options: &[CommandDataOption]) -> Result<NaiveDateTime, ParserError> {
        let date_parts: Result<Vec<i64>, String> =
            (0..3).map(|x| Self::get_int_option(options, x)).collect();

        let date_parts = date_parts.expect("User input expected.");

        if let Some(date) = NaiveDate::from_ymd_opt(
            date_parts[2] as i32,
            date_parts[1] as u32,
            date_parts[0] as u32,
        ) {
            if let Some(date) = date.and_hms_opt(0, 0, 0) {
                return Ok(date);
            }
        }

        Err(ParserError::Date)
    }

    fn get_int_option(options: &[CommandDataOption], index: usize) -> Result<i64, String> {
        if let Some(option) = options.get(index) {
            if let Some(value) = option.resolved.as_ref() {
                if let CommandDataOptionValue::Integer(data) = value {
                    return Ok(*data);
                }
            }
        }

        Err(format!("Option {} not found!", index))
    }
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

    pub fn parse_string(
        options: &[CommandDataOption],
        index: usize,
    ) -> Result<String, ParserError> {
        if let Some(option) = options.get(index) {
            if let Some(value) = option.resolved.as_ref() {
                if let CommandDataOptionValue::String(data) = value {
                    return Ok(data.clone());
                }
            }
        }

        Err(ParserError::Text(format!(
            "No Text option was found at index {}!",
            index
        )))
    }
}
