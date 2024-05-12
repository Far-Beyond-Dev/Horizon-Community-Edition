use axum::routing::get;

use socketioxide::{
    extract::SocketRef, socket::Socket, SocketIo
};
use serde_json::{json, Value};
use serde::{Deserialize, Serialize};
use tracing::info;
use socket_io::MessageHandler;

macro_rules! define_routes {
     ($app:expr, $($path:expr, $handler:expr),* $(,)?) => {
        $(
            $app = $app.route($path, get(|| async { $handler }));
        )*
     };
}

    

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Spin off client and server functions
    println!("+------------------------------------------------------------------------------------------------------------------------------------+");
    println!("|  __    __                      __                                       ______                                                     |");
    println!("| |  |  |  |                    |  |                                     /      |                                                    |");
    println!("| | $$  | $$  ______    ______   |$$ ________   ______   _______        |  $$$$$$|  ______    ______  __     __   ______    ______   |");
    println!("| | $$__| $$ /      |  /      | |  ||        | /      | |       |       | $$___|$$ /      |  /      ||  |   /  | /      |  /      |  |");
    println!("| | $$    $$|  $$$$$$||  $$$$$$|| $$ |$$$$$$$$|  $$$$$$|| $$$$$$$|       |$$    | |  $$$$$$||  $$$$$$||$$| /  $$|  $$$$$$||  $$$$$$| |");
    println!("| | $$$$$$$$| $$  | $$| $$   |$$| $$  /    $$ | $$  | $$| $$  | $$       _|$$$$$$|| $$    $$| $$   |$$ |$$|  $$ | $$    $$| $$   |$$ |");
    println!("| | $$  | $$| $$__/ $$| $$      | $$ /  $$$$_ | $$__/ $$| $$  | $$      |  |__| $$| $$$$$$$$| $$        |$$ $$  | $$$$$$$$| $$       |");
    println!("| | $$  | $$ |$$    $$| $$      | $$|  $$    | |$$    $$| $$  | $$       |$$    $$ |$$     || $$         |$$$    |$$     || $$       |");
    println!("|  |$$   |$$  |$$$$$$  |$$       |$$ |$$$$$$$$  |$$$$$$  |$$   |$$        |$$$$$$   |$$$$$$$ |$$          |$      |$$$$$$$ |$$       |");
    println!("|                                                                 V: 0.0.1-A                                                         |");
    println!("+------------------------------------------------------------------------------------------------------------------------------------+");
    println!("");

    println!("+-----------------------------------------------------------------------------------------+");
    println!("|  ,---.   ,--.                            ,-----.                                   ,--. |");
    println!("| (   .-',-'  '-. ,--,--.,--.--. ,---.     |  |) /_  ,---. ,--. ,--.,---. ,--,--,  ,-|  | |");
    println!("|  `  `-.'-.  .-'| ,-.  ||  .--'(  .-'     |  .-.  || (===) |  '  /| .-. ||  ,,  |' .-. | |");
    println!("|  _)   |  |  |  | '-'  ||  | .-'  `)      |  '--' /|   --.  |   / ' '-' '|  ||  || `-' | |");
    println!("| (____/   `--'   `--`--'`--  `----'       `------'  `----'.-'  /   `---' `--''--' `---'  |");
    println!("|                                    V: 0.0.1-A            `---'                          |");
    println!("+-----------------------------------------------------------------------------------------+");
    println!("");

    tokio::try_join!(
        run_internal_server(),
        run_client_server()
    )?;

    Ok(())
}

async fn run_internal_server() -> Result<(), Box<dyn std::error::Error>> {
    // Set up Socket.IO server for internal traffic on port 3001
    let (layer, io) = SocketIo::new_layer();
    
    io.ns("DBUp", |s: SocketRef| {
        println!("Received update result");
    });

    // Start the internal server
    println!("Starting internal server");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
    println!("Internal server listening on port 3001");
    axum::serve(listener, axum::Router::new().layer(layer)).await?;

    Ok(())
}

async fn run_client_server() -> Result<(), Box<dyn std::error::Error>> {
    // Set up Socket.IO server for clients on port 3000
    let (layer, io) = SocketIo::new_layer();
    
    io.ns("/", |s: SocketRef| {
        println!("New client connected!");
        
        s.on("message", |s: SocketRef| {
            s.emit("message-back", "Hello World!").ok();
        });
        
        s.on("ServerPrintToConsole", || {
            println!("Server console print received from client");
        });

        #[derive(Debug, Serialize, Deserialize)]
        struct MyData {
            player_position: String,
        }
            
        s.on("UpdatePlayerLocation", |s, data: serde_json::Value| {
            // Client Location Updated:
            if let Ok(parsed) = serde_json::from_value::<MyData>(data.clone()) {
                println!("Player location updated to: {}", parsed.player_position);
            } else {
                println!("Failed to parse data into MyData");
            }
            // Example JSON message received over the socket
            let json_message = r#"{ "data": [1.0, 2.0, 3.0] }"#;
        
            // Parse JSON message
            if let Ok(parsed) = serde_json::from_str::<Value>(json_message) {
                if let Some(data) = parsed.get("data") {
                    if let Some(data_array) = data.as_array() {
                        if data_array.len() == 3 {
                            if let (Some(first), Some(second), Some(third)) = (
                                data_array[0].as_f64(),
                                data_array[1].as_f64(),
                                data_array[2].as_f64(),
                            ) {
                                println!("Parsed data: {}, {}, {}", first, second, third);
                            }
                        }
                    }
                }
            }
        }) as MessageHandler<LocalAdapter, _>;
    });

    // Create Axum app for client server
    let mut app = axum::Router::new().layer(layer);
    define_routes!(app, "/", "Hello, World!");

    // Start the client server
    println!("Starting client server");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Client server listening on port 3000");
    axum::serve(listener, app).await?;

    Ok(())
}
