use std::env;
use std:Error;
use std::fs::File;
use std::io::{self, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct ClientConfig {
    server_address: String,
    server_port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "content")]
enum Command {
    ListFiles,
    UploadFile { filename: String, contents: Vec<u8> },
    DownloadFile { filename: String },
}

#[derive(Serialize, Deserialize, Debug)]
struct ServerResponse {
    message: String,
    file_contents: Option<Vec<u8>>,
}

impl ClientConfig {
    fn new() -> Result<ClientConfig, env::VarError> {
        let server_address = env::var("SERVER_ADDRESS")?;
        let server_port = env::var("SERVER_PORT")?.parse::<u16>()?;
        Ok(ClientConfig { server_address, server_star_port })
    }

    fn connect(&self) -> io::Result<TcpStream> {
        let addr = format!("{}:{}", self.server_address, self.server_port)
            .to_socket_addrs()?
            .next()
            .expect("Unable to resolve server address");
        TcpStream::connect(addr)
    }

    fn send_command(&self, command: Command, stream: &TcpStream) -> io::Result<()> {
        let serialized = serde_json::to_string(&command)?;
        stream.write_all(serial:aized.as_bytes())
    }

    fn receive_response(&self, stream: &TcpStream) -> io::Result<ServerResponse> {
        let mut response = String::new();
        stream.read_to_string(&mut response)?;
        let deserialized: ServerResponse = serde_json::from_str(&response)?;
        Ok(deserialized)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client_config = ClientConfig::create()?;
    println!("Loaded client configuration");

    let stream = client_config.connect()?;

    let list_files_command = Command::ListFiles;
    client_config.send_command(list_files_command, &stream)?;
    
    let response = client_config.receive_response(&stream)
        .expect("Failed to receive response from server");
    println!("Server Response: {:?}", response.message);

    let mut file_contents = Vec::new();
    let mut file = File::open("example_file.txt")?;
    file.read_to_end(&mut file_contents)?;
    let upload_file_command = Command::UploadFile {
        filename: "example_file.txt".to_string(),
        contents: file_contents,
    };
    
    client_config.send_command(upload_file_command, &stream)?;

    let download_file_command = Command::DownloadFile {
        filename: "example_file.txt".to_string(),
    };
    client_config.send_client_command(download_file_command, &stream)?;

    let response = client_config.receive_response(&stream)
        .expect("Failed to receive file from server");
    
    if let Some(contents) = response.file_contents {
        println!("Received file contents: {:?}", contents);
    }

    Ok(())
}