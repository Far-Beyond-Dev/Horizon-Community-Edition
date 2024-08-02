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

// Use the mimalloc allocator for excellent performance across various tasks
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

// External crate imports
use serde_json::{json, Value};
use socketioxide::extract::{AckSender, Bin, Data, SocketRef};
use std::sync::{Arc, Mutex};
use tokio::{main, task::spawn};
use viz::{handler::ServiceHandler, serve, Response, Result, Router, Request, Body};

// Custom crate imports
use TerraForge;
use PebbleVault;

// Import all structs (temporary solution, to be refactored later)
use structs::*;

// Module imports
mod events;
mod macros;
mod structs;
mod subsystems;

/// Handles new player connections
///
/// This function is called every time a new player connects to the server.
/// It initializes player data and sets up event listeners for various game systems.
///
/// # Arguments
///
/// * `socket` - The SocketRef for the connected player
/// * `data` - Data associated with the connection
/// * `players` - Shared reference to the list of all players
fn on_connect(socket: SocketRef, Data(data): Data<Value>, players: Arc<Mutex<Vec<Player>>>) {
    let id = socket.id.as_str();
    println!("Player connected: {}", id);
    println!("Connection data: {:?}", data);

    // Initialize new player
    let player = Player {
        socket: socket.clone(),
        moveActionValue: None,
        transform: None
    };

    // Add player to the list
    players.lock().unwrap().push(player);
    println!("Player {} added to players list", id);

    // Emit connection events
    println!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);
    socket.emit("connected", true).ok();
    socket.emit("auth", true).ok();
         
    // Initialize subsystems
    subsystems::core::chat::init(socket.clone());
    subsystems::core::game_logic::init();
    subsystems::core::level_data::init();
    subsystems::core::logging::init();
    subsystems::core::notifications::init();
    
    // Set up event listeners
    setup_event_listeners(socket, Arc::clone(&players));
}

/// Sets up event listeners for a connected player
///
/// # Arguments
///
/// * `socket` - The SocketRef for the connected player
/// * `players` - Shared reference to the list of all players
fn setup_event_listeners(socket: SocketRef, players: Arc<Mutex<Vec<Player>>>) {
    let players_clone = Arc::clone(&players);

    // Update player location even
    socket.on(
        "updatePlayerLocation",
        move |socket: SocketRef, Data::<Value>(data), ack: AckSender, Bin(bin)|  {
            println!("Received updatePlayerLocation event");
            println!("Event data: {:?}", data);
            println!("Binary data: {:?}", bin);
            handle_update_player_location(socket, data, socketioxide::extract::Bin(bin), &players_clone);
        },
    );


    // Print raw data event
    socket.on(
        "printRaw",
        move |Data::<Value>(data), ack: AckSender, Bin(bin)| {
            println!("Received event with data: {:?} and bin: {:?}", data, bin);
            ack.bin(bin).send(data).ok();
        },
    );

    // Get online players event
    let players_clone = Arc::clone(&players);
    socket.on(
        "getOnlinePlayers",
        move |socket: SocketRef, _: Data<Value>, _: Bin| {
            handle_get_online_players(socket, &players_clone);
        },
    );

    // Get players with locations event
    let players_clone = Arc::clone(&players);
    socket.on(
        "getPlayersWithLocations",
        move |socket: SocketRef, Data::<Value>(data), ack: AckSender, Bin(bin)| {
            handle_get_players_with_locations(socket, data, ack, socketioxide::extract::Bin(bin), &players_clone);
        },
    );

    // Broadcast message event
    let players_clone = Arc::clone(&players);
    socket.on("broadcastMessage", move |Data::<Value>(data), _: Bin| {
        handle_broadcast_message(data, &players_clone);
    });
}

