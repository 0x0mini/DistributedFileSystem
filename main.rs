use std::env;
use std::sync::{Arc, Mutex};
use thiserror::Error;

mod server;
mod client;
mod distributed_file_system;
mod logger;

#[derive(Debug, Error)]
enum AppError {
    #[error("Environment variable error: {0}")]
    EnvVarError(String),
    #[error("Server error: {0}")]
    ServerError(String),
    #[error("Client error: {0}")]
    ClientError(String),
    #[error("File System error: {0}")]
    FileSystemError(String),
}

fn main() -> Result<(), AppError> {
    dotenv::dotenv().ok();

    let server_address = env::var("SERVER_ADDRESS").map_err(|_| AppError::EnvVarError("SERVER_ADDRESS must be set".to_string()))?;
    let client_mode = env::var("CLIENT_MODE").unwrap_or_else(|_| "false".to_string());

    let dfs = Arc::new(Mutex::new(distributed_file_system::DistributedFileSystem::new()));

    if client_mode == "true" {
        client::start(dfs.clone(), &server_address)?;
    } else {
        server::start(dfs.clone(), &server_address)?;
    }

    Ok(())
}

mod server {
    use super::distributed_file_system::DistributedFileSystem;
    use super::{AppError, logger::log};
    use std::sync::{Arc, Mutex};

    pub fn start(dfs: Arc<Mutex<DistributedFileSystem>>, address: &str) -> Result<(), AppError> {
        log(format!("Starting server at {}", address));
        {
            let mut dfs = dfs.lock().unwrap();
            dfs.add_file("example.txt".to_string(), "Hello, Distributed World!".to_string())?;
        }
        
        let file_names = { dfs.lock().unwrap().list_file_names()? };
        log(format!("Current files in the system: {:?}", file_names));
        
        Err(AppError::ServerError("Failed to start the server".into()))
    }
}

mod client {
    use super::distributed_file_system::DistributedFileSystem;
    use super::{AppError, logger::log};
    use std::sync::{Arc, Mutex};

    pub fn start(dfs: Arc<Mutex<DistributedFileSystem>>, server_address: &str) -> Result<(), AppError> {
        log(format!("Connecting to server at {}", server_address));
        {
            let file_content = dfs.lock().unwrap().get_file_content("example.txt".into())?;
            log(format!("Retrieved file content: {}", file_content));
        }
        
        Err(AppError::ClientError("Failed to connect to the server".into()))
    }
}

mod distributed_file_system {
    use super::AppError;
    use std::collections::HashMap;

    pub struct DistributedFileSystem {
        files: HashMap<String, String>,
    }

    impl DistributedFileSystem {
        pub fn new() -> Self {
            DistributedFileSystem {
                files: HashMap::new(),
            }
        }

        pub fn add_file(&mut self, file_name: String, content: String) -> Result<(), AppError> {
            self.files.insert(file_name, content);
            Ok(())
        }

        pub fn get_file_content(&self, file_name: String) -> Result<String, AppError> {
            self.files.get(&file_name).cloned().ok_or(AppError::FileSystemError("File not found".into()))
        }

        pub fn list_file_names(&self) -> Result<Vec<String>, AppError> {
            Ok(self.files.keys().cloned().collect())
        }
    }
}

mod logger {
    pub fn log(message: String) {
        println!("{}", message);
    }
}