use crate::files::config::Strategy::{Append, Truncate};
use crate::files::config::{get_config, Logs};
use crate::files::variables::AppConfig;
use chrono::Local;
use log::{Level, LevelFilter, Log, Metadata, Record};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::MAIN_SEPARATOR;
use std::sync::{Arc, Mutex};

pub fn setup_logger() -> Result<(), Box<dyn Error>> {
    let logs_config = get_config().map_or_else(|_| Logs::new(), |config| config.logs);

    let config = AppConfig::new();
    let log_file_path = config.logs_path();

    // Ensure the logs directory exists
    if let Some(parent) = log_file_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Open or create the log file
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(logs_config.strategy == Truncate)
        .append(logs_config.strategy == Append)
        .open(log_file_path)?;

    let file = Arc::new(Mutex::new(file));

    // Set the logger
    log::set_boxed_logger(Box::new(Logger {
        file: file.clone(),
        level: logs_config.level.to_level_filter(),
        enabled: logs_config.enabled,
    }))?;
    log::set_max_level(logs_config.level.to_level_filter());

    Ok(())
}

struct Logger {
    file: Arc<Mutex<File>>,
    level: LevelFilter,
    enabled: bool,
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let now = Local::now();
        let timestamp = now.format("%Y-%m-%d %H:%M:%S%.3f");

        let file_info = match (record.file(), record.line()) {
            (Some(file), Some(line)) => format!("{}:{}", strip_src_prefix(file).to_string(), line),
            _ => String::from("unknown"),
        };

        // Define fixed widths
        let log_line = format!(
            "[{:<width$} {:<level_width$} {:<file_info_width$}] {}",
            timestamp,
            record.level(),
            file_info,
            record.args(),
            width = 23,
            level_width = 5,
            file_info_width = 23
        );

        if self.enabled {
            let mut file = self.file.lock().unwrap();
            writeln!(file, "{}", log_line).unwrap();
        }

        match record.level() {
            Level::Info | Level::Warn => {
                println!("{}", record.args());
            }
            Level::Error => {
                eprintln!("{}", record.args());
            }
            _ => (),
        }
    }

    fn flush(&self) {
        let mut file = self.file.lock().unwrap();
        file.flush().unwrap();
    }
}

fn strip_src_prefix(file_path: &str) -> &str {
    let prefix = format!("src{}", MAIN_SEPARATOR);
    file_path.strip_prefix(&prefix).unwrap_or(file_path)
}
