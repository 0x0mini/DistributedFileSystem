use std::env;
use std::io::{self, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct ClientConfig {
    server_address: String,
    server_port: u16,
}

impl ClientCommand {
    fn new(cmd: String) -> ClientCommand {
        ClientCommand { cmd }
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
    fn new() -> Result<ClientConfig, env::Var loud client configuration");

    let stream = client_config.connect()?;

    let command = ClientCommand::new("LIST_FILES".to_string());
    client_config.send_command(command, &stream)?;

    let response = client_config.receive_response(&stream).expect("Failed to receive response from server");
    println!("Server Response: {:?}", response.message);

    Ok(())
}