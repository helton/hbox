use crate::packages::Package;

pub struct Context {
    package_name: String,
    package_version: String,
}

impl Context {
    pub fn from(package: &Package) -> Self {
        Self {
            package_name: package.name.clone(),
            package_version: package.versions.current.clone(),
        }
    }

    pub fn apply(&self, mut text: String) -> String {
        text = text.replace("${hbox_package_name}", &self.package_name);
        text = text.replace("${hbox_package_version}", &self.package_version);
        text
    }
}
