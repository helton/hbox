use crate::packages::Package;
use atty::Stream;
use std::io::{self, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use log::{debug, error};

fn run_command(command: &str, stdin_buffer: Option<Vec<u8>>) -> bool {
    debug!("Running command: {}", command);

    let mut parts = command.split_whitespace();
    let cmd = parts.next().unwrap(); // Extract command
    let args = parts.collect::<Vec<_>>(); // Extract arguments

    let mut child = Command::new(cmd)
        .args(args)
        .stdout(Stdio::inherit()) // Inherit stdout
        .stderr(Stdio::inherit()) // Inherit stderr
        .stdin(Stdio::piped()) // Set stdin to piped to write the buffer later
        .spawn() // Spawn the command
        .expect("Failed to spawn command");

    // If stdin_buffer is Some, write it to the child's stdin
    if let Some(buffer) = stdin_buffer {
        let child_stdin = child.stdin.as_mut().expect("Failed to open stdin");
        child_stdin
            .write_all(&buffer)
            .expect("Failed to write to stdin");
    }

    // Wait for the command to complete and check the status
    match child.wait() {
        Ok(status) => status.success(),
        Err(e) => {
            error!("Command failed to complete: {}", e);
            false
        }
    }
}

pub fn run(package: &Package, binary: Option<String>, params: &Vec<String>) -> bool {
    let interactive = !atty::is(Stream::Stdin);

    let mut buffer = Vec::new();
    if interactive {
        io::stdin()
            .read_to_end(&mut buffer)
            .expect("Failed to read stdin");
    }

    let mut args = vec!["run".to_string(), "--rm".to_string()];
    if interactive {
        args.push("-i".to_string());
    }

    if let Some(volumes) = &package.index.volumes {
        for volume in volumes {
            let source = shellexpand::full(&volume.source).unwrap();
            if Path::new(&source.to_string()).exists() {
                args.push("-v".to_string());
                args.push(format!("{}:{}", &source, volume.target));
            } else {
                debug!("Volume source '{}' not found. Skipping.", source);
            }
        }
    }

    if let Some(current_directory) = &package.index.current_directory {
        args.push("-w".to_string());
        args.push(current_directory.clone());
    }

    if let Some(environment_variables) = &package.index.environment_variables {
        for env_var in environment_variables {
            args.push("-e".to_string());
            args.push(format!("{}={}", env_var.name, env_var.value));
        }
    }

    if let Some(b) = binary {
        if let Some(binaries) = &package.index.binaries {
            for binary in binaries {
                if binary.name == b {
                    args.push("--entrypoint".to_string());
                    args.push(binary.path.to_string());
                }
            }
        }
    }

    args.push(format!(
        "{}:{}",
        package.index.image.clone(),
        package.versions.current
    ));
    args.extend(params.iter().cloned());

    let command = format!("docker {}", args.join(" "));
    run_command(&command, Some(buffer))
}

pub fn pull(package: &Package) -> bool {
    let command = format!("docker pull {}", package.container_image_url());
    run_command(&command, None)
}
