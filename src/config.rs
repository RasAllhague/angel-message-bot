use std::{env, collections::HashMap, fs};

use serde::{Deserialize, Serialize};
use serenity::model::prelude::{ChannelId, GuildId, UserId};
use tokio::{fs::File, io::AsyncWriteExt};

use crate::commands::CommandError;

pub struct EnvironmentConfigurations {
    pub bot_token: String,
    pub config_path: String,
}

impl EnvironmentConfigurations {
    pub fn from_env() -> EnvironmentConfigurations {
        let token = env::var("ANGEL_BOT_TOKEN").expect("Expected bot token in the environment!");
        let config_path =
            env::var("ANGEL_BOT_CONFIGFILE").expect("Expected angel bot config in the environment!");

        EnvironmentConfigurations {
            bot_token: token,
            config_path,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub observed_user_id: UserId,
    pub deleted_message_send_channels: HashMap<GuildId, ChannelId>,
}

impl AppConfig {
    pub async fn load(file_path: &str) -> Result<Self, CommandError> {
        let contents = fs::read_to_string(file_path)?;

        let file: Self = serde_json::from_str(&contents)?;

        Ok(file)
    }
    
    pub async fn save(&self, file_path: &str) -> Result<(), CommandError> {
        let serialized = serde_json::to_string(&self)?;

        let mut file = File::create(file_path).await?;
        file.write_all(serialized.as_bytes()).await?;

        Ok(())
    }
}