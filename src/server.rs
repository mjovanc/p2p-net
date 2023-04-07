use std::io::Read;
use std::net::{TcpListener, TcpStream};

pub(crate) fn start() {
    println!("Starting server...");

    const HOST : &str ="127.0.0.1";
    const PORT : &str ="8477";

    // concatenating host address and port to create final endpoint
    let end_point : String = HOST.to_owned() + ":" +  PORT;

    // creating tcp listener at our end point
    let listener = TcpListener::bind(end_point).unwrap();
    println!("Web server is listening at port {}",PORT);

    // conneting to any incoming connections
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        // call function to process any incomming connections
        handle_connection(_stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}