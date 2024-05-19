use log::LevelFilter;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::str::FromStr;

use crate::configs::app::AppConfig;
use crate::serialization::{parse_json, save_json};

pub struct UserConfig {}

impl UserConfig {
    pub fn load() -> Result<Root, Box<dyn Error>> {
        let config = AppConfig::load();
        if config.config_file_path().exists() {
            parse_json(&config.config_file_path())
        } else {
            let root = Root::default();
            save_json(&root, &config.config_file_path())?;
            Ok(root)
        }
    }

    pub fn save(root: Root) -> Result<(), Box<dyn Error>> {
        let config = AppConfig::load();
        save_json(&root, &config.config_file_path())?;
        Ok(())
    }

    pub fn write_config_value(path: &str, value: &str) -> Result<(), Box<dyn Error>> {
        let mut root: Value = serde_json::to_value(Self::load()?)?;
        let (current, last_part) = Self::traverse_path(&mut root, path)?;

        let parsed_value =
            if value.eq_ignore_ascii_case("true") || value.eq_ignore_ascii_case("false") {
                Value::Bool(value.parse::<bool>()?)
            } else if let Ok(int_value) = value.parse::<i64>() {
                Value::Number(int_value.into())
            } else {
                Value::String(value.to_string())
            };

        match current {
            Value::Object(map) => {
                if map.contains_key(&last_part) {
                    map.insert(last_part, parsed_value);
                } else {
                    return Err(format!("Invalid configuration key: {}", path).into());
                }
            }
            _ => return Err("Invalid configuration path".into()),
        }

        let updated_root: Root = serde_json::from_value(root)?;
        Self::save(updated_root)?;

        Ok(())
    }

    pub fn read_config_value(path: &str) -> Result<(), Box<dyn Error>> {
        let mut root: Value = serde_json::to_value(Self::load()?)?;
        let (current, last_part) = Self::traverse_path(&mut root, path)?;

        match current.get(&last_part) {
            Some(value) => {
                if value.is_object() {
                    println!("{}", serde_json::to_string_pretty(&value)?);
                } else {
                    println!("{}", value);
                }
            }
            None => return Err(format!("Invalid configuration key: {}", path).into()),
        }

        Ok(())
    }

    fn traverse_path<'a>(
        root: &'a mut Value,
        path: &str,
    ) -> Result<(&'a mut Value, String), Box<dyn Error>> {
        let parts: Vec<&str> = path.split('.').collect();
        if parts.is_empty() {
            return Err("Invalid configuration path".into());
        }

        let mut current = root;
        for part in parts.iter().take(parts.len() - 1) {
            current = current.get_mut(*part).ok_or("Invalid configuration path")?;
        }

        let last_part = parts
            .last()
            .ok_or("Invalid configuration path")?
            .to_string();
        Ok((current, last_part))
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Root {
    pub engine: Engine,
    pub logs: Logs,
    pub experimental: Experimental,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Engine {
    Docker,
    Podman,
}

impl Default for Engine {
    fn default() -> Self {
        Self::Docker
    }
}

impl Engine {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Docker => "docker",
            Self::Podman => "podman",
        }
    }
}

impl FromStr for Engine {
    type Err = ();

    fn from_str(input: &str) -> Result<Engine, Self::Err> {
        match input.to_lowercase().as_str() {
            "docker" => Ok(Engine::Docker),
            "podman" => Ok(Engine::Podman),
            _ => Err(()),
        }
    }
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
