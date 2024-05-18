use crate::files::index::Package as IndexPackage;
use crate::files::variables::AppConfig;
use crate::files::versions::Package as VersionsPackage;
use log::{debug, info};
use std::error::Error;
use std::fs;

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub index: IndexPackage,
    pub versions: VersionsPackage,
}

// Public API
impl Package {
    pub fn new(name: &str, versions_package: VersionsPackage) -> Result<Self, Box<dyn Error>> {
        let index_package = crate::files::index::parse(name.to_owned())?;
        Ok(Self {
            name: String::from(name),
            index: index_package,
            versions: versions_package,
        })
    }

    pub fn load(name: &str) -> Result<Option<Self>, Box<dyn Error>> {
        if let Some(versions_package) = crate::files::versions::parse(name.to_owned())? {
            let index_package = crate::files::index::parse(name.to_owned())?;
            let package = Self::make_from(name, index_package, versions_package)?;
            Ok(Some(package))
        } else {
            Ok(None)
        }
    }

    pub fn load_all() -> Result<Vec<Self>, Box<dyn Error>> {
        let mut packages: Vec<Self> = Vec::new();
        let config = AppConfig::new();
        for entry in fs::read_dir(config.versions_path())? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                let name = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .ok_or("Invalid package name")?
                    .to_string();

                if let Some(package) = Self::load(&name)? {
                    debug!("Loading package {}", name);
                    packages.push(package);
                }
            }
        }
        Ok(packages)
    }

    pub fn container_image_url(&self) -> String {
        format!("{}:{}", self.index.image, self.versions.current)
    }

    pub fn print(&self, verbose: bool) {
        info!("- [{}]", self.name);
        if verbose {
            info!("  - image: {}", self.index.image);
            if let Some(volumes) = &self.index.volumes {
                info!("  - volumes:");
                for volume in volumes {
                    info!("    - {}:{}", volume.source, volume.target);
                }
            }
            if let Some(environment_variables) = &self.index.environment_variables {
                info!("  - environment variables:");
                for env_var in environment_variables {
                    info!("    - {}={}", env_var.name, env_var.value);
                }
            }
            if let Some(binaries) = &self.index.binaries {
                info!("  - binaries:");
                for binary in binaries {
                    info!("    - {} -> {}", binary.name, binary.path);
                }
            }
            info!("  - only_shim_binaries: {}", &self.index.only_shim_binaries);
            if let Some(current_directory) = &self.index.current_directory {
                info!("  - current directory: {}", current_directory.clone());
            }
        }
        info!("  - versions:");
        for version in &self.versions.versions {
            if version == &self.versions.current {
                info!("    - {} ✔", version);
            } else {
                info!("    - {}", version);
            }
        }
    }
}

// Private API
impl Package {
    fn make_from(
        name: &str,
        index_package: IndexPackage,
        versions_package: VersionsPackage,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            name: String::from(name),
            index: index_package,
            versions: versions_package,
        })
    }
}
