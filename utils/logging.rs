use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
}

pub struct Logger {
    log_file_path: String,
    log_level: LogLevel,
}

impl Logger {
    pub fn new() -> io::Result<Self> {
        let log_file_path = env::var("LOG_FILE_PATH").unwrap_or_else(|_| "log.txt".to_string());
        let log_level = match env::var("LOG_LEVEL").as_deref() {
            Ok("ERROR") => LogLevel::Error,
            Ok("WARNING") => LogLevel::Warning,
            Ok("INFO") => LogLevel::Info,
            Ok("DEBUG") | _ => LogLevel::Debug,
        };

        Ok(Self {
            log_file_path,
            log_level,
        })
    }

    pub fn log(&self, level: LogLevel, message: &str) -> io::Result<()> {
        if self.log_level as u8 >= level as u8 {
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(&self.log_file_path)?;
            writeln!(file, "[{:?}] {}", level, message)?;
        }
        Ok(())
    }

    pub fn error(&self, message: &str) -> io::Result<()> {
        self.log(LogLevel::Error, message)
    }

    pub fn warning(&self, message: &str) -> io::Result<()> {
        self.log(LogLevel::Warning, message)
    }

    pub fn info(&self, message: &str) -> io::Result<()> {
        self.log(LogLevel::Info, message)
    }

    pub fn debug(&self, message: &str) -> io::Result<()> {
        self.log(LogLevel::Debug, message)
    }

    pub fn delete_log_file(&self) -> io::Result<()> {
        std::fs::remove_file(&self.log_file_path)?;
        Ok(())
    }
}

fn example_use() -> io::Result<()> {
    let logger = Logger::new()?;
    logger.debug("This is a debug message")?;
    logger.info("This is an info message")?;
    logger.warning("This is a warning message")?;
    logger.error("This is an error message")?;
    
    Ok(())
}