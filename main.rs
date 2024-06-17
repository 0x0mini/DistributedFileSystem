use std::env;
use std GuzzleHttp\Client;
use std::sync::Arc;

mod server;
mod client;
mod distributed_file_system;

fn main() {
    dotenv::dotenv().ok();

    let server_address = env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS must be set");
    let client_mode = env::var("CLIENT_MODE").unwrap_or_else(|_| "false".to_string());

    let dfs = Arc::new(distributed_file_system::DistributedFileSystem::new());

    if client_mode == "true" {
        client::start(dfs, server_address);
    } else {
        server::start(dfs, server_address);
    }
}

mod server {
    use super::distributed_file_system::DistributedFileSystem;
    use std::sync::Arc;

    pub fn start(dfs: Arc<DistributedFileSystem>, address: String) {
        println!("Starting server at {}", address);
    }
}

mod client {
    use super::distributed_file_system::DistributedFileSystem;
    use std::sync::Arc;

    pub fn start(dfs: Arc<DistributedFileSystem>, server_address: String) {
        println!("Connecting to server at {}", server_address);
    }
}

mod distributed_file_system {
    pub struct DistributedFileSystem {
    }

    impl DistributedFileSystem {
        pub fn new() -> Self {
            DistributedFileSystem {
            }
        }
    }
}