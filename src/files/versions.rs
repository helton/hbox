use crate::files::variables::AppConfig;
use crate::serialization::{parse_json, save_json};
use log::{debug};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Root {
    pub packages: std::collections::HashMap<String, Package>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Package {
    pub versions: Vec<String>,
    pub current: String,
}

impl Package {
    pub fn new(version: &str) -> Self {
        Package {
            versions: vec![String::from(version)],
            current: String::from(version),
        }
    }
}

pub fn parse() -> Result<Root, Box<dyn Error>> {
    let config = AppConfig::new();
    parse_json(&config.versions_path())
}

pub fn upsert(name: &str, package: crate::packages::Package) -> Result<(), Box<dyn Error>> {
    debug!("Adding/Updating package '{}'", name);
    let config = AppConfig::new();
    let mut root = parse()?;
    root.packages.insert(String::from(name), package.versions);
    save_json(&root, &config.versions_path())?;
    Ok(())
}

pub fn remove(name: &str) -> Result<(), Box<dyn Error>> {
    debug!("Removing package: '{}'", name);
    let config = AppConfig::new();
    let mut root = parse()?;
    root.packages.remove(name);
    save_json(&root, &config.versions_path())?;
    Ok(())
}
