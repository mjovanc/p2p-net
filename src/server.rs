use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_p2p_connection(mut stream: TcpStream) {
    println!("New P2P connection from {}", stream.peer_addr().unwrap());

    let mut buf = [0; 1024];
    match stream.read(&mut buf) {
        Ok(n) if n > 0 => {
            let message = String::from_utf8_lossy(&buf[..n]);
            match message.trim() {
                "ping" => {
                    println!("Received ping from {}", stream.peer_addr().unwrap());
                    // Send a pong message back to the peer
                    let pong = "pong\n".as_bytes();
                    stream.write_all(pong).unwrap();
                },
                _ => {
                    println!("Invalid message from {}: {:?}", stream.peer_addr().unwrap(), message);
                }
            }
        },
        Ok(_) => {
            println!("Empty message from {}", stream.peer_addr().unwrap());
        },
        Err(e) => {
            println!("Error reading from {}: {}", stream.peer_addr().unwrap(), e);
        }
    }
}

pub(crate) fn start_p2p_server() {
    println!("Starting P2P server...");

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("P2P server is listening at port 7878");

    // listen for incoming P2P connections indefinitely
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // spawn a new thread to handle the incoming P2P connection
                std::thread::spawn(|| {
                    handle_p2p_connection(stream);
                });
            },
            Err(e) => {
                println!("Error accepting P2P connection: {}", e);
            }
        }
    }
}
