use std::collections::HashMap;
use std::env;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeError;

#[derive(Serialize, Deserialize, Debug)]
enum MessageType {
    Hello,
    Goodbye,
    DataTransfer,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    msg_type: MessageType,
    sender: String,
    content: String,
}

struct NetworkTopology {
    peers: HashMap<String, String>,
}

impl NetworkTopology {
    fn new() -> NetworkTopology {
        NetworkTopology {
            peers: HashMap::new(),
        }
    }

    fn add_peer(&mut self, id: String, address: String) {
        self.peers.insert(id, address);
    }

    fn remove_peer(&mut self, id: &String) {
        self.peers.remove(id);
    }
}

fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;

    if bytes_read == 0 {
        return Ok(());
    }

    let received_msg: Message = serde_json::from_slice(&buffer[..bytes_read])
        .map_err(|e| format!("Failed to deserialize received message: {}", e))?;

    println!("Received: {:?}", received_msg);

    match received_msg.msg_type {
        MessageType::Hello => {
            println!("Hello from {}", received_msg.sender);
        },
        MessageType::Goodbye => {
            println!("Goodbye from {}", received_msg.sender);
        },
        MessageType::DataTransfer => {
            println!("Data transfer from {}: {}", received_msg.sender, received_msg.content);
        },
    }

    Ok(())
}

fn connect_to_peer(peer_address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(peer_address)?;

    let msg = Message {
        msg_type: MessageType::Hello,
        sender: env::var("PEER_ID").unwrap_or_else(|_| "Unknown".into()),
        content: "Hello there!".into(),
    };

    let serialized_msg = serde_json::to_vec(&msg)
        .map_err(|e| format!("Failed to serialize message: {}", e))?;
    stream.write_all(&serialized_msg)?;

    Ok(())
}

fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let bind_address = env::var("LISTEN_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".into());
    let listener = TcpListener::bind(&bind_address)
        .map_err(|e| format!("Failed to bind to {}: {}", bind_address, e))?;

    println!("Server listening on {}", bind_address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = handle_client(stream) {
                    println!("Error handling client: {}", e);
                }
            }
            Err(e) => { println!("Failed to receive connection: {}", e); }
        }
    }
    Ok(())
}

fn main() {
    let network_topology = NetworkTopology::new();

    if let Err(e) = start_server() {
        println!("Failed to start the server: {}", e);
    }
}