use std::env;
use std::fs::File;
use std::path::Path;
use std::io::{self, Read, Write};

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

fn process_command(config: &Config, command: Command) -> io::Result<()> {
    match command {
        Command::Upload(filename, data) => {
            println!("Uploading file: {}", filename);
            upload_file(&config, filename, data)
        },
        Command::Download(filename) => {
            println!("Downloading file: {}", filename);
            download_file(&config, filename)
        },
        Command::Delete(filename) => {
            println!("Deleting file: {}", filename);
            delete_file(&config, filename)
        },
        Command::Search(query) => {
            println!("Searching for files containing: {}", query);
            search_files(&config, query)
        },
    }
}

fn upload_file(config: &Config, filename: String, data: Vec<u8>) -> io::Result<()> {
    let path = Path::new(&config.storage_path).join(filename);
    let mut file = File::create(path)?;
    file.write_all(&data)?;
    println!("Upload successful.");
    Ok(())
}

fn download_file(config: &Config, filename: String) -> io::Result<()> {
    let path = Path::new(&config.storage_path).join(filename);
    let mut file = File::open(path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    println!("Download successful. Data size: {} bytes", data.len());
    Ok(())
}

fn delete_file(config: &Config, filename: String) -> io::Result<()> {
    let path = Path::new(&config.storage_path).join(filename);
    std::fs::remove_file(path)?;
    println!("File deleted successfully.");
    Ok(())
}

fn search_files(config: &Config, query: String) -> io::Result<()> {
    let mut found = false;
    for entry in std::fs::read_dir(&config.storage_path)? {
        let entry = entry?;
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

fn main() -> io::Result<()> {
    let config = Config::new();
    
    process_command(&config, Command::Upload("example.txt".to_string(), b"Hello World!".to_vec()))?;
    process_command(&config, Command::Download("example.txt".to_string()))?;
    process_command(&config, Command::Delete("example.txt".to_string()))?;
    process_command(&config, Command::Search("example".to_string()))?;

    Ok(())
}