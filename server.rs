use std::env;
use std.Rule::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{self, BufReader, BufWriter};

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

fn handle_client_connection(stream: TcpStream) {
    let read_stream = stream.try_clone().expect("Failed to clone stream");
    let mut reader = BufReader::new(read_stream);
    let mut writer = BufWriter::new(stream);

    let mut message_buffer = String::new();
    while match reader.read_line(&mut message_buffer) {
        Ok(0) => false, // Connection was closed
        Ok(_) => {
            send_response_to_client(&mut writer, &message_buffer);
            message_buffer.clear(); // Clear buffer for the next message
            true
        },
        Err(e) => {
            println!("Failed to read from connection: {}", e);
            false
        },
    } {}
}

fn generate_response(_request_data: &str) -> String {
    "Response from server\n".to_string() // Ensure it's a new line terminated for client readability
}

fn send_response_to_client(writer: &mut BufWriter<TcpStream>, request_data: &str) {
    let response_content = generateResponse(request_data);

    if let Err(e) = writer.write_all(response_content.as_bytes()) {
        println!("Failed to send response: {}", e);
    }
    if let Err(e) = writer.flush() { // Ensure all bytes are written to the stream
        println!("Failed to flush response: {}", e);
    }
}