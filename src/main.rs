use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

// Define the server address
const SERVER: &str = "127.0.0.1:8080";

// Function to handle an incoming client connection
fn handle_client(mut stream: TcpStream) {
    // Create a buffer to store received data
    let mut buffer = [0; 1024];

    // Read data from the client into the buffer
    stream
        .read(&mut buffer)
        .expect("Failed to read from client");

    // Convert the received data to a String
    let request = String::from_utf8_lossy(&buffer[..]);

    // Print the received request
    println!("Request: {}", request);

    // Prepare a response to send back to the client
    let response = "Nice to hear from you!".as_bytes();

    // Write the response to the client
    stream
        .write(response)
        .expect("Failed to write the response");

    // Print a message indicating that the communication is completed
    println!("Completed");
}

fn main() {
    // Bind the server to the specified address
    let listener = TcpListener::bind(SERVER).expect("Failed to bind address");

    // Print a message indicating that the server is listening
    println!("Server listening at {}", SERVER);

    println!(" listner incoming{:?}", listener.incoming());

    // Accept incoming connections and spawn a new thread for each connection
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a new thread to handle the client connection
                std::thread::spawn(|| handle_client(stream));
            }
            Err(err) => {
                // Print an error message if accepting the connection fails
                eprint!("Error occurred: {}", err);
            }
        }
    }
}
