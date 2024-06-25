use socketioxide::extract::{SocketRef, Data};
use serde::Deserialize;
use tracing::info;

#[derive(Deserialize)]
struct ChatMessage {
    sender: String,
    recipient: Option<String>,
    message: String,
}

pub fn init(socket: SocketRef) {
    socket.on("newChat", |socket: SocketRef, Data(data): Data<String>| {
        info!("Received event: newChat with data: {:?}", data);

        // Parse the incoming chat message
        let chat_message: Result<ChatMessage, _> = serde_json::from_str(&data);
        match chat_message {
            Ok(message) => {
                // Handle the chat message
                if let Some(recipient) = message.recipient {
                    handle_whisper(socket.clone(), &message.sender, &recipient, &message.message);
                } else if message.message.trim().eq_ignore_ascii_case("help") {
                    handle_help(socket.clone(), &message.sender);
                } else {
                    handle_broadcast(socket.clone(), &message.sender, &message.message);
                }
            }
            Err(err) => {
                info!("Failed to parse chat message: {:?}", err);
            }
        }
    });
}

fn handle_whisper(socket: SocketRef, sender: &str, recipient: &str, message: &str) {
    info!("Whisper from {} to {}: {}", sender, recipient, message);
    // Example of emitting a message to a specific recipient
    socket.emit("whisper", (recipient, message)).ok();
}

fn handle_help(socket: SocketRef, sender: &str) {
    info!("Help requested by {}", sender);
    // Example of emitting a help message to the sender
    socket.emit("help", "Here's some help!").ok();
}

fn handle_broadcast(socket: SocketRef, sender: &str, message: &str) {
    info!("Broadcast from {}: {}", sender, message);
    // Example of broadcasting a message to all connected clients
    socket.broadcast().emit("broadcast", message).ok();
}