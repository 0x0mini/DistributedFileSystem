use std::env;
use std::net::TcpStream;
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct ClientConfig {
    server_address: String,
    server_port: u16,
}

impl ClientCommand {
    fn new(cmd: String) -> ClientCommand {
        ClientCommand{ cmd }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ClientCommand {
    cmd: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ServerResponse {
    message: String,
}

impl ClientConfig {
    fn new() -> ClientConfig {
        let server_address = env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS not set in .env");
        let server_port: u16 = env::var("SERVER_PORT")
            .expect("SERVER_PORT not set in .env")
            .parse()
            .expect("SERVER_PORT must be a valid integer");

        ClientConfig { server_address, server_port }
    }

    fn connect(&self) -> TcpStream {
        let connection_string = format!("{}:{}", self.server_address, self.server_port);
        TcpStream::connect(connection_string)
            .expect("Failed to connect to the server")
    }

    fn send_command(&self, command: ClientCommand, mut stream: &TcpStream) {
        let serialized = serde_json::to_string(&command).unwrap();
        stream.write(serialized.as_bytes())
            .expect("Failed to write to server stream");
    }

    fn receive_response(&self, mut stream: &TcpStream) -> ServerResponse {
        let mut buffer = Vec::new();
        stream.read_to_end(&mut buffer).expect("Failed to read from server stream");
        let response: ServerResponse = serde_json::from_slice(&buffer).expect("Failed to parse server response");
        response
    }
}

fn main() {
    let client_config = ClientConfig::new();
    let stream = client_config.connect();

    let command = ClientCommand::new("LIST_FILES".to_string());
    client_config.send_command(command, &stream);

    let response = client_process.receive_response(&stream);
    println!("Server Response: {:?}", response.message);
}