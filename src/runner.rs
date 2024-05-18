use log::{debug, error, info, warn};
use std::io::{stdin, IsTerminal, Read, Write, BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};
use crate::packages::Package;

fn run_command(command: &str, stdin_buffer: Option<Vec<u8>>) -> bool {
    debug!("Running command: {}", command);

    let mut parts = command.split_whitespace();
    let cmd = parts.next().unwrap(); // Extract command
    let args = parts.collect::<Vec<_>>(); // Extract arguments

    let mut child = Command::new(cmd)
        .args(args)
        .stdout(Stdio::piped()) // Redirect stdout to a pipe
        .stderr(Stdio::piped()) // Redirect stderr to a pipe
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

    // Function to log output from a pipe
    fn log_output<R: BufRead>(reader: R, log_fn: impl Fn(&str)) {
        for line in reader.lines() {
            match line {
                Ok(line) => log_fn(&line),
                Err(e) => error!("Failed to read line from output: {}", e),
            }
        }
    }

    // Read the child's stdout and stderr in separate threads and log them
    let stdout = child.stdout.take().expect("Failed to open stdout");
    let stderr = child.stderr.take().expect("Failed to open stderr");

    let stdout_thread = std::thread::spawn(move || {
        log_output(BufReader::new(stdout), |line| info!("{}", line));
    });

    let stderr_thread = std::thread::spawn(move || {
        log_output(BufReader::new(stderr), |line| error!("{}", line));
    });

    // Wait for the command to complete
    let status = child.wait();

    // Wait for the logging threads to finish
    let _ = stdout_thread.join();
    let _ = stderr_thread.join();

    // Check the command status
    match status {
        Ok(status) => status.success(),
        Err(e) => {
            error!("Command failed to complete: {}", e);
            false
        }
    }
}

pub fn run(package: &Package, binary: Option<String>, params: &Vec<String>) -> bool {
    let interactive = !stdin().is_terminal();

    let mut buffer = Vec::new();
    if interactive {
        stdin()
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
                warn!("Volume source '{}' not found. Skipping.", source);
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
