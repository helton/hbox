use crate::files::config::get_config;
use crate::files::variables::AppConfig;
use chrono::Local;
use log::{Level, LevelFilter, Log, Metadata, Record};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::{Arc, Mutex};

pub fn setup_logger() -> Result<(), Box<dyn Error>> {
    let (log_level, log_strategy) = if let Ok(config) = get_config() {
        (
            match config.logs.level.to_lowercase().as_str() {
                "info" => LevelFilter::Info,
                "warn" => LevelFilter::Warn,
                "debug" => LevelFilter::Debug,
                "trace" => LevelFilter::Trace,
                _ => LevelFilter::Error,
            },
            config.logs.strategy,
        )
    } else {
        (LevelFilter::Info, "truncate".to_string())
    };

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
        .truncate(log_strategy == "truncate")
        .append(log_strategy == "append")
        .open(log_file_path)?;

    let file = Arc::new(Mutex::new(file));

    // Set the logger
    log::set_boxed_logger(Box::new(Logger {
        file: file.clone(),
        level: log_level,
    }))?;
    log::set_max_level(log_level);

    Ok(())
}

struct Logger {
    file: Arc<Mutex<File>>,
    level: LevelFilter,
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
            (Some(file), Some(line)) => format!("{}:{}", file, line),
            _ => String::from("unknown"),
        };

        let log_line = format!(
            "[{} {} {}] - {}",
            timestamp,
            record.level(),
            file_info,
            record.args()
        );

        if self.level <= LevelFilter::Debug
            && (record.level() == Level::Info
                || record.level() == Level::Warn
                || record.level() == Level::Error)
        {
            let mut file = self.file.lock().unwrap();
            writeln!(file, "{}", log_line).unwrap();
        }

        match record.level() {
            Level::Trace | Level::Debug => {
                let mut file = self.file.lock().unwrap();
                writeln!(file, "{}", log_line).unwrap();
            }
            Level::Info => {
                println!("{}", record.args());
            }
            Level::Warn => {
                println!("{}", record.args());
            }
            Level::Error => {
                eprintln!("{}", record.args());
            }
        }
    }

    fn flush(&self) {
        let mut file = self.file.lock().unwrap();
        file.flush().unwrap();
    }
}
