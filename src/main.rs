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
 
////////////////////////////////////////////////////////////
// Use the jemalloc allocator, which boasts fragmentation //
// resistance and scalable concurrency support            //
////////////////////////////////////////////////////////////
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

///////////////////////////////////////////
// Import a few things to get us started //
///////////////////////////////////////////

use http::status;
use serde_json::{json, Value};
use socketioxide::extract::{AckSender, Bin, Data, SocketRef};
use std::sync::{Arc, Mutex};
use tokio::{main, task::spawn};
use tracing::{debug, info};
use viz::{future::ok, handler::ServiceHandler, serve, Response, Result, Router, Request, Body};
use PebbleVault;
use TerraForge;

// WARNING
// Import all structs (when we have a ton of structs this will be very bad but should be fine for now)

use structs::*;

/////////////////////////////////////
// Import the modules we will need //
/////////////////////////////////////

mod events;
mod macros;
mod structs;
mod subsystems;

///////////////////////////////////////////////////////////////
//                         WARNING                           //
// on_connect runs every time a new player connects to the   //
// server avoid putting memory hungry code here if possible! //
///////////////////////////////////////////////////////////////

fn on_connect(socket: SocketRef, Data(data): Data<Value>, players: Arc<Mutex<Vec<Player>>>) {
    let id = socket.id.as_str();
    println!("Starting subsystems for player: {}", id);

    // Authenticate the user
    let player = Player {
        socket: socket.clone(),
        location: None, // Initialize with no location
    };

    players.lock().unwrap().push(player);
    println!("Player {} added to players list", id);

    println!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);
    socket.emit("connected", true).ok();
    socket.emit("auth", true).ok();
         
    /////////////////////////////////////////////////////////
    // Setup external event listeners for the more complex //
    // systems                                             //
    /////////////////////////////////////////////////////////
    
    //subsystems::actors::main::main::main;

    subsystems::game_logic::init();
    subsystems::chat::init(socket.clone());
    subsystems::leaderboard::init();
    subsystems::level_data::init();
    subsystems::logging::init();
    subsystems::notifications::init();

    // subsystems::player_data::init(socket.clone());
    
    ////////////////////////////////////////////////////////
    // Register some additional custom events with our    // 
    // socket server. Your custom events will be          //
    // registered here as well as in the ./events/mod.rs  //
    // file                                               //
    ////////////////////////////////////////////////////////
    
    define_event!(socket, 
        "test", events::test::main(),
        );

    let players_clone = Arc::clone(&players);
    
    ////////////////////////////////////////////////////////////////////////////////
    //                                 TEMPORARY                                  //
    // see subsystems/player_data.rs this code will be moved there in the future  //
    ////////////////////////////////////////////////////////////////////////////////

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
                    let mut players: std::sync::MutexGuard<Vec<Player>> = players_clone.lock().unwrap();
                    if let Some(player) = players.iter_mut().find(|p: &&mut Player| p.socket.id == socket.id)
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

    let players_clone: Arc<Mutex<Vec<Player>>> = Arc::clone(&players);
    socket.on(
        "getOnlinePlayers",
        move |socket: SocketRef, _: Data<Value>, _: Bin| {
            info!("Responding with online players list");
            let players: std::sync::MutexGuard<Vec<Player>> = players_clone.lock().unwrap();
            let online_players_json = serde_json::to_value(
                players
                    .iter()
                    .map(|player| json!({ "id": player.socket.id }))
                    .collect::<Vec<_>>(),
            )
            .unwrap();
            debug!("Player Array as JSON: {}", online_players_json);
            socket.emit("onlinePlayers", online_players_json).ok();
        },
    );

    let players_clone: Arc<Mutex<Vec<Player>>> = Arc::clone(&players);

    socket.on(
        "getPlayersWithLocations",
        move |socket: SocketRef, _: Data<Value>, _: Bin| {
            info!("Responding with players and locations list");
            let players: std::sync::MutexGuard<Vec<Player>> = players_clone.lock().unwrap();
            let players_with_locations_json = serde_json::to_value(
                players
                    .iter()
                    .map(|player| json!({ "id": player.socket.id, "location": player.location }))
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

    let players_clone: Arc<Mutex<Vec<Player>>> = Arc::clone(&players);
    socket.on("broadcastMessage", move |Data::<Value>(data), _: Bin| {
        let players: std::sync::MutexGuard<Vec<Player>> = players_clone.lock().unwrap();
        for player in &*players {
            player.socket.emit("broadcastMessage", data.clone()).ok();
        }
    });
}


// This handels redirecting browser users to the master server to see the dashboard
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

    /////////////////////////////
    // SERVER STARTUP SEQUENCE //
    /////////////////////////////

    // Show branding
    subsystems::startup::main();

    // this is in it's own thread so it does not take up the main thread because this task runs
    // throughout the lifetime of the server and would prevent anything else from running
    let _terraforge_thread = spawn(async {
        // this is in it's own thread to not take up the main thread. because otherwise that would
        // be catastrophically bad for performance, because then the tasks would not complete.
        TerraForge::main();
    });
    
    println!("{}", PebbleVault::greet("Rust"));
    let db = PebbleVault::create_db();

    ////////////////////////////////////////////////////////////////////////
    //                              DEBUG ONLY                            //
    // The code below allows for the creation of some test bodies within  //
    // pebblevault, this is normally done automatically by TerraForge.    //
    ////////////////////////////////////////////////////////////////////////
    
    // PebbleVault::create_spatial_index(db, "SpaceBody", "1");
    // PebbleVault::create_galaxy(db, "Galaxy", "Artermis");
    // PebbleVault::create_galaxy(db, "Galaxy", "Athena");
    // PebbleVault::create_galaxy(db, "Galaxy", "Hades");
    // PebbleVault::get_k_nearest_galaxies(db, "Artermis");

    // Define a place to put new players
    let players: Arc<Mutex<Vec<Player>>> = Arc::new(Mutex::new(Vec::new()));
    let (svc, io) = socketioxide::SocketIo::new_svc();
    let players_clone: Arc<Mutex<Vec<Player>>> = players.clone();

    // Handle New player connections
    io.ns("/", move |socket: SocketRef, data: Data<Value>| {
        println!("Player Connected!");
        on_connect(socket, data, players_clone.clone())
    });
    
    //Create a router to handel incoming connections
    let app = Router::new()
        // If the user sends a GET request we redirect them to
        // the master server which hosts the horizon dashboard
        // if the master server itself has a master it too will
        // redirect them until they reach the highest level master
        // server

        .get("/", redirect_to_master_panel)

        // This is an any connection that is not handled above,
        // we cosider these legitimate players and treat their
        // request as them attempting to join the server
        .any("/*", ServiceHandler::new(svc));


    info!("Starting server");
    
    // Define a listener on port 3000
    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    
    // Print any errors encountered while creating the listener
    if let Err(e) = serve(listener, app).await {
        println!("{}", e);
    }
    Ok(())
}
