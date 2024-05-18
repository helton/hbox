use crate::files::versions::{remove, upsert};
use crate::packages::Package;
use crate::runner::run;
use crate::shims::{add_shim, remove_shim};
use colored::*;
use log::{debug, info};
use std::env;
use std::error::Error;

pub fn show_info() -> Result<(), Box<dyn Error>> {
    debug!("Showing info");
    info!("\n{}:", "System Information".bold().underline());
    info!("{}:", "OS Details".bold());
    info!("  Name          : {}", env::consts::OS.bright_blue());
    info!("  Architecture  : {}", env::consts::ARCH.bright_blue());
    info!("  Family        : {}", env::consts::FAMILY.bright_blue());
    info!("\n{}:", "Application Configuration".bold().underline());
    info!("Version         : {}", env!("CARGO_PKG_VERSION").green());
    info!("Environment Vars :");
    info!(
        "  HBOX_DIR       : {}",
        env::var("HBOX_DIR")
            .unwrap_or_else(|_| String::from("~/.hbox"))
            .bright_yellow()
    );
    info!("\n");
    Ok(())
}

pub fn list_packages(name: Option<&str>, verbose: bool) -> Result<(), Box<dyn Error>> {
    debug!("Listing packages");
    if let Some(name) = name {
        if let Some(package) = Package::load(name)? {
            package.print(verbose);
            Ok(())
        } else {
            Err(format!(
                "Package '{}' was not found. Add the package first via 'add' command.",
                name
            )
            .into())
        }
    } else {
        let packages = Package::load_all()?;
        if !packages.is_empty() {
            for package in &packages {
                package.print(verbose);
            }
            Ok(())
        } else {
            Err("Could not find any packages installed.".into())
        }
    }
}

pub fn add_package(name: String, version: String, set_default: bool) -> Result<(), Box<dyn Error>> {
    debug!(
        "Adding package '{}' version '{}'. Set as default? = {}",
        &name, &version, set_default
    );
    if let Some(mut package) = Package::load(&name)? {
        if package.versions.versions.contains(&version) {
            return Err(format!("'{}' version {} already exists.", name, version).into());
        } else {
            package.versions.versions.push(version.clone());
        }
        if set_default {
            package.versions.current = version.clone();
        }
        let current = package.versions.current.clone();
        do_add_package(&name, &version, package)?;
        info!(
            "Added '{}' version '{}'. Current version is '{}'.",
            name, version, current
        );
    } else {
        let package = Package::new(&name, crate::files::versions::Package::new(&version))?;
        do_add_package(&name, &version, package)?;
        info!(
            "Added '{}' version '{}'. Current version is '{}'.",
            name, version, version
        );
    }
    Ok(())
}

pub fn remove_package(name: String, version: Option<String>) -> Result<(), Box<dyn Error>> {
    debug!(
        "Removing version '{}' of '{}'",
        version.clone().unwrap_or_else(|| "<all>".to_owned()),
        &name
    );
    match (Package::load(&name)?, version) {
        (Some(mut package), Some(version)) => {
            if package.versions.current == version && package.versions.versions.len() > 1 {
                Err(format!(
                    "Cannot remove the current active version '{}' of '{}'.",
                    version, name
                )
                .into())
            } else {
                if package.versions.versions.contains(&version) {
                    package.versions.versions.retain(|v| v != version.as_str());
                    if package.versions.versions.is_empty() {
                        do_remove_package(package)?;
                        info!("Removed package '{}'.", name);
                    } else {
                        upsert(&name, package)?;
                        info!("Removed version '{}' of '{}'.", version, name);
                    }
                    Ok(())
                } else {
                    Err(format!("Version '{}' of '{}' does not exists.", version, name).into())
                }
            }
        }
        (Some(package), None) => {
            do_remove_package(package)?;
            info!("Removed package '{}'.", name);
            Ok(())
        }
        (None, _) => Err(format!("Package '{}' does not exists.", name).into()),
    }
}

pub fn set_package_version(name: String, version: String) -> Result<(), Box<dyn Error>> {
    debug!("Setting package '{}' to version '{}'", &name, &version);
    if let Some(mut package) = Package::load(&name)? {
        if package.versions.versions.contains(&version) {
            package.versions.current = version.clone();
            upsert(&name, package)?;
            info!("Package '{}' set to version '{}'", name, version);
            Ok(())
        } else {
            Err(format!(
                "Version '{}' of package '{}' not found. Add the version first via 'add' command.",
                version, name
            )
            .into())
        }
    } else {
        Err(format!("Package '{}' does not exists.", name).into())
    }
}

pub fn run_package(name: String, subcommand: Vec<String>) -> Result<(), Box<dyn Error>> {
    debug!("Running {} {}", &name, &subcommand.join(" "));
    let parts: Vec<&str> = name.split("::").collect();
    let (package_name, binary) = match parts.as_slice() {
        [package_name] => (package_name.to_string(), None),
        [package_name, binary] => (package_name.to_string(), Some(binary.to_string())),
        _ => return Err(format!("Invalid package name '{}'.", name).into()),
    };

    if let Some(package) = Package::load(&package_name)? {
        run(&package, binary, &subcommand);
        Ok(())
    } else {
        Err(format!("Package '{}' does not exists.", name).into())
    }
}

fn do_add_package(name: &String, version: &String, package: Package) -> Result<(), Box<dyn Error>> {
    let mut new_package = package.clone();
    new_package.versions.current = version.clone();
    if crate::runner::pull(&new_package) {
        if !&package.index.only_shim_binaries {
            add_shim(&name, None)?;
        }
        if let Some(binaries) = &package.index.binaries {
            for binary in binaries {
                add_shim(&name, Some(&binary.name))?;
            }
        }
        upsert(&name, package)?;
        Ok(())
    } else {
        Err(format!("Failed to add package '{}' at version '{}'.", name, version).into())
    }
}

fn do_remove_package(package: Package) -> Result<(), Box<dyn Error>> {
    remove(&package.name)?;
    if !&package.index.only_shim_binaries {
        remove_shim(&package.name)?;
    }
    if let Some(binaries) = &package.index.binaries {
        for binary in binaries {
            remove_shim(&binary.name)?;
        }
    }
    Ok(())
}
