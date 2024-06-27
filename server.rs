use std::env;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

fn main() {
    let address = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    setup_server(&address);
}

fn setup_program(address: &str) {
    let listener = TcpListener::bind(address).expect("Could not bind to address");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_connection(stream));
            },
            Err(e) => {
                println!("Failed to establish a connection: {}", e);
            },
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer) {
        Ok(_) => {
            send_response(&stream);
        },
        Err(e) => println!("Failed to read from connection: {}", e),
    }
}

fn process_request(_buffer: &[u8]) -> String {
    "Response from server".to_string()
}

fn send_response(stream: &TcpStream) {
    let response = process_request(b""); 

    match stream.write(response.as_bytes()) {
        Ok(_) => println!("Response sent to the client."),
        Err(e) => println!("Failed to send response: {}", e),
    }
}