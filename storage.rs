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

    fn store_files(&mut self, files: &[(String, Vec<u8>)]) -> io::Result<()> {
        for (file_name, content) in files {
            let file_path = self.generate_file_path(file_name);
            let mut file = File::create(&file_path)?;
            file.write_all(content)?;
            self.storage_paths.insert(file_name.to_string(), file_path);
        }
        Ok(())
    }

    fn retrieve_files(&self, file_names: &[String]) -> io::Result<HashMap<String, Vec<u8>>> {
        let mut contents = HashMap::new();
        for file_name in file_names {
            if let Some(path) = self.storage_paths.get(file_name) {
                let mut file = File::open(path)?;
                let mut content = Vec::new();
                file.read_to_end(&mut content)?;
                contents.insert(file_name.clone(), content);
            } else {
                return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
            }
        }
        Ok(contents)
    }

    fn delete_files(&mut self, file_names: &[String]) -> io::Result<()> {
        for file_name in file_names {
            if let Some(path) = self.storage_paths.remove(file_name) {
                fs::remove_file(path)?;
            } else {
                return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
            }
        }
        Ok(())
    }

    fn generate_file_path(&self, file_name: &str) -> String {
        format!("{}/{}", self.base_dir, file_name)
    }
}

fn main() {
    let mut dfs = DistributedFileSystem::new();

    // Example of batching storage operations
    let files_to_store = vec![
        ("example.txt".to_string(), b"Hello, world!".to_vec()),
        ("another_file.txt".to_string(), b"Content of another file.".to_vec()),
    ];
    dfs.store_files(&files_to_store).unwrap();

    // Example of batched retrieval
    let file_names = files_to_store.into_iter().map(|(name, _)| name).collect::<Vec<_>>();
    let retrieved_contents = dfs.retrieve_files(&file_names).unwrap();
    for (name, content) in retrieved_contents {
        println!("Retrieved content for {}: {:?}", name, String::from_utf8_lossy(&content));
    }

    // Example of batched delete
    dfs.delete_files(&file_names).unwrap();
}