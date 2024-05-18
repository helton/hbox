use crate::files::variables::AppConfig;
use crate::serialization::parse_json;
use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Root {
    pub packages: HashMap<String, Package>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Package {
    pub image: String,
    pub volumes: Option<Vec<Volume>>,
    pub current_directory: Option<String>,
    pub environment_variables: Option<Vec<EnvironmentVariable>>,
    pub binaries: Option<Vec<Binary>>,
    #[serde(default)]
    pub only_shim_binaries: bool,
}

impl Package {
    pub fn new(name: &str) -> Self {
        Package {
            image: format!("docker.io/{}", name),
            volumes: None,
            current_directory: None,
            environment_variables: None,
            binaries: None,
            only_shim_binaries: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Volume {
    pub source: String,
    pub target: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Binary {
    pub name: String,
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnvironmentVariable {
    pub name: String,
    pub value: String,
}

fn load_package(
    name: &str,
    index_path: &Path,
    overrides_path: &Path,
) -> Result<Package, Box<dyn Error>> {
    let index_file = index_path.join(format!("{}.json", name));
    let override_file = overrides_path.join(format!("{}.json", name));

    if override_file.exists() {
        debug!("Using override config at {:?}", &override_file);
        parse_json(&override_file)
    } else {
        debug!("Using normal config at {:?}", &index_file);
        parse_json(&index_file)
    }
}

pub fn parse(name: String) -> Result<Root, Box<dyn Error>> {
    let config = AppConfig::new();
    let index_path = config.index_path().to_owned();
    let package_dir = Path::new(&index_path);
    let overrides_path = config.overrides_path().to_owned();
    let overrides_dir = Path::new(&overrides_path);
    let mut packages = HashMap::new();

    let package = load_package(&name, package_dir, overrides_dir)?;
    packages.insert(name, package);

    Ok(Root { packages })
}
