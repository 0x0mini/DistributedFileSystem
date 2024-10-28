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
    log_file_handle: Option<File>, // Caching the file handle
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
            log_file_handle: None, // Initially, there is no file handle
        })
    }

    pub fn log(&mut self, level: LogLevel, message: &str) -> io::Result<()> {
        if self.log_level as u8 >= level as u8 {
            if self.log_file_handle.is_none() {
                let file = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(&self.log_file_path)?;
                self.log_file_handle = Some(file);
            }

            if let Some(ref mut file) = self.log_file_handle {
                writeln!(file, "[{:?}] {}", level, message)?;
            }
        }
        Ok(())
    }

    pub fn error(&mut self, message: &str) -> io::Result<()> {
        self.log(LogLevel::Error, message)
    }

    pub fn warning(&mut self, message: &str) -> io::Result<()> {
        self.log(LogLevel::Warning, message)
    }

    pub fn info(&mut self, message: &str) -> io::Result<()> {
        self.log(LogLevel::Info, message)
    }

    pub fn debug(&mut self, message: &str) -> io::Result<()> {
        self.log(LogLevel::Debug, message)
    }

    pub fn delete_log_file(&mut self) -> io::Result<()> {
        // Ensure the file handle is closed before trying to delete the file
        self.log_file_handle = None;
        std::fs::remove_file(&self.log_file_path)?;
        Ok(())
    }
}

fn example_use() -> io::Result<()> {
    let mut logger = Logger::new()?;
    logger.debug("This is a debug message")?;
    logger.info("This is an info message")?;
    logger.warning("This is a warning message")?;
    logger.error("This is an error message")?;
    
    Ok(())
}