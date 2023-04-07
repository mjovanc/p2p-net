use std::io::{Read, Write};
use std::net::TcpStream;

pub(crate) fn send_p2p_message(message: String) {
    println!("Sending P2P message: {}", message);

    let mut stream = match TcpStream::connect("127.0.0.1:7878") {
        Ok(stream) => stream,
        Err(e) => {
            println!("Error connecting to P2P server: {}", e);
            return;
        }
    };

    let p2p_message = format!("{}\n", message);
    let bytes = p2p_message.as_bytes();
    match stream.write_all(&bytes) {
        Ok(_) => {
            println!("P2P message sent successfully");
            let mut buf = [0; 1024];
            match stream.read(&mut buf) {
                Ok(n) if n > 0 => {
                    let response = String::from_utf8_lossy(&buf[..n]);
                    println!("P2P server response: {}", response);
                },
                Ok(_) => {
                    println!("Empty response from P2P server");
                },
                Err(e) => {
                    println!("Error reading from P2P server: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Error sending P2P message: {}", e);
        }
    }
}
