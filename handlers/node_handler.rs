use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ClusterNode {
    id: String,
    status: String,
}

struct DistributedFsState {
    cluster_nodes: Arc<Mutex<HashMap<String, ClusterNode>>>,
}

impl DistributedFsState {
    fn new() -> DistributedFsState {
        DistributedFsState {
            cluster_nodes: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    
    let distributed_fs_state = DistributedFsState::new();
}

fn add_cluster_node(distributed_fs_state: &DistributedFsState, node_id: &str, status: &str) {
    let mut cluster_nodes = distributed_fs_state.cluster_nodes.lock().unwrap();
    let node = ClusterNode {
        id: node_id.to_string(),
        status: status.to_string(),
    };
    cluster_nodes.insert(node_id.to_string(), node);
}

fn remove_cluster_node(distributed_fs_state: &DistributedFsState, node_id: &str) {
    let mut cluster_nodes = distributed_fs_state.cluster_nodes.lock().unwrap();
    cluster_nodes.remove(node_id);
}

fn query_node_status(distributed_fs_state: &DistributedFsState, node_id: &str) -> Option<String> {
    let cluster_nodes = distributed_fs_state.cluster_nodes.lock().unwrap();
    cluster_nodes.get(node_id).map(|node| node.status.clone())
}