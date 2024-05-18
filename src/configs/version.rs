use crate::configs::app::AppConfig;
use crate::serialization::{parse_json, save_json};
use log::debug;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::remove_file;

pub struct VersionConfig {}

impl VersionConfig {
    pub fn load(name: String) -> Result<Option<Package>, Box<dyn Error>> {
        let config = AppConfig::load();
        let version_file = config.versions_path().join(format!("{}.json", name));
        if version_file.exists() {
            Ok(parse_json(&version_file)?)
        } else {
            Ok(None)
        }
    }

    pub fn upsert(name: &str, package: crate::packages::Package) -> Result<(), Box<dyn Error>> {
        debug!("Adding/Updating package '{}'", name);
        let config = AppConfig::load();
        let version_file = config.versions_path().join(format!("{}.json", name));
        debug!("Saving file {:?}", &version_file);
        save_json(&package.versions, &version_file)?;
        Ok(())
    }

    pub fn remove(name: &str) -> Result<(), Box<dyn Error>> {
        debug!("Removing package: '{}'", name);
        let config = AppConfig::load();
        let version_file = config.versions_path().join(format!("{}.json", name));
        if version_file.exists() {
            debug!("Removing file {:?}", &version_file);
            remove_file(&version_file)?
        }
        Ok(())
    }
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
