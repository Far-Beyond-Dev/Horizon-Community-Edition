use serde_json::{json, Value};
use socketioxide::extract::{AckSender, Bin, Data, SocketRef};
use std::sync::{Arc, Mutex};
use tracing::{debug, info};
use crate::{on_connect, Player};
use crate::structs::*;


pub fn init (socket: SocketRef) {
    let players: Arc<Mutex<Vec<Player>>> = Arc::new(Mutex::new(Vec::new()));


    let (_io, io2) = socketioxide::SocketIo::new_svc();

    let players_clone = players.clone();
    let players_clone_two = players.clone();
    io2.ns("/", move |socket, data| {
        on_connect(socket, data, players_clone.clone())
    });


    socket.on(
        "UpdatePlayerLocation",
        move |socket: SocketRef, Data::<Value>(data), Bin(bin)| {
            info!(
                "Received event: UpdatePlayerLocation with data: {:?} and bin: {:?}",
                data, bin
            );

            // Extract location from data
            match serde_json::from_value::<Location>(data.clone()) {
                Ok(location) => {
                    let mut players = players_clone_two.lock().unwrap();
                    if let Some(player) = players.iter_mut().find(|p| p.id == socket.id.to_string())
                    {
                        player.location = Some(location);
                        info!("Updated player location: {:?}", player);
                    } else {
                        info!("Player not found: {}", socket.id);
                    }
                }
                Err(err) => {
                    info!("Failed to parse location: {:?}", err);
                } 
            }
            
            socket.bin(bin).emit("message-back", data).ok();
        },
    );
    
    socket.on(
        "message-with-ack",
        move |Data::<Value>(data), ack: AckSender, Bin(bin)| {
            info!(
                "Received event: message-with-ack with data: {:?} and bin: {:?}",
                data, bin
            );
            ack.bin(bin).send(data).ok();
        },
    );
    
    let players_clone = Arc::clone(&players);
    socket.on(
        "getOnlinePlayers",
        move |socket: SocketRef, _: Data<Value>, _: Bin| {
            info!("Responding with online players list");
            let players = players_clone.lock().unwrap();

            let online_players_json = serde_json::to_value(
                players
                .iter()
                .map(|player| json!({ "id": player.id }))
                    .collect::<Vec<_>>(),
                )
                .unwrap();
            
            debug!("Player Array as JSON: {}", online_players_json);
            socket.emit("onlinePlayers", online_players_json).ok();
        },
    );

    let players_clone = Arc::clone(&players);
    socket.on(
        "getPlayersWithLocations",
        move |socket: SocketRef, _: Data<Value>, _: Bin| {
            info!("Responding with players and locations list");
            let players = players_clone.lock().unwrap();
            
            let players_with_locations_json = serde_json::to_value(
                players
                .iter()
                .map(|player| json!({ "id": player.id, "location": player.location }))
                .collect::<Vec<_>>(),
            )
            .unwrap();
        
            info!(
                "Players with Locations as JSON: {}",
                players_with_locations_json
            );
            let players = vec![players_with_locations_json];

            socket.emit("playersWithLocations", &players).ok();
        },
    );
    
    let players_clone = Arc::clone(&players);
    socket.on("broadcastMessage", move |Data::<Value>(data), _: Bin| {
        let players = players_clone.lock().unwrap();
        for player in &*players {
            player.socket.emit("broadcastMessage", data.clone()).ok();
        }
    });
}
