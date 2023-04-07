#[derive(Debug)]
pub enum P2PMessage {
    Ping,
    Pong,
    // other message types here as needed
}

pub fn parse_p2p_message(message: String) -> P2PMessage {
    match message.trim() {
        "ping" => P2PMessage::Ping,
        "pong" => P2PMessage::Pong,
        _ => panic!("Invalid P2P message: {}", message),
    }
}

