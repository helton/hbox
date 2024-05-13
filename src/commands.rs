use crate::files::versions::{remove, upsert};
use crate::packages;
use crate::packages::Package;
use crate::runner::run;
use crate::shims::add_shim;
use colored::*;
use std::env;
use std::error::Error;

pub fn show_info() -> Result<(), Box<dyn Error>> {
    println!("\n{}:", "System Information".bold().underline());
    println!("{}:", "OS Details".bold());
    println!("  Name          : {}", env::consts::OS.bright_blue());
    println!("  Architecture  : {}", env::consts::ARCH.bright_blue());
    println!("  Family        : {}", env::consts::FAMILY.bright_blue());
    println!("\n{}:", "Application Configuration".bold().underline());
    println!("Version         : {}", env!("CARGO_PKG_VERSION").green());
    println!("Environment Vars :");
    println!(
        "  HBOX_DIR       : {}",
        env::var("HBOX_DIR")
            .unwrap_or_else(|_| String::from("~/.hbox"))
            .bright_yellow()
    );
    println!("\n");
    Ok(())
}

pub fn list_packages(name: Option<&str>) -> Result<(), Box<dyn Error>> {
    if let Some(name) = name {
        if let Some(package) = packages::Package::load(name)? {
            package.print();
            Ok(())
        } else {
            Err(format!(
                "Package '{}' was not found. Add the package first via 'add' command.",
                name
            )
            .into())
        }
    } else {
        let packages = packages::Package::load_all()?;
        if !packages.is_empty() {
            for package in &packages {
                package.print();
            }
            Ok(())
        } else {
            Err(Box::from("Could not find any packages installed."))
        }
    }
}

pub fn add_package(name: String, version: String, set_default: bool) -> Result<(), Box<dyn Error>> {
    if let Some(mut package) = Package::load(&name)? {
        if package.versions.versions.contains(&version) {
            return Err(format!("'{}' version {} already exists.", name, version).into());
        } else {
            package.versions.versions.push(version.clone());
        }
        if set_default {
            package.versions.current = version.clone();
        }
        run_pull_and_add_shim(&name, &version, package)?;
    } else {
        let package = Package::new(&name, crate::files::versions::Package::new(&version))?;
        run_pull_and_add_shim(&name, &version, package)?;
    }
    Ok(())
}

pub fn remove_package(name: String, version: Option<String>) -> Result<(), Box<dyn Error>> {
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
                        remove(&name)?;
                        println!("Removed package '{}'.", name);
                    } else {
                        upsert(&name, package)?;
                        println!("Removed version '{}' of '{}'.", version, name);
                    }
                    Ok(())
                } else {
                    Err(format!("Version '{}' of '{}' does not exists.", version, name).into())
                }
            }
        }
        (Some(_), None) => {
            remove(&name)?;
            Ok(())
        }
        (None, _) => Err(format!("Package '{}' does not exists.", name).into()),
    }
}

pub fn set_package_version(name: String, version: String) -> Result<(), Box<dyn Error>> {
    if let Some(mut package) = Package::load(&name)? {
        if package.versions.versions.contains(&version) {
            package.versions.current = version.clone();
            upsert(&name, package)?;
            println!("'{}' set to version '{}'", name, version);
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
    if let Some(package) = Package::load(&name)? {
        run(&package, &subcommand);
        Ok(())
    } else {
        Err(format!("Package '{}' does not exists.", name).into())
    }
}

fn run_pull_and_add_shim(
    name: &String,
    version: &String,
    package: Package,
) -> Result<(), Box<dyn Error>> {
    let mut new_package = package.clone();
    new_package.versions.current = version.clone();
    if crate::runner::pull(&new_package) {
        upsert(&name, package)?;
        add_shim(&name)?;
        Ok(())
    } else {
        Err(format!("Failed to add package '{}' at version '{}'.", name, version).into())
    }
}