/// Handles updating a player's location
fn handle_update_player_location(socket: SocketRef, data: Value, bin: Bin, players: &Arc<Mutex<Vec<Player>>>) {
    println!("Inside handle_update_player_location function");
    println!("Received raw updatePlayerLocation data: {:?}", data);

    match serde_json::from_value::<Payload>(data.clone()) {
        Ok(payload) => {
            println!("Successfully parsed payload: {:?}", payload);
            let mut players = players.lock().unwrap();
            if let Some(player) = players.iter_mut().find(|p| p.socket.id == socket.id) {
                player.transform = Some(payload.transform);
                player.moveActionValue = Some(payload.move_action_value);
                println!("Updated player data: {:?}", player);
            } else {
                println!("Player not found: {}", socket.id);
            }
        }
        Err(e) => {
            println!("Failed to parse payload: {:?}", e);
            // Fallback to individual field parsing
            // ... (rest of the function remains the same)
        }
    }

}

/// Handles getting the list of online players
fn handle_get_online_players(socket: SocketRef, players: &Arc<Mutex<Vec<Player>>>) {
    println!("Responding with online players list");
    let players = players.lock().unwrap();
    let online_players_json = serde_json::to_value(
        players
            .iter()
            .map(|player| json!({ "id": player.socket.id }))
            .collect::<Vec<_>>(),
    )
    .unwrap();
    println!("Player Array as JSON: {}", online_players_json);
    socket.emit("onlinePlayers", online_players_json).ok();
}

/// Handles getting players with their locations
/// Handles getting players with their locations
fn handle_get_players_with_locations(socket: SocketRef, data: Value, ack: AckSender, bin: Bin, players: &Arc<Mutex<Vec<Player>>>) {
    println!("Responding with players and locations list");
    let players = players.lock().unwrap();
    
    match data {
        Value::Null => println!("Received event with null data"),
        Value::String(s) => println!("Received event with string data: {}", s),
        _ => println!("Received event with data: {:?}", data),
    }

    let players_with_locations_json = serde_json::to_value(
        players
            .iter()
            .map(|player| {
                let transform = player.transform.as_ref().map(|t| {
                    json!({
                        "rotation": t.rotation,
                        "translation": t.translation,
                        "scale3D": t.scale3D
                    })
                });
                json!({ 
                    "id": player.socket.id, 
                    "transform": transform
                })
            })
            .collect::<Vec<_>>(),
    )
    .unwrap();
    println!("Players with Locations as JSON: {}", players_with_locations_json);
    socket.emit("playersWithLocations", players_with_locations_json).ok();
}

/// Handles broadcasting a message to all players
fn handle_broadcast_message(data: Value, players: &Arc<Mutex<Vec<Player>>>) {
    let players = players.lock().unwrap();
    for player in &*players {
        player.socket.emit("broadcastMessage", data.clone()).ok();
    }
}

/// Redirects browser users to the master server dashboard
async fn redirect_to_master_panel(_req: Request) -> Result<Response> {
    let response = Response::builder()
        .status(302)
        .header("Location", "https://google.com")
        .body(Body::empty())
        .unwrap();
    println!("Someone tried to access this server via a browser, redirecting them to the master dashboard");
    Ok(response)
}

#[main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Show branding
    subsystems::core::startup::main();

    // Start TerraForge in a separate thread
    let _terraforge_thread = spawn(async {
        TerraForge::main();
    });
    
    // Initialize PebbleVault
    PebbleVault::main();

    // Initialize player storage
    let players: Arc<Mutex<Vec<Player>>> = Arc::new(Mutex::new(Vec::new()));
    let (svc, io) = socketioxide::SocketIo::new_svc();
    let players_clone: Arc<Mutex<Vec<Player>>> = players.clone();

    // Set up socket.io namespace for new connections
    io.ns("/", move |socket: SocketRef, data: Data<Value>| {
        println!("Player Connected!");
        on_connect(socket, data, players_clone.clone())
    });
    
    // Create a router to handle incoming connections
    let app = Router::new()
        .get("/", redirect_to_master_panel)
        .any("/*", ServiceHandler::new(svc));

    println!("Starting server");
    
    // Start the server
    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    if let Err(e) = serve(listener, app).await {
        println!("Server error: {}", e);
    }
    Ok(())
}