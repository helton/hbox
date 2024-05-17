use crate::variables::AppConfig;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

pub fn add_shim(name: &str, binary: Option<&str>) -> std::io::Result<()> {
    let config = AppConfig::new();
    let shim_name = binary.unwrap_or_else(|| name);
    let shims_file_path = get_shims_path(shim_name, config);

    if !shims_file_path.exists() {
        fs::create_dir_all(shims_file_path.parent().unwrap())?;
        let mut shim_file = File::create(&shims_file_path)?;

        let command = match binary {
            Some(bin) => format!("{}::{}", name, bin),
            None => name.to_string()
        };

        if std::env::consts::OS == "windows" {
            shim_file.write_all(b"@echo off\n")?;
            shim_file.write_all(format!("hbox.exe run {} %*\n", command).as_bytes())?;
        } else {
            shim_file.write_all(b"#!/bin/sh\n")?;
            shim_file.write_all(format!("hbox run {} \"$@\"\n", command).as_bytes())?;
        }

        if std::env::consts::OS != "windows" {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = shim_file.metadata()?.permissions();
                perms.set_mode(0o755); // rwx for user, rx for group and others
                fs::set_permissions(&shims_file_path, perms)?;
            }
        }
    }
    Ok(())
}

pub fn remove_shim(name: &str) -> std::io::Result<()> {
    let config = AppConfig::new();
    let shims_file_path = get_shims_path(name, config);

    if shims_file_path.exists() {
        fs::remove_file(shims_file_path)?;
    }
    Ok(())
}

fn get_shims_path(name: &str, config: AppConfig) -> PathBuf {
    if cfg!(target_os = "windows") {
        config.shims_path().join(format!("{}.bat", name))
    } else {
        config.shims_path().join(name)
    }
}
