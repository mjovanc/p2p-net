mod server;
mod client;
mod message;

fn main() {
    // start the P2P server in a separate thread
    std::thread::spawn(|| {
        server::start_p2p_server();
    });

    loop {
        // Read user input from the command line
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        // Send the user's input as a P2P message
        client::send_p2p_message(input.trim().to_string());
    }
}