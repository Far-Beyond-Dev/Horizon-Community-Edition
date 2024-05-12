use axum::routing::get;
use socketioxide::{extract::*, socket, SocketIo};
use serde_json::Value;
use serde::{Serialize, Deserialize};
// use tracing::info;


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
    
    io.ns("DBUp", |socket: SocketRef| {
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
    
    io.ns("/", |socket: SocketRef| {
        println!("New client connected!");
        
        socket.on("message", |socket: SocketRef| {
            socket.emit("message-back", "Hello World!").ok();
        });
        
        socket.on("ServerPrintToConsole", || {
            println!("Server console print received from client");
        });
        
        #[derive(Debug, Serialize, Deserialize)]
        struct MyData {
            player_location: String,
        }

        let (_, io) = SocketIo::new_svc();
        io.ns("/", |socket: SocketRef| {
            // Register an async handler for the "test" event and extract the data as a `MyData` struct
            // Extract the binary payload as a `Vec<Bytes>` with the Bin extractor.
            // It should be the last extractor because it consumes the request
            socket.on("test", |socket: SocketRef, Data::<MyData>(data), ack: AckSender, Bin(bin)| async move {
                println!("Received a test message {:?}", data);
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                ack.bin(bin).send(data).ok(); // The data received is sent back to the client through the ack
                socket.emit("test-test", MyData {player_location: "Teste".to_string()}).ok(); // Emit a message to the client
            });
        });

        //s.on("UpdatePlayerLocation", |socket: SocketRef, message: String| {
            //println!("UpdatePlayerLocation: {}", message);
        //});



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
