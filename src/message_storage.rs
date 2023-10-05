use std::path::Path;

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use serenity::model::prelude::Message;
use tokio::{fs::{self, File}, io::AsyncWriteExt};

use crate::commands::CommandError;

#[derive(Serialize, Deserialize)]
pub struct MessageStorage {
    pub messages: Vec<(DateTime<Utc>, Message)>
}

impl MessageStorage {
    pub async fn load(file_path: &Path) -> Result<Self, CommandError> {
        if !file_path.exists() {
            let storage = MessageStorage {
                messages: Vec::new(),
            };

            storage.save(file_path).await?;
        }

        let contents = fs::read_to_string(file_path).await?;

        let file: Self = serde_json::from_str(&contents)?;

        Ok(file)
    }

    pub async fn save(&self, file_path: &Path) -> Result<(), CommandError> {
        let serialized = serde_json::to_string(&self)?;

        let mut file = File::create(file_path).await?;
        file.write_all(serialized.as_bytes()).await?;

        Ok(())
    }
}
