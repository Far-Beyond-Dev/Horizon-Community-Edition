/////////////////////////////////////////////////////////////////////////////////////////////////////
//                                       Horizon Game Server                                       //
//                                                                                                 //
//  This server software is part of a distributed system designed to facilitate communication      //
//  and data transfer between multiple child servers and a master server. Each child server        //
//  operates within a "Region map" managed by the master server, which keeps track of their        //
//  coordinates in a relative cubic light-year space. The coordinates are stored in 64-bit floats  //
//  to avoid coordinate overflow and to ensure high precision.                                     //
//                                                                                                 //
/////////////////////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////
// Use the mimalloc allocator, which boasts excellent performance //
// across a variety of tasks, while being small (8k LOC)          //
////////////////////////////////////////////////////////////////////
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

///////////////////////////////////////////
// Import a few things to get us started //
///////////////////////////////////////////

// Imported some third party crates
use serde_json::Value;
use socketioxide::extract::{Data, SocketRef};
use std::sync::{Arc, Mutex};
use tokio::{main, task::spawn};
use tracing::info;
use viz::{handler::ServiceHandler, serve, Response, Result, Router, Request, Body};

// Import some custom crates from the crates folder in /src
use TerraForge;
use PebbleVault;

//////////////////////////////////////////////////////////////
//                    !!!! WARNING !!!!                     //
// Import all structs (when we have a ton of structs this   //
// will be very bad but should be fine for now)             //
//////////////////////////////////////////////////////////////
use structs::*;

/////////////////////////////////////
// Import the modules we will need //
/////////////////////////////////////

mod events;
mod macros;
mod structs;
mod players;
mod subsystems;

///////////////////////////////////////////////////////////////
//                    !!!! WARNING !!!!                      //
// on_connect runs every time a new player connects to the   //
// server avoid putting memory hungry code here if possible! //
///////////////////////////////////////////////////////////////

/// Handles new player connections to the server.
///
/// This function is called every time a new player connects to the server. It initializes
/// player data, sets up event listeners, and starts necessary subsystems.
///
/// # Arguments
///
/// * `socket` - A reference to the socket connection for the new player.
/// * `data` - Data received with the connection event.
/// * `players` - A thread-safe reference to the collection of all connected players.
///
/// # Warning
///
/// Avoid putting memory-hungry code in this function as it runs for every new connection.
fn on_connect(socket: SocketRef, Data(data): Data<Value>, players: Arc<Mutex<Vec<Player>>>) {
    // Send an optional event to the player that they can hook into to run some post-connection functions
    socket.emit("connected", true).ok();

    // Fetch ID from socket data
    let id = socket.id.as_str();

    // Display join message in log
    println!("Welcome player {} to the game!", id);

    // Authenticate the user
    let player = Player::new(socket.clone(), id.to_string());
    
    // Init the player-related event handlers
    players::init(socket.clone(), players.clone());

    players.lock().unwrap().push(player);

    // Display player join debug messages
    println!("Player {} added to players list", id);
    println!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);

    // Send an optional event to the player that they can hook into to run some post-authentication functions
    socket.emit("auth", true).ok();
         
    ///////////////////////////////////////////////////////////
    //  Setup external event listeners for the more complex  //
    //  systems                                              //
    ///////////////////////////////////////////////////////////
    
    // Initialize extra subsystems to listen to events from the client
    subsystems::core::chat::init(socket.clone(), players.clone());
    subsystems::core::game_logic::init();
    subsystems::core::level_data::init();
    subsystems::core::logging::init();
    subsystems::core::notifications::init();

    // DO NOT INIT SUBSYSTEMS BEYOND THIS POINT
    // Send an optional event to the player that they can hook into to start the game client side
    // This event confirms that the server is fully ready to handle data from the player
    let _ = socket.emit("preplay", true);
    socket.emit("beginplay", true).ok();

}

/// Redirects browser users to the master server dashboard.
///
/// This function handles HTTP GET requests to the root path and redirects
/// the user to the master server's dashboard.
///
/// # Arguments
///
/// * `_req` - The incoming HTTP request (unused in this function).
///
/// # Returns
///
/// A `Result` containing the HTTP response with a 302 redirect status.
async fn redirect_to_master_panel(_req: Request) -> Result<Response> {
    let response = Response::builder()
        .status(302)
        .header("Location", "https://google.com")
        .body(Body::empty())
        .unwrap();
    println!("Someone tried to access this server via a browser, redirecting them to the master dashboard");
    Ok(response)
}

/// The main entry point for the Horizon Game Server.
///
/// This function initializes the server, sets up necessary components,
/// and starts listening for incoming connections.
///
/// # Returns
///
/// A `Result` indicating whether the server started successfully or encountered an error.
#[main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /////////////////////////////
    // SERVER STARTUP SEQUENCE //
    /////////////////////////////

    // Show startup ascii art
    subsystems::core::startup::main();

    // Start the TerraForge thread
    let _terraforge_thread = spawn(async {
        TerraForge::main();
    });
    
    // Start up the database
    PebbleVault::main();

    // Define a place to put new players
    let players: Arc<Mutex<Vec<Player>>> = Arc::new(Mutex::new(Vec::new()));
    let (svc, io) = socketioxide::SocketIo::new_svc();
    let players_clone: Arc<Mutex<Vec<Player>>> = players.clone();

    // Handle new player connections
    io.ns("/", move |socket: SocketRef, data: Data<Value>| {
        println!("Player Connected!");
        on_connect(socket, data, players_clone.clone())
    });
    
    // Create a router to handle incoming connections
    let app = Router::new()
        .get("/", redirect_to_master_panel)
        .any("/*", ServiceHandler::new(svc));

    info!("Starting server");
    
    // Define a listener on port 3000
    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    
    // Start the server and handle any errors
    if let Err(e) = serve(listener, app).await {
        println!("{}", e);
    }
    Ok(())
}