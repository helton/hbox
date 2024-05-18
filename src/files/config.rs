use log::LevelFilter;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::str::FromStr;

use crate::files::variables::AppConfig;
use crate::serialization::{parse_json, save_json};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Root {
    pub logs: Logs,
    pub experimental: Experimental,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Logs {
    #[serde(default)]
    pub enabled: bool,
    pub level: Level,
    pub strategy: Strategy,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Strategy {
    Truncate,
    Append,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Experimental {
    #[serde(default)]
    pub capture_stdout: bool,
    #[serde(default)]
    pub capture_stderr: bool,
}

impl Default for Logs {
    fn default() -> Self {
        Self {
            enabled: false,
            level: Level::Info,
            strategy: Strategy::Append,
        }
    }
}

impl Level {
    pub fn to_level_filter(&self) -> LevelFilter {
        match self {
            Level::Info => LevelFilter::Info,
            Level::Warn => LevelFilter::Warn,
            Level::Debug => LevelFilter::Debug,
            Level::Trace => LevelFilter::Trace,
            _ => LevelFilter::Info,
        }
    }
}

impl FromStr for Level {
    type Err = ();

    fn from_str(input: &str) -> Result<Level, Self::Err> {
        match input {
            "trace" => Ok(Level::Trace),
            "debug" => Ok(Level::Debug),
            "info" => Ok(Level::Info),
            "warn" => Ok(Level::Warn),
            "error" => Ok(Level::Error),
            _ => Err(()),
        }
    }
}

impl FromStr for Strategy {
    type Err = ();

    fn from_str(input: &str) -> Result<Strategy, Self::Err> {
        match input {
            "truncate" => Ok(Strategy::Truncate),
            "append" => Ok(Strategy::Append),
            _ => Err(()),
        }
    }
}

pub fn get_config() -> Result<Root, Box<dyn Error>> {
    let config = AppConfig::new();
    if config.config_path().exists() {
        parse_json(&config.config_path())
    } else {
        let root = Root::default();
        save_json(&root, &config.config_path())?;
        Ok(root)
    }
}
