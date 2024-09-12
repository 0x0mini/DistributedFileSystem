use std::collections::HashMap;
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
    fn new() -> Self {
        Self {
            cluster_nodes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn add_cluster_node(&self, node_id: &str, status: &str) {
        let node = ClusterNode {
            id: node_id.to_string(),
            status: status.to_string(),
        };
        let mut cluster_nodes = self.cluster_nodes.lock().unwrap();
        cluster_nodes.insert(node_id.to_string(), node);
    }

    fn remove_cluster_node(&self, node_id: &str) {
        let mut cluster_nodes = self.cluster_nodes.lock().unwrap();
        cluster_nodes.remove(node_id);
    }

    fn query_node_status(&self, node_id: &str) -> Option<String> {
        let cluster_nodes = self.cluster_nodes.lock().unwrap();
        cluster_nodes.get(node_id).map(|node| node.status.clone())
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let distributed_fs_state = DistributedFsState::new();

    distributed_fs_state.add_cluster_node("node1", "active");
    distributed_fs_state.add_cluster_node("node2", "inactive");
    println!("{:?}", distributed_fs_state.query_node_status("node1")); // Some("active")
    distributed_fs_state.remove_cluster_node("node2");
}