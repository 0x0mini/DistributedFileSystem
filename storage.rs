use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf; 

const BASE_DIR_ENV_KEY: &str = "DFS_BASE_DIR";

struct DistributedFileSystem {
    storage_paths: HashMap<String, String>,
    base_dir: String, 
}

impl DistributedFileSystem {
    fn new() -> Self {
        let base_dir = env::var(BASE_DIR_ENV_KEY).unwrap_or_else(|_| "./data".to_string());
        Self {
            storage_paths: HashMap::new(),
            base_dir, 
        }
    }

    fn store_file(&mut self, file_name: &str, content: &[u8]) -> io::Result<()> {
        let file_path = self.generate_file_path(file_name);
        let mut file = File::create(&file_path)?;
        file.write_all(content)?;
        self.storage_paths.insert(file_name.to_string(), file_path);
        Ok(())
    }

    fn retrieve_file(&self, file_name: &str) -> io::Result<Vec<u8>> {
        if let Some(path) = self.storage_paths.get(file_name) {
            let mut file = File::open(path)?;
            let mut content = Vec::new();
            file.read_to_end(&mut content)?;
            Ok(content)
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "File not found"))
        }
    }

    fn delete_file(&mut self, file_name: &str) -> io::Result<()> {
        if let Some(path) = self.storage_paths.remove(file_name) {
            fs::remove_file(path)?;
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "File not found"))
        }
    }

    fn generate_file_path(&self, file_name: &str) -> String {
        format!("{}/{}", self.base_dir, file_name)
    }
}

fn main() {
    let mut dfs = DistributedFileSystem::new();

    let file_name = "example.txt";
    let content = b"Hello, world!";
    dfs.store_file(file_name, content).unwrap();

    let retrieved_content = dfs.retrieve_file(file_name).unwrap(); 
    println!("Retrieved content: {:?}", String::from_utf8_lossy(&retrieved_content));

    dfs.delete_file(file_name).unwrap();
}