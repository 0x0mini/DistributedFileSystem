use std::env;
use std::io::{self, BufReader, BufWriter, Write, Read};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let server_address = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    start_server(&server_address);
}

fn start_server(address: &str) {
    let server_listener = TcpListener::bind(address).expect("Could not bind to address");
    println!("Server running on {}", address);
    for connection in server_listener.incoming() {
        match connection {
            Ok(tcp_stream) => {
                thread::spawn(|| handle_client_connection(tcp_stream));
            },
            Err(e) => {
                eprintln!("Failed to establish a connection: {}", e);
            },
        }
    }
}

fn handle_client_connection(stream: TcpStream) {
    let read_stream = stream.try_clone().expect("Failed to clone stream");
    let mut reader = BufReader::new(read_stream);
    let mut writer = BufWriter::new(stream);

    let mut message_buffer = String::new();
    loop {
        match reader.read_to_string(&mut message_buffer) {
            Ok(0) => break, // Connection was closed
            Ok(_) => {
                send_response_to_client(&mut writer, &message_buffer);
                message_buffer.clear(); // Clear buffer to avoid sending duplicate responses
                break; // Assuming one message per connection; remove if you want to keep the connection open
            },
            Err(e) => {
                eprintln!("Failed to read from connection: {}", e);
                break;
            },
        }
    }
}

fn generate_response(_request_data: &str) -> String {
    "Response from server\n".to_string() // Ensure it's a new line terminated for client readability
}

fn send_response_to_client(writer: &mut BufWriter<TcpStream>, request_data: &str) {
    let response_content = generate_response(request_data);

    if let Err(e) = writer.write_all(response_content.as_bytes()) {
        eprintln!("Failed to send response: {}", e);
    }
    if let Err(e) = writer.flush() { // Ensure all bytes are written to the stream
        eprintln!("Failed to flush response: {}", e);
    }
}