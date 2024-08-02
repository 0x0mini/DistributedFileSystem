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
        Command::Upload(filename, data) => upload_file(&config, filename, data),
        Command::Download(filename) => download_file(&config, filename),
        Command::Delete(filename) => delete_file(&config, filename),
        Command::Search(query) => search_files(&config, query),
    }
}

fn upload_file(config: &Config, filename: String, data: Vec<u8>) -> io::Result<()> {
    let path = Path::new(&config.storage_path).join(filename);
    let mut file = File::create(path)?;
    file.write_all(&data)?;
    Ok(())
}

fn download_file(config: &Config, filename: String) -> io::Result<()> {
    let path = Path::new(&config.storage_path).join(filename);
    let mut file = File::open(path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    println!("Downloaded data: {:?}", data);
    Ok(())
}

fn delete_file(config: &Config, filename: String) -> io::Result<()> {
    let path = Path::new(&config.storage_path).join(filename);
    std::fs::remove_file(path)?;
    Ok(())
}

fn search_files(config: &Config, query: String) -> io::Result<()> {
    for entry in std::fs::read_dir(&config.storage_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.to_string_lossy().contains(&query) {
            println!("Found: {}", path.display());
        }
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