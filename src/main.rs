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
#[cfg(target_os = "linux")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

///////////////////////////////////////////
// Import a few things to get us started //
///////////////////////////////////////////

// Import some third party crates
use colored::Colorize;
use horizon_data_types::*;
use serde_json::Value;
use socketioxide::extract::{Data, SocketRef};
use std::sync::{Arc, Mutex};
use tokio::{main, task::spawn};
use tracing::info;
use viz::{handler::ServiceHandler, serve, Body, Request, Response, Result, Router};
use uuid::Uuid;

// Load the plugins API
use plugin_test_api as plugin_api;
use plugin_test_plugins as plugins;

use PebbleVault;

//////////////////////////////////////////////////////////////
//                    !!!! WARNING !!!!                     //
// Import all structs (when we have a ton of structs this   //
// will be very bad but should be fine for now)             //
//////////////////////////////////////////////////////////////

/////////////////////////////////////
// Import the modules we will need //
/////////////////////////////////////

mod macros;
mod players;
mod plugin_manager;

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
async fn on_connect(socket: SocketRef, Data(data): Data<Value>, players: Arc<Mutex<Vec<Player>>>) {
    // Send an optional event to the player that they can hook into to run some post-connection functions
    // socket.emit("connected", true).ok(); TODO: Fix this data param

    // Fetch ID from socket data
    let id = socket.id.as_str();

    let all_plugins = plugins::plugins();
    
    // Display join message in log
    println!("Welcome player {} to the game!", id);
    
    // Authenticate the user
    let player = Player::new(socket.clone(),Uuid::new_v4());
    
    // Init the player-related event handlers
    players::init(socket.clone(), players.clone());
    
    
    for (ref name, ref plugin) in all_plugins.list.iter() {
        plugin.broadcast_game_event(&&plugin.get_plugin(), plugin_api::GameEvent::PlayerJoined((player.clone())));
    }

    players.lock().unwrap().push(player);

    // Display player join debug messages
    println!("Player {} added to players list", id);
    println!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);

    // Send an optional event to the player that they can hook into to run some post-authentication functions
    //  socket.emit("auth", true).ok();  TODO: Fix this

    ///////////////////////////////////////////////////////////
    //  Setup external event listeners for the more complex  //
    //  systems                                              //
    ///////////////////////////////////////////////////////////

    // DO NOT INIT SUBSYSTEMS BEYOND THIS POINT
    // Send an optional event to the player that they can hook into to start the game client side
    // This event confirms that the server is fully ready to handle data from the player

    // let _ = socket.emit("preplay", true);   TODO: Fix this
    // socket.emit("beginplay", true).ok();    TODO: Fix this
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
        .header("Location", "https://youtu.be/dQw4w9WgXcQ")
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

    let all_plugins = plugins::plugins();

    println!("Your Horizon plugins greet you!");
    for (ref name, ref plugin) in all_plugins.list.iter() {
        let instance = plugin.get_instance();
        println!("\t{}: \"{}\"", name, (*instance).say_hello());
    }

    // Start the plugin Manager thread
    let mut _plugin_manager = spawn(async {
        let mut manager = plugin_manager::PluginManager::new();

        // manager.load_plugins_from_directory("./plugins/").is_err() {
        //     println!("Error: Failed to load plugins from dir");
        // }

        let rx = manager
            .monitor_directory_for_changes("./plugins")
            .expect("Failed to monitor directory");

        let manager_ref = Arc::new(Mutex::new(manager));
        let manager_handle = Arc::clone(&manager_ref);
    });

    // Start the PebbleVault thread
    //   let pebble_vault_thread = tokio::spawn(async move {
    //       // Run the initial tests
    //       if let Err(e) = PebbleVault::tests::run_tests() {
    //           eprintln!("Error running initial PebbleVault tests: {}", e);
    //       }
    //   
    //       // Set up parameters for the load tests
    //       let db_path = "load_test.db";
    //       let num_objects = 10_000;
    //       let num_regions = 5;
    //       let num_operations = 3;
    //       let interval = std::time::Duration::from_secs(300); // Run every 5 minutes
    //   
    //       loop {
    //           // Run the regular load test
    //           println!("\n{}", "Running regular load test".blue());
    //           match PebbleVault::VaultManager::<PebbleVault::load_test::LoadTestData>::new(db_path) {
    //               Ok(mut vault_manager) => {
    //                   if let Err(e) = PebbleVault::load_test::run_load_test(
    //                       &mut vault_manager,
    //                       num_objects,
    //                       num_regions,
    //                       num_operations,
    //                   ) {
    //                       eprintln!("Error in regular load test: {}", e);
    //                   } else {
    //                       println!("{}", "Regular load test completed successfully".green());
    //                   }
    //               }
    //               Err(e) => eprintln!("Error creating VaultManager for regular load test: {}", e),
    //           }
    //   
    //           // Run the arbitrary data load test
    //           println!("\n{}", "Running arbitrary data load test".blue());
    //           if let Err(e) =
    //               PebbleVault::load_test::run_arbitrary_data_load_test(num_objects, num_regions)
    //           {
    //               eprintln!("Error in arbitrary data load test: {}", e);
    //           } else {
    //               println!(
    //                   "{}",
    //                   "Arbitrary data load test completed successfully".green()
    //               );
    //           }
    //   
    //           // Wait for the specified interval before running the next tests
    //           tokio::time::sleep(interval).await;
    //       }
    //   });

    println!("Finished starting plugin threads");

    // Define a place to put new players
    let players: Arc<Mutex<Vec<Player>>> = Arc::new(Mutex::new(Vec::new()));
    let (svc, io) = socketioxide::SocketIo::new_svc();
    let players_clone: Arc<Mutex<Vec<Player>>> = players.clone();

    // Handle new player connections
    io.ns("/", move |socket: SocketRef, data: Data<Value>| {
        println!("Player Connected!");
        on_connect(socket, data, players_clone.clone())
    });

    // Create a router to handle incoming network requests
    let app = Router::new()
        .get("/", redirect_to_master_panel) // Handle accessing server from browser
        .any("/*", ServiceHandler::new(svc)); // Any other protocalls go to socket server

    info!("Starting server");

    // Define a listener on port 3000
    let listener: tokio::net::TcpListener =
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on 0.0.0.0:3000");

    // Start the server and handle any errors
    if let Err(e) = serve(listener, app).await {
        println!("{}", e);
    }
    Ok(())
}
