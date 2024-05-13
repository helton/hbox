use std::io::{self, Read, Write};
use std::process::{Command, Stdio};
use atty::Stream;
use crate::packages::Package;

fn run_command(command: &str, stdin_buffer: Option<Vec<u8>>) -> bool {
    println!("Running command: {}", command);

    let mut parts = command.split_whitespace();
    let cmd = parts.next().unwrap();  // Extract command
    let args = parts.collect::<Vec<_>>();  // Extract arguments

    let mut child = Command::new(cmd)
        .args(args)
        .stdout(Stdio::inherit())  // Inherit stdout
        .stderr(Stdio::inherit())  // Inherit stderr
        .stdin(Stdio::piped())  // Set stdin to piped to write the buffer later
        .spawn()  // Spawn the command
        .expect("Failed to spawn command");

    // If stdin_buffer is Some, write it to the child's stdin
    if let Some(buffer) = stdin_buffer {
        let child_stdin = child.stdin.as_mut().expect("Failed to open stdin");
        child_stdin.write_all(&buffer).expect("Failed to write to stdin");
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
        io::stdin().read_to_end(&mut buffer).expect("Failed to read stdin");
        buffer.len();
    }

    let interactive_flag = if interactive { "-i " } else { "" };
    let command = format!("docker run {}{} {}", interactive_flag, package.container_image_url(), params.join(" "));

    run_command(&command, Some(buffer))
}

pub fn pull(package: &Package) -> bool {
    let command = format!("docker pull {}", package.container_image_url());
    // Normally, pull does not need stdin interaction
    run_command(&command, None)
}
