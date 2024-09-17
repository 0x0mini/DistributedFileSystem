use serde::{Deserialize, Serialize};
use std::env;
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub file_path: String,
    pub max_connections: i32,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv::dotenv().ok();

        let file_path = env::var("FILE_PATH")?;
        let max_connections = env::var("MAX_CONNECTIONS")?.parse::<i32>()?;

        Ok(Self {
            file_path,
            max_connections,
        })
    }

    pub fn from_json(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file_content = fs::read_to_string(file_path)?;
        let config = serde_json::from_str::<Self>(&file_content)?;

        if config.max_connections <= 0 {
            return Err("max_connections must be greater than 0".into());
        }

        Ok(config)
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        if self.max_connections < 1 {
            Err("Max connections must be at least 1")
        } else {
            Ok(())
        }
    }
}