use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);

    // Extract the requested path from the first line of the request
    let path = request.lines().next().unwrap().split_whitespace().nth(1).unwrap();

    // Build the response
    let response = format!("HTTP/1.1 200 OK\r\n\r\nRequested path: {}\r\n", path);

    // Send the response back to the client
    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();
    println!("Server listening on http://127.0.0.1:80");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a new thread or use an async runtime to handle each incoming connection
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}
