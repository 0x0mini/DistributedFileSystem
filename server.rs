use std::env;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

fn main() {
    let server_address = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    start_server(&server_address);
}

fn start_server(address: &str) {
    let server_listener = TcpListener::bind(address).expect("Could not bind to address");
    for connection in server_listener.incoming() {
        match connection {
            Ok(tcp_stream) => {
                thread::spawn(|| handle_client_connection(tcp_stream));
            },
            Err(e) => {
                println!("Failed to establish a connection: {}", e);
            },
        }
    }
}

fn handle_client_connection(mut stream: TcpStream) {
    let mut message_buffer = [0; 1024];

    match stream.read(&mut message_buffer) {
        Ok(_) => {
            send_response_to_client(&stream);
        },
        Err(e) => println!("Failed to read from connection: {}", e),
    }
}

fn generate_response(_request_data: &[u8]) -> String {
    "Response from server".to_string()
}

fn send_response_to_client(stream: &TcpStream) {
    let response_content = generate_response(b"");

    match stream.write(response_content.as_bytes()) {
        Ok(_) => println!("Response sent to the client."),
        Err(e) => println!("Failed to send response: {}", e),
    }
}