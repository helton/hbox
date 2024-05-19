use crate::configs::index::Binary;
use crate::configs::user::UserConfig;
use crate::packages::Package;
use log::{debug, error, info, warn};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::io::{stdin, BufRead, BufReader, IsTerminal, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;

pub fn pull(package: &Package) -> bool {
    let image = format!("{}:{}", package.index.image, package.versions.current);
    run_command_with_args("docker", &["pull".to_string(), image], None)
}

pub fn run(package: &Package, binary: Option<String>, params: &Vec<String>) -> bool {
    let interactive = !stdin().is_terminal();
    let mut buffer = Vec::new();
    if interactive {
        stdin()
            .read_to_end(&mut buffer)
            .expect("Failed to read stdin");
    }

    let mut args = vec!["run".to_string()];
    args.push(if interactive {
        "-i".to_string()
    } else {
        "-it".to_string()
    });

    let binary = get_binary(package, &binary);

    add_default_flags(package, &mut args);
    add_volumes(package, &mut args);
    add_current_directory(package, &mut args);
    add_environment_variables(package, &mut args);
    add_binary_entrypoint(binary, &mut args);
    add_container_image(package, &mut args);
    add_binary_cmd(binary, &mut args);

    if should_wrap_args(binary) {
        debug!("Wrapping params in quotes");
        let escaped_params: Vec<String> = params
            .iter()
            .map(|param| param.replace("\"", "\\\""))
            .collect();
        args.push(escaped_params.join(" "));
    } else {
        args.extend(params.iter().cloned());
    }

    run_command_with_args("docker", &args, Some(buffer))
}

fn should_wrap_args(binary: Option<&Binary>) -> bool {
    binary.map_or(false, |bin| bin.wrap_args)
}

fn generate_random_name(package: &Package) -> String {
    let id: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    format!("hbox-{}-{}-{}", package.name, package.versions.current, id)
}

fn add_default_flags(package: &Package, args: &mut Vec<String>) {
    args.push("--rm".to_string());
    args.push("--name".to_string());
    args.push(generate_random_name(package));
}

fn add_container_image(package: &Package, args: &mut Vec<String>) {
    args.push(format!(
        "{}:{}",
        package.index.image, package.versions.current
    ));
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
            let expanded_value = shellexpand::full(&env_var.value).unwrap_or_default();
            args.push("-e".to_string());
            args.push(format!("{}={}", env_var.name, expanded_value));
        }
    }
}

fn add_binary_entrypoint(binary: Option<&Binary>, args: &mut Vec<String>) {
    if let Some(binary) = binary {
        args.push("--entrypoint".to_string());
        args.push(binary.path.to_string());
    }
}

fn add_binary_cmd(binary: Option<&Binary>, args: &mut Vec<String>) {
    if let Some(binary) = binary {
        if let Some(cmd) = &binary.cmd {
            args.extend(cmd.iter().cloned());
        }
    }
}

fn get_binary<'a>(package: &'a Package, binary: &Option<String>) -> Option<&'a Binary> {
    binary.as_ref().and_then(|b| {
        package
            .index
            .binaries
            .as_ref()
            .and_then(|binaries| binaries.iter().find(|binary| binary.name == *b))
    })
}

fn get_stdio(
    config: &crate::configs::user::Root,
    stdin_buffer: &Option<Vec<u8>>,
) -> (Stdio, Stdio, Stdio) {
    let stdin = if let Some(b) = stdin_buffer {
        if b.is_empty() {
            Stdio::inherit()
        } else {
            Stdio::piped()
        }
    } else {
        Stdio::inherit()
    };

    let stdout = if config.experimental.capture_stdout {
        Stdio::piped()
    } else {
        Stdio::inherit()
    };
    let stderr = if config.experimental.capture_stderr {
        Stdio::piped()
    } else {
        Stdio::inherit()
    };

    (stdin, stdout, stderr)
}

fn run_command_with_args(command: &str, args: &[String], stdin_buffer: Option<Vec<u8>>) -> bool {
    debug!("Running command: {} {}", command, args.join(" "));

    let config = UserConfig::load().unwrap_or_default();
    let (stdin, stdout, stderr) = get_stdio(&config, &stdin_buffer);

    let mut child = Command::new(command)
        .args(args)
        .stdout(stdout)
        .stderr(stderr)
        .stdin(stdin)
        .spawn()
        .expect("Failed to spawn command");

    if let Some(buffer) = stdin_buffer {
        if !buffer.is_empty() {
            let child_stdin = child.stdin.as_mut().expect("Failed to open stdin");
            child_stdin
                .write_all(&buffer)
                .expect("Failed to write to stdin");
        }
    }

    let stdout_thread = spawn_log_thread(
        child.stdout.take(),
        |line| info!("{}", line),
        config.experimental.capture_stdout,
    );
    let stderr_thread = spawn_log_thread(
        child.stderr.take(),
        |line| error!("{}", line),
        config.experimental.capture_stderr,
    );

    let status = child.wait().expect("Failed to wait on child process");

    if let Some(thread) = stdout_thread {
        let _ = thread.join();
    }

    if let Some(thread) = stderr_thread {
        let _ = thread.join();
    }

    status.success()
}

fn spawn_log_thread<R: Read + Send + 'static>(
    reader: Option<R>,
    log_fn: impl Fn(&str) + Send + 'static,
    capture: bool,
) -> Option<thread::JoinHandle<()>> {
    if !capture {
        return None;
    }
    let reader = reader.expect("Failed to open reader");
    Some(thread::spawn(move || {
        let reader = BufReader::new(reader);
        for line in reader.split(b'\n') {
            match line {
                Ok(line) => match std::str::from_utf8(&line) {
                    Ok(line) => log_fn(line),
                    Err(_) => error!(
                        "Failed to read line from output: stream did not contain valid UTF-8"
                    ),
                },
                Err(e) => error!("Failed to read line from output: {}", e),
            }
        }
    }))
}
