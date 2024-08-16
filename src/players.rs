use serde_json::{json, Value};
use socketioxide::extract::{Data, SocketRef};
use std::sync::{Arc, Mutex};
use tracing::{debug, info};
use crate::structs::*;


pub fn init(socket: SocketRef, players: Arc<Mutex<Vec<Player>>>) {
    /////////////////////////////////////////////////////////
    //  Register some additional custom events with our    // 
    //  socket server. Your custom events will be          //
    //  registered here as well as in the ./events/mod.rs  //
    //  file                                               //
    /////////////////////////////////////////////////////////
    
    // Register events for player interactions
    let players_clone = Arc::clone(&players);
    socket.on("updatePlayerLocation", move |s, d|
        update_player_location(s, d, players_clone.clone()),
    );

    let players_clone = Arc::clone(&players);
    socket.on("getOnlinePlayers", move |s|
        get_online_players(s, players_clone.clone()),
    );

    let players_clone = Arc::clone(&players);
    socket.on("getPlayersWithLocations", move |s, d|
        get_players_with_locations(s, d, players_clone.clone()),
    );

    let players_clone = Arc::clone(&players);
    socket.on("broadcastMessage", move |d|
        broadcast_message(d, players_clone.clone()),
    );

    // Register events using the socketioxide API directly
    let players_clone = Arc::clone(&players);
    socket.on("updatePlayerLocation", move |s: SocketRef, d: Data<Value>| {
        update_player_location(s, d, players_clone.clone())
    });

    let players_clone = Arc::clone(&players);
    socket.on("getOnlinePlayers", move |s: SocketRef| {
        get_online_players(s, players_clone.clone())
    });

    let players_clone = Arc::clone(&players);
    socket.on("getPlayersWithLocations", move |s: SocketRef, d: Data<Value>| {
        get_players_with_locations(s, d, players_clone.clone())
    });

    let players_clone = Arc::clone(&players);
    socket.on("broadcastMessage", move |d: Data<Value>| {
        broadcast_message(d, players_clone.clone())
    });
}


/// Updates the location and related information of a player based on received data.
///
/// This function processes incoming data about a player's position, rotation, scale, and movement,
/// updating the corresponding player's information in the shared players collection.
///
/// # Arguments
///
/// * `socket` - A reference to the WebSocket connection for the client sending the update.
/// * `data` - The data received with the event, expected to contain player transform information.
/// * `players` - A thread-safe reference to the collection of all players in the game.
///
/// # Behavior
///
/// 1. Logs the received event data for debugging purposes.
/// 2. Attempts to extract the "transform" object from the received data.
/// 3. If successful, it locks the shared players collection and finds the player matching the socket ID.
/// 4. For the found player, it updates the following information:
///    - Transform (rotation, translation, scale3D, and location)
///    - Move Action Value (player's movement input)
///    - Control Rotation
/// 5. Prints debug information about the updated player.
/// 6. Sends a reply to the client with the received data.
///
/// # Error Handling
///
/// - Prints error messages if:
///   - The transform data structure is invalid
///   - The player is not found
///   - The location data cannot be parsed
///
/// # Note
///
/// This function assumes specific data structures for the incoming data and may fail silently
/// if the expected fields are missing or in an unexpected format.
pub fn update_player_location(socket: SocketRef, data: Data<Value>, players: Arc<Mutex<Vec<Player>>>) {
    println!(
        "Received event: UpdatePlayerLocation with data: {:?}",
        data.0  // Access the inner Value
    );
    // Extract location from data
    if let Some(transform) = data.0.get("transform").and_then(|t| t.as_object()) {
        let mut players = players.lock().unwrap();
        if let Some(player) = players.iter_mut().find(|p| p.socket.id == socket.id) {
            // Do the actual parsing
            if let (Some(rotation), Some(translation), Some(scale3d)) = (
                transform.get("rotation"),
                transform.get("translation"),
                transform.get("scale3D")
            ) {
                let (rot_w, rot_x, rot_y, rot_z) = parse_rotation(rotation);
                let (trans_x, trans_y, trans_z) = parse_xyz(translation);
                let (scale3d_x, scale3d_y, scale3d_z) = parse_xyz(scale3d);
           
                // Create or update the transform
                let mut transform = player.transform.take().unwrap_or_else(|| Transform {
                    rotation: None,
                    translation: None,
                    scale3D: Scale3D { x: 1.0, y: 1.0, z: 1.0 },
                    location: None,
                });
           
                // Update rotation
                transform.rotation = Some(Rotation { w: rot_w, x: rot_x, y: rot_y, z: rot_z });
           
                // Update translation
                let new_translation = Translation { x: trans_x, y: trans_y, z: trans_z };
                transform.translation = Some(new_translation.clone());
                transform.location = Some(new_translation);
           
                // Update scale3D
                transform.scale3D = Scale3D { x: scale3d_x, y: scale3d_y, z: scale3d_z };
                // Update the player's transform
                player.transform = Some(transform);
           
                // Parse player movement axis values
                if let Some(move_action) = data.0.get("move Action Value") {
                    let (mv_action_value_x, mv_action_value_y) = parse_xy(move_action);
                    player.moveActionValue = Some(MoveActionValue { x: mv_action_value_x, y: mv_action_value_y });
                }
                if let Some(controlRotation) = data.0.get("control Rotation") {
                    let (ctrl_rot_x, ctrl_rot_y, ctrl_rot_z) = parse_xyz(controlRotation);
                    player.controlRotation = Some(Vec3D { x: ctrl_rot_x, y: ctrl_rot_y, z: ctrl_rot_z });
                }
                // Print a debug statement
                println!("Updated player location: {:?}", player);
            } else {
                println!("Invalid transform data structure");
            }
        } else {
            println!("Player not found: {}", socket.id);
        }
    } else {
        println!("Failed to parse location: transform field not found or is not an object");
    }
    // Send a reply containing the correct data
    socket.emit("messageBack", data.0).ok();
}

