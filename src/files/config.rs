use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::files::variables::AppConfig;
use crate::serialization::parse_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Root {
    pub log_level: String,
}

pub fn get_config() -> Result<Root, Box<dyn Error>> {
    let config = AppConfig::new();
    parse_json(&config.config_path())
}
