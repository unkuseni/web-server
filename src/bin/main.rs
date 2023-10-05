use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use web_server::ThreadPool;

fn main() {
    // Bind the TCP listener to the specified address
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
let pool = ThreadPool::new(4);
    // Start an infinite loop to handle incoming connections
    for stream in listener.incoming() {
        // Unwrap the incoming stream
        let stream = stream.unwrap();

        // Call the handle_connection function to handle the incoming stream
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

// Function to handle the incoming TCP stream
fn handle_connection(mut stream: TcpStream) {
    // Create a buffer to read the incoming data
    let mut buf = [0; 512];

    // Read the data from the stream into the buffer
    stream.read(&mut buf).unwrap();

    // Define the HTTP GET request
    let get = b"GET / HTTP/1.1\r\n";

    // Check if the received data starts with the GET request
    let (status_line, filename) = if buf.starts_with(get) {
        // If it starts with the GET request, set the status line and filename
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        // If it doesn't start with the GET request, set the status line and filename
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    // Read the contents of the file specified by the filename
    let contents = fs::read_to_string(filename).unwrap();

    // Create the response by combining the status line and file contents
    let response = format!("{}{}", status_line, contents);

    // Write the response to the stream
    stream.write(response.as_bytes()).unwrap();

    // Flush the stream to ensure all data is sent
    stream.flush().unwrap();
}
