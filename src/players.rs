use serde_json::{json, Value};
use serde::Serialize;
use horizon_logger::{HorizonLogger, log_info, log_debug, log_warn, log_error, log_critical, LogLevel};
use socketioxide::extract::{Data, SocketRef};
use std::fmt::Debug;
use std::sync::RwLock;
use std::sync::Arc;
use tracing::{debug, info};
use std::time::{Duration, Instant};
use horizon_data_types::*;
use rayon::*; //We need all the Rayon!!!!
use iter::IntoParallelIterator;
use iter::ParallelIterator;
use iter::IntoParallelRefIterator;

// impl Default for MoveActionValue {
//     fn default() -> Self {
//         MoveActionValue { x: 0.0, y: 0.0 }
//     }
// }

pub fn init(socket: SocketRef, players: Arc<RwLock<Vec<Player>>>) {
    /////////////////////////////////////////////////////////////
    //  Register some additional custom events with our        //
    //  socket server. Your custom events will be              //
    //  registered here as well as in the ./events/mod.rs      //
    //  file                                                   //
    /////////////////////////////////////////////////////////////

    let players_disconnect = players.clone();
    let logger: Arc<HorizonLogger> = Arc::new(HorizonLogger::new());

    let temp_logger: Arc<HorizonLogger> = logger.clone();
    socket.on_disconnect(move |s| {
        on_disconnect(s, players_disconnect.clone(), temp_logger)
    });

    // Register events for player interactions
    let players_clone = Arc::clone(&players);
    let temp_logger: Arc<HorizonLogger> = logger.clone();

    socket.on("updatePlayerLocation", move |s, d|
        update_player_location(s, d, players_clone.clone(), temp_logger),
    );

    let players_clone = Arc::clone(&players);
    socket.on("playerJump", move |s: SocketRef, d: Data<Value>| {
        player_jump(s, d)
    });

    let players_clone = Arc::clone(&players);
    socket.on("playerWalkToggle", move |s: SocketRef, d: Data<Value>| {
        player_walk_toggle(s, d)
    });

    let players_clone = Arc::clone(&players);
    let temp_logger: Arc<HorizonLogger> = logger.clone();
    socket.on("getOnlinePlayers", move |s|
        get_online_players(s, players_clone.clone(), temp_logger),
    );

    let players_clone = Arc::clone(&players);
    let temp_logger: Arc<HorizonLogger> = logger.clone();

    socket.on("getPlayersWithLocations", move |s, d: Data<Value>|
        get_players_with_locations(s, d, players_clone.clone(), temp_logger),
    );

    let players_clone = Arc::clone(&players);
    socket.on("broadcastMessage", move |d|
        broadcast_message(d, players_clone.clone()),
    );

    // Register events using the socketioxide API directly
    let players_clone: Arc<RwLock<Vec<Player>>> = Arc::clone(&players);
    let temp_logger: Arc<HorizonLogger> = logger.clone();

    socket.on("updatePlayerLocation", move |s: SocketRef, d: Data<Value>| {
        update_player_location(s, d, players_clone.clone(), temp_logger)
    });

    let players_clone = Arc::clone(&players);
    let temp_logger: Arc<HorizonLogger> = logger.clone();
    socket.on("getOnlinePlayers", move |s: SocketRef| {
        get_online_players(s, players_clone.clone(), temp_logger)
    });

    let players_clone = Arc::clone(&players);
    socket.on("getPlayersWithLocations", move |s: SocketRef, d: Data<Value>| {
        get_players_with_locations(s, d, players_clone.clone(), logger.clone())
    });

    let players_clone = Arc::clone(&players);
    socket.on("broadcastMessage", move |d: Data<Value>| {
        broadcast_message(d, players_clone.clone())
    });
}

