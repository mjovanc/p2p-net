mod server;
mod client;

fn main() {
    server::start();

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        client::send_message(input);
    }
}
