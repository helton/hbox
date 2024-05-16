use crate::serialization::parse_json;
use crate::variables::AppConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Root {
    pub packages: HashMap<String, Package>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Package {
    pub image: String,
    pub volumes: Option<Vec<Volume>>,
    pub current_directory: Option<String>,
}

impl Package {
    pub fn new(name: &str) -> Self {
        Package {
            image: format!("docker.io/{}", name),
            volumes: None,
            current_directory: None
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Volume {
    pub source: String,
    pub target: String,
}

pub fn parse() -> Result<Root, Box<dyn Error>> {
    let config = AppConfig::new();
    parse_json(&config.index_path())
}
