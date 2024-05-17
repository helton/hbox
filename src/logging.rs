use std::env;
use std::error::Error;
use log::LevelFilter;
use log::{Level};
use env_logger::{Builder, Target};
use crate::files::config::{get_config};
use std::io::Write;
use colored::Colorize;

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
        match record.level() {
            Level::Debug => {
                let file_info = match (record.file(), record.line()) {
                    (Some(file), Some(line)) => format!("{}:{}", file, line),
                    _ => String::from("unknown"),
                };

                let log_line = format!("[{} {} {}] {}",
                                       chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"),
                                       record.target(),
                                       file_info,
                                       record.args());
                writeln!(buf, "{}", log_line)
            },
            Level::Error => {
                let log_line = format!("{}", record.args());
                writeln!(buf, "{}", log_line.red())
            },
            Level::Warn => {
                let log_line = format!("{}", record.args());
                writeln!(buf, "{}", log_line.yellow())
            },
            _ => {
                let log_line = format!("{}", record.args());
                writeln!(buf, "{}", log_line)
            },
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