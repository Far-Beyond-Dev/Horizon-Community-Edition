////////////////////////////////////////////////////////////////////////////////////////////////////
//                                      Horizon Game Server                                       //
//                                                                                                //
// This server software is part of a distributed system designed to facilitate communication      //
// and data transfer between multiple child servers and a master server. Each child server        //
// operates within a "Region map" managed by the master server, which keeps track of their        //
// coordinates in a relative cubic light-year space. The coordinates are stored in 64-bit floats  //
// to avoid coordinate overflow and to ensure high precision.                                     //
//                                                                                                //
////////////////////////////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////
// Import a few things to get us started //
///////////////////////////////////////////

use serde_json::{json, Value};
use socketioxide::extract::{AckSender, Bin, Data, SocketRef};
use std::sync::{Arc, Mutex};
use tokio::main;
use tracing::{debug, info};
use tracing_subscriber::FmtSubscriber;
use viz::{handler::ServiceHandler, serve, Result, Router};


// WARNING
// Import all structs (when we have a ton of structs this will be very bad but should be fine for now)

use structs::*;

/////////////////////////////////////
// Import the modules we will need //
/////////////////////////////////////

mod events;
mod macros;
mod utilities;
mod structs;

///////////////////////////////////////////////////////////////
//                         WARNING                           //
// on_connect runs every time a new player connects to the   //
// server avoid putting memory hungry code here if possible! //
///////////////////////////////////////////////////////////////

fn on_connect(socket: SocketRef, Data(data): Data<Value>, players: Arc<Mutex<Vec<Player>>>) {
    // Authenticate the user
    let player = Player {
        id: socket.id.to_string(),
        socket: socket.clone(),
        location: None, // Initialize with no location
    };

    players.lock().unwrap().push(player);

    info!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);
    socket.emit("connected", true).ok();
    socket.emit("auth", data).ok();

    let players_clone = Arc::clone(&players);

    /////////////////////////////////////////////////////////
    // Setup external event listeners for the more complex //
    // systems                                             //
    /////////////////////////////////////////////////////////

    utilities::chat::main();
    utilities::game_logic::main();
    utilities::leaderboard::main();
    utilities::level_data::main();
    utilities::logging::main();
    utilities::notifications::main();
    utilities::player_data::main();

    ////////////////////////////////////////////////////////
    // Register some custom events with our socket server //
    // Your custom events will also be registered here as //
    // well as in the ./events/mod.rs file.               //
    ////////////////////////////////////////////////////////

    define_event!(socket, "test", events::test::main());

    ////////////////////////////////////////////////////////
    // Register some custom events with our socket server //
    ////////////////////////////////////////////////////////

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
                    let mut players = players_clone.lock().unwrap();
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

#[main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    //////////////////////////////////
    // Show branding during startup //
    //////////////////////////////////
    utilities::startup::main();

    //TerraForge::main();
    // let test = TerraForge::main();
    // println!("TerraForge: {:?}", test);

    let players: Arc<Mutex<Vec<Player>>> = Arc::new(Mutex::new(Vec::new()));

    let (svc, io) = socketioxide::SocketIo::new_svc();

    let players_clone = players.clone();
    io.ns("/", move |socket, data| {
        on_connect(socket, data, players_clone.clone())
    });

    let app = Router::new()
        .get("/", |_| async { Ok("Hello, World!") })
        .any("/*", ServiceHandler::new(svc));

    info!("Starting server");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    if let Err(e) = serve(listener, app).await {
        println!("{}", e);
    }

    Ok(())
}