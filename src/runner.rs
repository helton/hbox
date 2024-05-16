use crate::packages::Package;
use atty::Stream;
use std::io::{self, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};

fn run_command(command: &str, stdin_buffer: Option<Vec<u8>>) -> bool {
    println!("Running command: {}", command);

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
            println!("Command failed to complete: {}", e);
            false
        }
    }
}

pub fn run(package: &Package, params: &Vec<String>) -> bool {
    let interactive = !atty::is(Stream::Stdin);

    let mut buffer = Vec::new();
    if interactive {
        io::stdin()
            .read_to_end(&mut buffer)
            .expect("Failed to read stdin");
    }

    let mut args = vec!["run".to_string()];
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
                println!("Volume source '{}' not found. Skipping.", source);
            }
        }
    }

    if let Some(current_directory) = &package.index.current_directory {
        args.push("-w".to_string());
        args.push(current_directory.clone());
    }

    args.push(format!(
        "{}:{}",
        package.index.image.clone(),
        package.versions.current
    ));
    args.extend(params.iter().cloned());

    let command = format!("docker {}", args.join(" "));
    println!("Running {}", command);
    run_command(&command, Some(buffer))
}

pub fn pull(package: &Package) -> bool {
    let command = format!("docker pull {}", package.container_image_url());
    run_command(&command, None)
}
