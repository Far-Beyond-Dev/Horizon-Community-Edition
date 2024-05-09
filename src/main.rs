use axum::routing::get;
use socketioxide::{
    extract::SocketRef,
    SocketIo,
};
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

// +---------------------------------+
// | Pretty-Print Server information |
// +---------------------------------+

    println!("+-----------------------------------------------------------------------------------------+");
    println!("|  ,---.   ,--.                            ,-----.                                   ,--. |");
    println!("| (   .-',-'  '-. ,--,--.,--.--. ,---.     |  |) /_  ,---. ,--. ,--.,---. ,--,--,  ,-|  | |");
    println!("|  `  `-.'-.  .-'| ,-.  ||  .--'(  .-'     |  .-.  || .-. : |  '  /| .-. ||      |' .-. | |");
    println!("|  _)   |  |  |  | '-'  ||  | .-'  `)      |  '--' /|   --.  |   ' ' '-' '|  ||  || `-' | |");
    println!("| (____/   `--'   `--`--'`--  `----'       `------'  `----'.-'  /   `---' `--''--' `---'  |");
    println!("|                                    V: 0.0.1-A            `---'                          |");
    println!("+-----------------------------------------------------------------------------------------+");
    println!("");
    

// +------------------------------+
// | GOLANG Socket Event Handlers |
// +------------------------------+

//    // Emit a "update" event with transaction data to the Go server
//    let tx_data = "Transaction data goes here";
//    // socket.emit("update", &tx_data).await.expect("Failed to emit event");
//
//    let (layer, io) = SocketIo::new_layer();
//    // Connect to the Socket.IO server running on Go
//    let socket = ClientBuilder::new("http://localhost:3001")
//        .connect()
//        .expect("Failed to connect to server");
//
//    // Handle incoming events from the GOLANG server (if any)
//    // socket.on("updateResult", |res| {
//    //     println!("Received update result: {:?}", res);
//    // });


// +------------------------------+
// |  UE5 Socket Event Handlers   |
// +------------------------------+

    let (layer, io) = SocketIo::new_layer();
    // Register a handler for the default namespace
    io.ns("/", |s: SocketRef| {
        
        // For each "message" event received, send a "message-back" event with the "Hello World!" event
        s.on("message", |s: SocketRef| {
            s.emit("message-back", "Hello World!").ok();
        });
        
        // An event for printing data to the server console from client
        s.on("ServerPrintToConsole", || {
            println!("Server console print recieved from client");
        });
    });

    let mut app = axum::Router::new()
    .layer(layer);


// +--------------------------------+
// | Setup server home page in http |
// +--------------------------------+

    define_routes!(app, "/", "Hello, World!");

    println!("Starting server");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Stars Beyond dedicated server listening on all interfaces (0.0.0.0) via port 3000");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}