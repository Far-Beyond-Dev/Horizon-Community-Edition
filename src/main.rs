use axum::routing::get;
use socketioxide::{
    extract::SocketRef,
    SocketIo,
};
use rust_socketio::{ClientBuilder, Payload, RawClient};

use tracing::info;

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
    println!("+-----------------------------------------------------------------------------------------+");
    println!("|  ,---.   ,--.                            ,-----.                                   ,--. |");
    println!("| (   .-',-'  '-. ,--,--.,--.--. ,---.     |  |) /_  ,---. ,--. ,--.,---. ,--,--,  ,-|  | |");
    println!("|  `  `-.'-.  .-'| ,-.  ||  .--'(  .-'     |  .-.  || (===) |  '  /| .-. ||  ,,  |' .-. | |");
    println!("|  _)   |  |  |  | '-'  ||  | .-'  `)      |  '--' /|   --.  |   / ' '-' '|  ||  || `-' | |");
    println!("| (____/   `--'   `--`--'`--  `----'       `------'  `----'.-'  /   `---' `--''--' `---'  |");
    println!("|                                    V: 0.0.1-A            `---'                          |");
    println!("+-----------------------------------------------------------------------------------------+");
    println!("");

    tokio::try_join!(/*run_client(),*/ run_server())?;

    Ok(())
}

    async fn run_client() -> Result<(), Box<dyn std::error::Error>> {
        let socket = ClientBuilder::new("http://localhost:4200")
        .namespace("/admin")
        .on("test", |message, _| eprintln!("Error: {:#?}", message))
        .on("error", |err, _| eprintln!("Error: {:#?}", err))
        .connect()
        .expect("Connection failed");

        Ok(())
    }

async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    // Set up Socket.IO server
    let (layer, io) = SocketIo::new_layer();
    // Register a handler for the default namespace
    io.ns("/", |s: SocketRef| {
        // Connection message
        println!("New client connected!");
        
        // For each "message" event received, send a "message-back" event with the "Hello World!" event
        s.on("message", |s: SocketRef| {
            s.emit("message-back", "Hello World!").ok();
        });
        
        // An event for printing data to the server console from client
        s.on("ServerPrintToConsole", || {
            println!("Server console print received from client");
        });
    });

    // Create Axum app
    let mut app = axum::Router::new().layer(layer);
    // Setup server home page in HTTP
    define_routes!(app, "/", "Hello, World!");

    // Start the server
    println!("Starting server");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Stars Beyond dedicated server listening on all interfaces (0.0.0.0) via port 3000");
    axum::serve(listener, app).await?;

    Ok(())
}
