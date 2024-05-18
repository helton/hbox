use crate::files::config::get_config;
use colored::Colorize;
use env_logger::{Builder, Target};
use log::Level;
use log::LevelFilter;
use std::env;
use std::error::Error;
use std::io::Write;

pub fn setup_logger() -> Result<(), Box<dyn Error>> {
    let log_level = match get_config() {
        Ok(config) => match config.log_level.to_lowercase().as_str() {
            "info" => LevelFilter::Info,
            "warn" => LevelFilter::Warn,
            "debug" => LevelFilter::Debug,
            "trace" => LevelFilter::Trace,
            _ => LevelFilter::Error,
        },
        Err(_) => LevelFilter::Error,
    };

    let mut builder = Builder::new();

    // Log the errors to stderr
    builder.filter_level(LevelFilter::Error);
    builder.target(Target::Stderr);

    // Customize format
    builder.format(move |buf, record| {
        let file_info = match (record.file(), record.line()) {
            (Some(file), Some(line)) => format!("{}:{}", file, line),
            _ => String::from("unknown"),
        };

        let now = chrono::Local::now();
        let timestamp = now.format("%Y-%m-%d %H:%M:%S%.3f");

        match record.level() {
            Level::Trace => {
                let log_line = format!("[{} TRACE {}] - {}", timestamp, file_info, record.args());
                writeln!(buf, "{}", log_line.italic())
            }
            Level::Debug => {
                let log_line = format!("[{} DEBUG {}] - {}", timestamp, file_info, record.args());
                writeln!(buf, "{}", log_line.italic())
            }
            Level::Info => {
                let log_line = format!("{}", record.args());
                writeln!(buf, "{}", log_line)
            }
            Level::Warn => {
                let log_line = format!("{}", record.args());
                writeln!(buf, "{}", log_line.underline())
            }
            Level::Error => {
                let log_line = format!("{}", record.args());
                writeln!(buf, "{}", log_line.red().bold())
            }
        }
    });

    // Log all other stuff to stdout
    builder.filter(Some("hbox"), log_level);
    builder.target(Target::Stdout);

    if env::var("RUST_LOG").is_ok() {
        builder.parse_filters(&env::var("RUST_LOG").unwrap());
    };

    builder.init();

    Ok(())
}
