use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

struct Config {
    storage_path: String,
}

impl Config {
    fn new() -> Self {
        let storage_path = env::var("STORAGE_PATH").expect("STORAGE_PATH must be set");
        Config { storage_path }
    }
}

enum Command {
    Upload(String, Vec<u8>),
    Download(String),
    Delete(String),
    Search(String),
}

fn process_command(config: &Config, command: Command) -> Result<(), String> {
    match command {
        Command::Upload(filename, data) => {
            println!("Uploading file: {}", filename);
            upload_file(config, filename, data).map_err(|e| format!("Upload failed: {}", e))
        }
        Command::Download(filename) => {
            println!("Downloading file: {}", filename);
            download_file(config, filename).map_err(|e| format!("Download failed: {}", e))
        }
        Command::Delete(filename) => {
            println!("Deleting file: {}", filename);
            delete_file(config, filename).map_err(|e| format!("Deletion failed: {}", e))
        }
        Command::Search(query) => {
            println!("Searching for files containing: {}", query);
            search_files(config, query).map_err(|e| format!("Search failed: {}", e))
        }
    }
}

fn upload_file(config: &Config, filename: String, data: Vec<u8>) -> io::Result<()> {
    let path = Path::new(&config.storage_path).join(&filename);
    let mut file = File::create(path).map_err(|e| io::Error::new(e.kind(), format!("Failed to create file {}: {}", filename, e)))?;
    file.write_all(&data).map_err(|e| io::Error::new(e.kind(), format!("Failed to write to file {}: {}", filename, e)))?;
    println!("Upload successful.");
    Ok(())
}

fn download_file(config: &Config, filename: String) -> io::Result<()> {
    let path = Path::new(&config.storage_path).join(&filename);
    let mut file = File::open(path).map_err(|e| io::Error::new(e.kind(), format!("Failed to open file {}: {}", filename, e)))?;
    let mut data = Vec::new();
    file.read_to_end(&mut data).map_err(|e| io::Error::new(e.kind(), format!("Failed to read file {}: {}", filename, e)))?;
    println!("Download successful. Data size: {} bytes", data.len());
    Ok(())
}

fn delete_file(config: &Config, filename: String) -> io::Result<()> {
    let path = Path::new(&config.storage_path).join(&filename);
    std::fs::remove_file(path).map_err(|e| io::Error::new(e.kind(), format!("Failed to delete file {}: {}", filename, e)))?;
    println!("File deleted successfully.");
    Ok(())
}

fn search_files(config: &Config, query: String) -> io::Result<()> {
    let mut found = false;
    for entry in std::fs::read_dir(&config.storage_path).map_err(|e| io::Error::new(e.kind(), format!("Failed to read directory {}: {}", &config.storage_path, e)))? {
        let entry = entry.map_err(|e| io::Error::new(e.kind(), format!("Failed to read directory entry: {}", e)))?;
        let path = entry.path();
        if path.is_file() && path.to_string_lossy().contains(&query) {
            println!("Found: {}", path.display());
            found = true;
        }
    }
    if !found {
        println!("No files found matching the query.");
    }
    Ok(())
}

fn main() -> Result<(), String> {
    let config = Config::new();

    process_command(&config, Command::Upload("example.txt".to_string(), b"Hello World!".to_vec()))?;
    process_command(&config, Command::Download("example.txt".to_string()))?;
    process_command(&config, Command::Delete("example.txt".to_string()))?;
    process_command(&config, Command::Search("example".to_string()))?;

    Ok(())
}