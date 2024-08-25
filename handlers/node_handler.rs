use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Node {
    id: String,
    status: String,
}

struct AppState {
    nodes: Arc<Mutex<HashMap<String, Node>>>,
}

impl AppState {
    fn new() -> AppState {
        AppState {
            nodes: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    
    let app_state = AppState::new();
}

fn add_node(app_state: &AppState, node_id: &str, status: &str) {
    let mut nodes = app_state.nodes.lock().unwrap();
    let node = Node {
        id: node_id.to_string(),
        status: status.to_string(),
    };
    nodes.insert(node_id.to_string(), node);
}

fn remove_node(app_state: &AppState, node_id: &str) {
    let mut nodes = app_state.nodes.lock().unwrap();
    nodes.remove(node_id);
}

fn query_node_status(app_state: &AppState, node_id: &str) -> Option<String> {
    let nodes = app_state.nodes.lock().unwrap();
    nodes.get(node_id).map(|node| node.status.clone())
}