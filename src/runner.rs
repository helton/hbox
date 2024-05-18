use crate::packages::Package;
use log::{debug, error, info, warn};
use std::io::{stdin, BufRead, BufReader, IsTerminal, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};

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

    add_volumes(&package, &mut args);
    add_current_directory(&package, &mut args);
    add_environment_variables(&package, &mut args);
    add_binary_entrypoint(&package, &binary, &mut args);

    args.push(format!(
        "{}:{}",
        package.index.image.clone(),
        package.versions.current
    ));
    args.extend(params.iter().cloned());

    run_command_with_args("docker", &args, Some(buffer))
}

fn add_volumes(package: &Package, args: &mut Vec<String>) {
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
}

fn add_current_directory(package: &Package, args: &mut Vec<String>) {
    if let Some(current_directory) = &package.index.current_directory {
        args.push("-w".to_string());
        args.push(current_directory.clone());
    }
}

fn add_environment_variables(package: &Package, args: &mut Vec<String>) {
    if let Some(environment_variables) = &package.index.environment_variables {
        for env_var in environment_variables {
            args.push("-e".to_string());
            args.push(format!("{}={}", env_var.name, env_var.value));
        }
    }
}

fn add_binary_entrypoint(package: &Package, binary: &Option<String>, args: &mut Vec<String>) {
    if let Some(b) = binary {
        if let Some(binaries) = &package.index.binaries {
            for binary in binaries {
                if binary.name == *b {
                    args.push("--entrypoint".to_string());
                    args.push(binary.path.to_string());
                }
            }
        }
    }
}

fn run_command_with_args(command: &str, args: &[String], stdin_buffer: Option<Vec<u8>>) -> bool {
    debug!("Running command: {} {:?}", command, args);

    let mut child = Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to spawn command");

    if let Some(buffer) = stdin_buffer {
        let child_stdin = child.stdin.as_mut().expect("Failed to open stdin");
        child_stdin
            .write_all(&buffer)
            .expect("Failed to write to stdin");
    }

    let stdout_thread = spawn_log_thread(
        BufReader::new(child.stdout.take().expect("Failed to open stdout")),
        |line| info!("{}", line),
    );
    let stderr_thread = spawn_log_thread(
        BufReader::new(child.stderr.take().expect("Failed to open stderr")),
        |line| error!("{}", line),
    );

    let status = child.wait();
    let _ = stdout_thread.join();
    let _ = stderr_thread.join();

    match status {
        Ok(status) => status.success(),
        Err(e) => {
            error!("Command failed to complete: {}", e);
            false
        }
    }
}

fn spawn_log_thread<R: BufRead + Send + 'static>(
    reader: R,
    log_fn: impl Fn(&str) + Send + 'static,
) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        for line in reader.lines() {
            match line {
                Ok(line) => log_fn(&line),
                Err(e) => error!("Failed to read line from output: {}", e),
            }
        }
    })
}

pub fn pull(package: &Package) -> bool {
    let command = format!("docker pull {}", package.container_image_url());
    run_command_with_args("docker", &[command], None)
}
