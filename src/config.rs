use std::env;

pub struct AppConfigurations {
    pub bot_token: String,
    pub database_url: String,
    pub lls_file_path: String,
}

impl AppConfigurations {
    pub fn from_env() -> AppConfigurations {
        let token = env::var("WALTER_BOT_TOKEN").expect("Expected a token in the environment!");
        let db_url =
            env::var("WALTER_DATABASE_URL").expect("Expected database url in the environment!");
        let lls_file_path =
            env::var("WALTER_LLS_PATH").expect("Expected walter lls path in the environment!");

        AppConfigurations {
            bot_token: token,
            database_url: db_url,
            lls_file_path: lls_file_path,
        }
    }
}
