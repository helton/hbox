use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::files::variables::AppConfig;
use crate::serialization::{parse_json, save_json};

#[derive(Serialize, Deserialize, Debug)]
pub struct Root {
    pub logs: Logs,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Logs {
    pub level: String,
    pub strategy: String,
}

pub fn get_config() -> Result<Root, Box<dyn Error>> {
    let config = AppConfig::new();
    if config.config_path().exists() {
        parse_json(&config.config_path())
    } else {
        let root = Root {
            logs: Logs {
                level: "info".to_owned(),
                strategy: "append".to_owned(),
            },
        };
        save_json(&root, &config.config_path())?;
        Ok(root)
    }
}