pub fn on_disconnect(socket: SocketRef, players: Arc<RwLock<Vec<Player>>>, logger: Arc<HorizonLogger>) {
    if let Some(mut players) = players.write().log(&logger, LogLevel::WARN, "I/O Error", "Acquiring ReadWriteLock failed") {
        if let Some(index) = players.iter().position(|p| p.socket.id == socket.id) {
            players.remove(index);
            log_info!(logger, "CONNECTION", "Player {} disconnected, and cleaned up successfully", socket.id)
        } else {
            log_info!(logger, "CONNECTION", "Player {} successfully, but cleanup failed due to a corrupted player state. (This could be caused by plugins registering fake players improperly)", socket.id)
        }
    }
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
pub fn update_player_location(socket: SocketRef, data: Data<Value>, players: Arc<RwLock<Vec<Player>>>, logger: Arc<HorizonLogger>) {
    println!("Received event: UpdatePlayerLocation with data: {:?}", data.0);

    let player_data = &data.0; // This is going to by the Json data
    if let Some(mut players) = players.write().log(&logger, LogLevel::WARN, "I/O Error", "Acquiring ReadWriteLock failed") {
        if let Some(player) = players.iter_mut().find(|p| p.socket.id == socket.id) {
            // Update control rotation
            if let Some(control_rotation) = player_data.get("controlRotation") {
                player.controlRotation = Some(Vec3D {
                    x: control_rotation["x"].as_f64().unwrap_or(0.0),
                    y: control_rotation["y"].as_f64().unwrap_or(0.0),
                    z: control_rotation["z"].as_f64().unwrap_or(0.0),
                });
            }

            // Update root position
            if let Some(root_position) = player_data.get("rootPosition") {
                let new_position = Translation {
                    x: root_position["x"].as_f64().unwrap_or(0.0),
                    y: root_position["y"].as_f64().unwrap_or(0.0),
                    z: root_position["z"].as_f64().unwrap_or(0.0),
                };
                player.transform.get_or_insert(Transform::default()).location = Some(new_position);
            }

            // Update root rotation
            if let Some(root_rotation) = player_data.get("rootRotation") {
                let new_rotation = Rotation {
                    x: root_rotation["x"].as_f64().unwrap_or(0.0),
                    y: root_rotation["y"].as_f64().unwrap_or(0.0),
                    z: root_rotation["z"].as_f64().unwrap_or(0.0),
                    w: 1.0, // Assuming w is not provided in the data
                };
                player.transform.get_or_insert(Transform::default()).rotation = Some(new_rotation);
            }

            // Update root velocity
            if let Some(root_velocity) = player_data.get("rootVelocity") {
                player.root_velocity = Some(Vec3D {
                    x: root_velocity["x"].as_f64().unwrap_or(0.0),
                    y: root_velocity["y"].as_f64().unwrap_or(0.0),
                    z: root_velocity["z"].as_f64().unwrap_or(0.0),
                });
            }

            // Update key bone data
            if let Some(key_bone_data) = player_data.get("keyBoneData").and_then(|v| v.as_array()) {
                let key_joints: Vec<Vec3D> = key_bone_data.into_par_iter()
                    .filter_map(|bone| {
                        Some(Vec3D {
                            x: bone["x"].as_f64()?,
                            y: bone["y"].as_f64()?,
                            z: bone["z"].as_f64()?,
                        })
                    })
                    .collect();
                // You might want to store this key_joints data in your Player struct
            }

            // Process trajectory path
            if let Some(trajectory) = player_data.get("trajectoryPath").and_then(|v| v.as_array()) {
                let path: Vec<TrajectoryPoint> = trajectory.into_par_iter()
                    .filter_map(|point| {
                        Some(TrajectoryPoint {
                            accumulated_seconds: point["accumulatedSeconds"].as_f64()?,
                            facing: Rotation {
                                w: point["facing"]["w"].as_f64()?,
                                x: point["facing"]["x"].as_f64()?,
                                y: point["facing"]["y"].as_f64()?,
                                z: point["facing"]["z"].as_f64()?,
                            },
                            position: Translation {
                                x: point["position"]["x"].as_f64()?,
                                y: point["position"]["y"].as_f64()?,
                                z: point["position"]["z"].as_f64()?,
                            },
                        })
                    })
                    .collect();
                // Store this path in your Player struct for prediction and smoothing
                player.trajectory_path = Some(path);
            }

            println!("Updated player state: {:?}", player);
        } else {
            println!("Player not found: {}", socket.id);
        }


        // Send a reply containing the correct data. This will only happen if and only if the player data is writeable
        socket.emit("messageBack", &json!({
            "status": "success",
            "message": "Player location updated successfully"
        })).ok();
    } else {
        // Send a reply containing the correct data. This will happen if the player data isn't writeable
        socket.emit("messageBack", &json!({
            "status": "failure",
            "message": "Player location update failed"
        })).ok();
    }
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
pub fn get_online_players(socket: SocketRef, players: Arc<RwLock<Vec<Player>>>, logger: Arc<HorizonLogger>) {
    info!("Responding with online players list");
    let players = players.read(); // previously write but it only requires read-only access

    if let Some(players) = players.log(&logger, LogLevel::WARN, "I/O Error", "Acquiring ReadWriteLock failed ") {
        let online_players_json = serde_json::to_value(
            players
                .par_iter()
                .map(|player| json!({ "id": player.socket.id }))
                .collect::<Vec<_>>(),
        )
        .map_err(|e|{eprintln!("Failed to get json for online players: {}",e)});
        debug!("Player Array as JSON: {:#?}", online_players_json);
        socket.emit("onlinePlayers", &online_players_json).ok();
    } else {
        let error_data: Value = json!({
            "message": "Failed to read player data. Please try again later.",
            "code": "LOCK_READ_FAILURE"
        });
        let _ = socket.emit("error", &error_data);
    }
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

#[derive(Serialize)]
struct PlayersResponse {
    players: Vec<serde_json::Value>
}

pub fn get_players_with_locations(socket: SocketRef, data: Data<Value>, players: Arc<RwLock<Vec<Player>>>, logger: Arc<HorizonLogger>) {
    println!("Responding with players and locations list");
    let players = players.read(); // Previously write now read

    if let Some(players) = players.log(&logger, LogLevel::WARN, "I/O Error", "Acquiring ReadWrite Lock failed") {
        println!("Received event with data: {:?}", data.0);

        let players_with_locations_json: Vec<serde_json::Value> = players
        .par_iter() // Convert to parallel iterator faster searching
        .map(|player| {
            json!({
                "Id": player.id,
                "Root Position": player.transform.as_ref().and_then(|t| t.location.as_ref()),
                "Root Rotation": player.transform.as_ref().and_then(|t| t.rotation.as_ref()),
                "Root Velocity": player.root_velocity,
                "Control Rotation": player.controlRotation,
    //          "Move Action Value": player.moveActionValue,
                "Trajectory Path": player.trajectory_path.as_ref().map(|path|
                    path.iter().take(10).map(|point| json!({
                        "accumulatedSeconds": point.accumulated_seconds,
                        "facing": point.facing,
                        "position": point.position,
                    })).collect::<Vec<_>>()
                ),
                "KeyJoints": player.key_joints,
                "AnimationState": player.animation_state,
                "IsActive": player.is_active,
                "LastUpdateTime": player.last_update.elapsed().as_secs_f64(),
            })
        })
        .collect();

        println!("Number of players: {}", players_with_locations_json.len());

        // Create the response with a "players" field
        let response = PlayersResponse {
            players: players_with_locations_json
        };

        // Serialize the response

        if let Some(response_json) = serde_json::to_value(response).log(&logger, LogLevel::ERROR, "SERIALIZATION", "JSON Parsing Error") {
            println!("Sending players data: {:?}", response_json);
            println!("JSON string being sent: {}", serde_json::to_string(&response_json).unwrap()); // Should not fail. It's the inverse function of to_value

            let _ = socket.emit("playersWithLocations", &response_json);
        } else {
            let error_data: Value = json!({
                "message": "Failed parse json data",
                "code": "Serialization error"
            });

            let _ = socket.emit("error", &error_data); // Take a look
        }
    } else {
        let error_data: Value = json!({
            "message": "Failed to read player data. Please try again later.",
            "code": "LOCK_READ_FAILURE"
        });

        let _ = socket.emit("error", &error_data);
    }
}

pub fn broadcast_message(data: Data<Value>, players: Arc<RwLock<Vec<Player>>>) {
    if let Ok(players_guard) = players.read() {
        // Access the Vec's elements through .iter()
        for player in players_guard.iter() {
            player.socket.emit("broadcastMessage", &data.0).ok();
        }
    }
}

fn player_jump(socket: SocketRef, data: Data<Value>) {
    // Process the jump event
    // You might want to update the player's state or notify other players

    // Emit the playerJumped event
    socket.emit("playerJumped", &true).expect("Failed to emit playerJumped event");
}

fn player_walk_toggle(socket: SocketRef, data: Data<Value>) {
    // Process the jump event
    // You might want to update the player's state or notify other players

    // Emit the playerJumped event
    socket.emit("playerWalkToggled", &true).expect("Failed to emit playerWalkToggled event");
}

pub async fn cleanup_inactive_players(players: Arc<RwLock<Vec<Player>>>, logger: Arc<HorizonLogger>) {
    let inactive_threshold = Duration::from_secs(60); // 1 minute

    loop {
        tokio::time::sleep(Duration::from_secs(30)).await; // Run every 30 seconds

        if let Some(mut players) = players.write().log(&logger, LogLevel::WARN, "CLEANUP", "Starting cleanup of inactive players") {
            let now = Instant::now();

            players.retain(|player| {
                if !player.is_active && now.duration_since(player.last_update) > inactive_threshold {
                    println!("Removing inactive player: {}", player.socket.id);
                    false // Remove the player
                } else {
                    true // Keep the player
                }
            });
        }
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


trait Logging<T>{
    /// Logs an error from a `Result<T, E>` if it is `Err` using the provided `HorizonLogger`.
    ///
    /// # Arguments
    /// * `logger` - The logger instance to use.
    /// * `log_level` - The severity level for logging.
    /// * `kind` - The component or context identifier.
    /// * `msg` - The custom message to log.
    ///
    /// # Returns
    /// * `Option<T>` - `Some(T)` if `Ok`, otherwise `None`.
    fn log(self, logger: &HorizonLogger, log_level: LogLevel, kind: &str, msg: &str) -> Option<T>;
}

impl <T, E: Debug> Logging<T> for Result<T, E> {
    fn log(self, logger: &HorizonLogger, log_level: LogLevel, kind: &str, msg: &str) -> Option<T> {
        if let Err(err) = &self {
            match log_level {
                LogLevel::DEBUG => log_debug!(logger, kind, "{}: {:?}", msg, err),
                LogLevel::INFO => log_info!(logger, kind, "{}: {:?}", msg, err),
                LogLevel::WARN => log_warn!(logger, kind, "{}: {:?}", msg, err),
                LogLevel::ERROR => log_error!(logger, kind, "{}: {:?}", msg, err),
                LogLevel::CRITICAL => log_critical!(logger, kind, "{}: {:?}", msg, err),
            }
        }

        self.ok()
    }
}