/// Retrieves and sends a list of online players to a connected client.
///
/// This function gathers the IDs of all currently connected players and sends them
/// to the requesting client via a WebSocket connection.
///
/// # Arguments
///
/// * `socket` - A reference to the WebSocket connection for the client requesting the data.
/// * `players` - A thread-safe reference to the collection of all players in the game.
///
/// # Behavior
///
/// 1. Logs an info message about responding with the online players list.
/// 2. Acquires a lock on the shared players collection.
/// 3. Constructs a JSON array containing the ID of each connected player.
/// 4. Logs the constructed JSON array for debugging purposes.
/// 5. Emits an "onlinePlayers" event to the requesting client with the JSON data.
///
/// # Note
///
/// This function only sends the IDs of the players, not any additional information.
///
/// # Errors
///
/// While the function itself doesn't return a Result, it silently ignores any errors
/// that occur when emitting the event to the socket.
pub fn get_online_players(socket: SocketRef, players: Arc<Mutex<Vec<Player>>>) {
    info!("Responding with online players list");
    let players = players.lock().unwrap();
    let online_players_json = serde_json::to_value(
        players
            .iter()
            .map(|player| json!({ "id": player.socket.id }))
            .collect::<Vec<_>>(),
    )
    .unwrap();
    debug!("Player Array as JSON: {}", online_players_json);
    socket.emit("onlinePlayers", online_players_json).ok();
}

/// Retrieves and sends player locations and related information to a connected client.
///
/// This function is responsible for gathering data about all players, including their
/// positions, rotations, and movement information, and sending it to a specific client
/// via a WebSocket connection.
///
/// # Arguments
///
/// * `socket` - A reference to the WebSocket connection for the client requesting the data.
/// * `data` - The data received with the event (currently unused in the function body).
/// * `players` - A thread-safe reference to the collection of all players in the game.
///
/// # Behavior
///
/// 1. Logs an info message about responding with the players and locations list.
/// 2. Acquires a lock on the shared players collection.
/// 3. Prints the received event data for debugging purposes.
/// 4. Constructs a JSON representation of each player's data, including:
///    - Player ID
///    - Transform (likely position and orientation)
///    - Move Action Value (possibly related to current movement state)
///    - Rotation (player's current rotation)
/// 5. Serializes the collected player data into a JSON value.
/// 6. Emits a "playersWithLocations" event to the requesting client with the serialized data.
///
/// # Note
///
/// This function assumes that all players have valid data for transform, moveActionValue,
/// and controlRotation. It may panic if these values are None.
///
/// # Errors
///
/// While the function itself doesn't return a Result, it silently ignores any errors
/// that occur when emitting the event to the socket.
pub fn get_players_with_locations(socket: SocketRef, data: Data<Value>, players: Arc<Mutex<Vec<Player>>>) {
    info!("Responding with players and locations list");
    let players = players.lock().unwrap();
    
    println!("Received event with data: {:?}", data.0);  // Access the inner Value
    let players_with_locations_json = serde_json::to_value(
        
        // Create response packet structure
        players
            .iter()
            .map(|player| json!({ 
                "Id": player.socket.id, 
                "Transform": player.transform.as_ref().unwrap(),
                "Move Action Value": player.moveActionValue.as_ref().unwrap(),
                "Rotation": player.controlRotation.as_ref().unwrap()
            }))
            .collect::<Vec<_>>(),
    )
    .unwrap();

    // Prepare and send the data
    let players = vec![players_with_locations_json];
    socket.emit("playersWithLocations", &players).ok();
}


