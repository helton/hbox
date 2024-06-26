use crate::configs::app::AppConfig;
use crate::serialization::parse_json;
use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

pub struct IndexConfig {}

impl IndexConfig {
    pub fn load(name: String) -> Result<Package, Box<dyn Error>> {
        let config = AppConfig::load();
        let index_path = config.index_path().to_owned();
        let package_dir = Path::new(&index_path);
        let overrides_path = config.overrides_path().to_owned();
        let overrides_dir = Path::new(&overrides_path);
        let package = Self::load_package(&name, package_dir, overrides_dir)?;

        Ok(package)
    }

    fn load_package(
        name: &str,
        index_path: &Path,
        overrides_path: &Path,
    ) -> Result<Package, Box<dyn Error>> {
        let shard_dir = name.chars().next().unwrap().to_string().to_lowercase();
        let index_file = index_path
            .join(&Path::new(&shard_dir))
            .join(format!("{}.json", name));
        let override_file = overrides_path.join(format!("{}.json", name));

        if override_file.exists() {
            debug!("Using override config at {:?}", &override_file);
            Ok(parse_json(&override_file)?)
        } else if index_file.exists() {
            debug!("Using index config at {:?}", &index_file);
            Ok(parse_json(&index_file)?)
        } else {
            debug!("No config file found for {}, using default values", &name);
            Ok(Package::new(name))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Package {
    pub image: Image,
    pub volumes: Option<Vec<Volume>>,
    pub ports: Option<Vec<Port>>,
    pub current_directory: Option<String>,
    pub environment_variables: Option<Vec<EnvironmentVariable>>,
    pub binaries: Option<Vec<Binary>>,
    #[serde(default)]
    pub only_shim_binaries: bool,
}

impl Package {
    pub fn new(name: &str) -> Self {
        Package {
            image: Image {
                name: format!("docker.io/{}", name),
                build: None,
            },
            volumes: None,
            ports: None,
            current_directory: None,
            environment_variables: None,
            binaries: None,
            only_shim_binaries: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {
    pub name: String,
    pub build: Option<Build>,
}

impl Image {
    pub fn is_local(&self) -> bool {
        self.build.is_some()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Build {
    pub context: String,
    pub dockerfile: String,
    pub args: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Volume {
    pub source: String,
    pub target: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Port {
    pub host: u16,
    pub container: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Binary {
    pub name: String,
    pub path: String,
    pub cmd: Option<Vec<String>>,
    #[serde(default)]
    pub wrap_args: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnvironmentVariable {
    pub name: String,
    pub value: String,
}
