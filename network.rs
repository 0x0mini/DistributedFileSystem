use std::collections::HashMap;
use std::env;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use serde::{Deserialize, Serialize};

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

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;

    if bytes_read == 0 {
        return Ok(());
    }

    let received_msg: Message = serde_json::from_slice(&buffer[..bytes_read]).unwrap();

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

fn connect_to_peer(peer_address: &str) -> io::Result<()> {
    let mut stream = TcpStream::connect(peer_address)?;
    let msg = Message {
        msg_type: MessageType::Hello,
        sender: env::var("PEER_ID").unwrap_or_else(|_| "Unknown".into()),
        content: "Hello there!".into(),
    };

    let serialized_msg = serde_json::to_vec(&msg).unwrap();
    stream.write_all(&serialized_msg)?;

    Ok(())
}

fn start_server() -> io::Result<()> {
    let listener = TcpListener::bind(env::var("LISTEN_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".into()))?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream)?;
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