/// Forward the message content to all clients
pub fn broadcast_message(data: Data<Value>, players: Arc<Mutex<Vec<Player>>>) {
    let players = players.lock().unwrap();
    for player in &*players {
        player.socket.emit("broadcastMessage", data.0.clone()).ok();
    }
}


///////////////////////////////////////////////////////
//                 Parsing Functions                 //
//  These functions assist in parsing common data    //
//  fields out of JSON objects. Eventually, when     //
//  Horizon's internal systems transition to binary, //
//  these will be updated accordingly.               //
///////////////////////////////////////////////////////

/// Parses a rotation from a JSON Value into individual components.
///
/// # Arguments
///
/// * `parse` - A reference to a JSON Value containing rotation data.
///
/// # Returns
///
/// A tuple of four `f64` values representing (w, x, y, z) components of the rotation.
///
/// # Behavior
///
/// - Attempts to extract "w", "x", "y", and "z" fields from the input JSON.
/// - If any field is missing or invalid, it defaults to 0.0 for that component.
///
/// # Note
///
/// This function is designed to work with quaternion representations of rotations.
fn parse_rotation(parse: &Value) -> (f64, f64, f64, f64) {
    (
        parse_f64(&parse["w"]).unwrap_or(0.0),
        parse_f64(&parse["x"]).unwrap_or(0.0),
        parse_f64(&parse["y"]).unwrap_or(0.0),
        parse_f64(&parse["z"]).unwrap_or(0.0),
    )
}

/// Parses a 3D vector from a JSON Value into individual components.
///
/// # Arguments
///
/// * `parse` - A reference to a JSON Value containing 3D vector data.
///
/// # Returns
///
/// A tuple of three `f64` values representing (x, y, z) components of the vector.
///
/// # Behavior
///
/// - Attempts to extract "x", "y", and "z" fields from the input JSON.
/// - If any field is missing or invalid, it defaults to 0.0 for that component.
///
/// # Note
///
/// This function is commonly used for parsing position or scale data in 3D space.
fn parse_xyz(parse: &Value) -> (f64, f64, f64) {
    (
        parse_f64(&parse["x"]).unwrap_or(0.0),
        parse_f64(&parse["y"]).unwrap_or(0.0),
        parse_f64(&parse["z"]).unwrap_or(0.0),
    )
}

/// Parses a 2D vector from a JSON Value into individual components.
///
/// # Arguments
///
/// * `parse` - A reference to a JSON Value containing 2D vector data.
///
/// # Returns
///
/// A tuple of two `f64` values representing (x, y) components of the vector.
///
/// # Behavior
///
/// - Attempts to extract "x" and "y" fields from the input JSON.
/// - If any field is missing or invalid, it defaults to 0.0 for that component.
///
/// # Note
///
/// This function is commonly used for parsing 2D coordinates or movement vectors.
fn parse_xy(parse: &Value) -> (f64, f64) {
    (
        parse_f64(&parse["x"]).unwrap_or(0.0),
        parse_f64(&parse["y"]).unwrap_or(0.0),
    )
}

/// Attempts to parse a single f64 value from a JSON Value.
///
/// # Arguments
///
/// * `n` - A reference to a JSON Value expected to contain a number.
///
/// # Returns
///
/// A `Result` containing either the parsed `f64` value or an `std::io::Error`.
///
/// # Errors
///
/// Returns an `std::io::Error` with kind `InvalidData` if the value cannot be parsed as an f64.
///
/// # Note
///
/// This function is used internally by other parsing functions to handle individual numeric values.
fn parse_f64(n: &Value) -> Result<f64, std::io::Error> {
    n.as_f64().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid f64 value"))
}