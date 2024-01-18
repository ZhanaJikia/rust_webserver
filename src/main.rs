use std::net::{TcpListener, TcpStream};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request:\n{}", request);

    // Extract the requested path from the first line of the request
    let path = request.lines().next().unwrap().split_whitespace().nth(1).unwrap();

    // Build the response
    let response = if path == "/" {
        serve_file("www/index.html")
    } else {
        let file_path = format!("www/{}", &path[1..]); // Remove leading '/'
        serve_file(&file_path)
    };

    // Send the response back to the client
    stream.write_all(response.as_bytes()).unwrap();
}

fn serve_file(file_path: &str) -> String {
    let path = Path::new(file_path);
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => return format!("HTTP/1.1 404 Not Found\r\n\r\nFile Not Found"),
    };

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    format!("HTTP/1.1 200 OK\r\n\r\n{}", content)
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
