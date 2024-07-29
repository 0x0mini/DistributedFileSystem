use std::collections::HashMap;
use std::env;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeError;
use std::fmt;

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

#[derive(Debug)]
enum MyError {
    Io(io::Error),
    EnvVar(env::VarError),
    Serde(SerdeError),
    Custom(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MyError::Io(ref err) => write!(f, "IO error: {}", err),
            MyError::EnvVar(ref err) => write!(f, "Environment variable error: {}", err),
            MyError::Serde(ref err) => write!(f, "Serialization/Deserialization error: {}", err),
            MyError::Custom(ref err) => write!(f, "Custom error: {}", err),
        }
    }
}

impl From<io::Error> for MyError {
    fn from(error: io::Error) -> MyError {
        MyError::Io(error)
    }
}

impl From<SerdeError> for MyError {
    fn from(error: SerdeError) -> MyError {
        MyError::Serde(error)
    }
}

impl From<env::VarError> for MyError {
    fn from(error: env::VarError) -> MyError {
        MyError::EnvVar(error)
    }
}

fn handle_client(mut stream: TcpStream) -> Result<(), MyError> {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;

    if bytes_read == 0 {
        return Ok(());
    }

    let received_msg: Message = serde_json::from_slice(&buffer[..bytes_read])
        .map_err(MyError::Serde)?;

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

fn connect_to_peer(peer_address: &str) -> Result<(), MyError> {
    let mut stream = TcpStream::connect(peer_address)?;

    let msg = Message {
        msg_type: MessageType::Hello,
        sender: env::var("PEER_ID")?,
        content: "Hello there!".into(),
    };

    let serialized_msg = serde_json::to_vec(&msg)?;
    stream.write_all(&serialized_msg)?;

    Ok(())
}

fn start_server() -> Result<(), MyError> {
    let bind_address = env::var("LISTEN_ADDR")?;
    let listener = TcpListener::bind(&bind_address)?;

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