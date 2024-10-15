use serde_json::{json, Value};
use socketioxide::extract::{SocketRef, Data};
use std::sync::{Arc, Mutex};
use serde::Deserialize;
use tracing::{debug, info};
use horizon_data_types::*;

#[derive(Debug, Deserialize)]
struct WhisperData {
    recipient: String,
    message: String,
}

pub fn init(socket: SocketRef, players: Arc<Mutex<Vec<Player>>>) {
    socket.on("whisper", |socket: SocketRef, Data(data): Data<Value>| async move {
        let whisper_data: WhisperData = match serde_json::from_value(data) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Error parsing whisper data: {}", e);
                return;
            }
        };
        handle_whisper(socket, &whisper_data.recipient, &whisper_data.message);
    });

    socket.on("broadcast", |socket: SocketRef, Data(data): Data<Value>| async move {
        let message = match data.as_str() {
            Some(msg) => msg,
            None => {
                eprintln!("Invalid broadcast message format");
                return;
            }
        };
        handle_broadcast(socket, message);
    });

    socket.on("help", |socket: SocketRef, _: Data<Value>| async move {
        handle_help(socket);
    });

    info!("Starting chat subsystem...");
}

fn handle_whisper(socket: SocketRef, recipient: &str, message: &str) {
    info!("Whisper to {}: {}", recipient, message);
    
    // In a real application, you'd want to find the recipient's socket and send only to them
    // For this example, we'll send it back to the sender (simulating the recipient)
    let whisper_message = json!({
        "sender": socket.id,
        "message": message
    });
    socket.emit("whisper", whisper_message).ok();
}

fn handle_broadcast(socket: SocketRef, message: &str) {
    info!("Broadcast: {}", message);
    
    let broadcast_message = json!({
        "sender": socket.id,
        "message": message
    });
    socket.broadcast().emit("broadcast", broadcast_message).ok();
}

fn handle_help(socket: SocketRef) {
    info!("Help requested by {}", socket.id);
    
    let help_message = "Available commands:\n\
                        /whisper <recipient> <message> - Send a private message\n\
                        /broadcast <message> - Send a message to all users\n\
                        /help - Show this help message";
    
    socket.emit("help", help_message).ok();
}