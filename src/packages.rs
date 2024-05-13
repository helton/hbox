use crate::files::index::Package as IndexPackage;
use crate::files::versions::Package as VersionsPackage;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub index: IndexPackage,
    pub versions: VersionsPackage,
}

// Public API
impl Package {
    pub fn new(name: &str, versions_package: VersionsPackage) -> Result<Self, Box<dyn Error>> {
        let index_packages = crate::files::index::parse()?;
        let index_package = index_packages
            .packages
            .get(name)
            .map(|pkg| pkg.to_owned())
            .unwrap_or_else(|| IndexPackage::new(name));
        Ok(Self {
            name: String::from(name),
            index: index_package,
            versions: versions_package,
        })
    }

    pub fn load(name: &str) -> Result<Option<Self>, Box<dyn Error>> {
        let index_package = crate::files::index::parse()?.packages.remove(name);
        let versions_package = crate::files::versions::parse()?.packages.remove(name);
        Self::make_from(name, index_package, versions_package)
    }

    pub fn load_all() -> Result<Vec<Self>, Box<dyn Error>> {
        let versions_config = crate::files::versions::parse()?;
        let mut packages: Vec<Self> = Vec::new();

        for name in versions_config.packages.keys() {
            if let Some(package) = Self::load(name)? {
                packages.push(package);
            }
        }

        Ok(packages)
    }

    pub fn container_image_url(&self) -> String {
        format!("{}:{}", self.index.image, self.versions.current)
    }

    pub fn print(&self) {
        println!("- {}", self.name);
        for version in &self.versions.versions {
            if version == &self.versions.current {
                println!("  - {} ✔", version);
            } else {
                println!("  - {}", version);
            }
        }
    }
}

// Private API
impl Package {
    fn make_from(
        name: &str,
        index_package: Option<IndexPackage>,
        versions_package: Option<VersionsPackage>,
    ) -> Result<Option<Self>, Box<dyn Error>> {
        match (index_package, versions_package) {
            (Some(index_package), Some(versions_package)) => Ok(Some(Self {
                name: String::from(name),
                index: index_package,
                versions: versions_package,
            })),
            (None, Some(versions_package)) => Ok(Some(Self {
                name: String::from(name),
                index: IndexPackage::new(name),
                versions: versions_package,
            })),
            (_, None) => Ok(None),
        }
    }
}
