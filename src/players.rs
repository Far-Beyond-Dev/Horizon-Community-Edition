use serde_json::{json, Value};
use socketioxide::extract::{Data, SocketRef};
use std::sync::{Arc, Mutex};
use tracing::{debug, info};
use crate::structs::*;

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

pub fn get_players_with_locations(socket: SocketRef, data: Data<Value>, players: Arc<Mutex<Vec<Player>>>) {
    info!("Responding with players and locations list");
    let players = players.lock().unwrap();
    
    println!("Received event with data: {:?}", data.0);  // Access the inner Value
    let players_with_locations_json = serde_json::to_value(
        players
            .iter()
            .map(|player| json!({ 
                "id": player.socket.id, 
                "transform": player.transform.as_ref().unwrap().location
            }))
            .collect::<Vec<_>>(),
    )
    .unwrap();
    info!(
        "Players with Locations as JSON: {}",
        players_with_locations_json
    );
    let players = vec![players_with_locations_json];
    socket.emit("playersWithLocations", &players).ok();
}

pub fn broadcast_message(data: Data<Value>, players: Arc<Mutex<Vec<Player>>>) {
    let players = players.lock().unwrap();
    for player in &*players {
        player.socket.emit("broadcastMessage", data.0.clone()).ok();
    }
}

fn parse_rotation(parse: &Value) -> (f64, f64, f64, f64) {
    (
        parse_f64(&parse["w"]).unwrap_or(0.0),
        parse_f64(&parse["x"]).unwrap_or(0.0),
        parse_f64(&parse["y"]).unwrap_or(0.0),
        parse_f64(&parse["z"]).unwrap_or(0.0),
    )
}

fn parse_xyz(parse: &Value) -> (f64, f64, f64) {
    (
        parse_f64(&parse["x"]).unwrap_or(0.0),
        parse_f64(&parse["y"]).unwrap_or(0.0),
        parse_f64(&parse["z"]).unwrap_or(0.0),
    )
}

fn parse_xy(parse: &Value) -> (f64, f64) {
    (
        parse_f64(&parse["x"]).unwrap_or(0.0),
        parse_f64(&parse["y"]).unwrap_or(0.0),
    )
}

fn parse_f64(n: &Value) -> Result<f64, std::io::Error> {
    n.as_f64().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid f64 value"))
}