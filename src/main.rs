use serde_json::{json, Value}; // Import json macro and Value from serde_json
use socketioxide::{
    extract::{AckSender, Bin, Data, SocketRef},
};
use std::sync::{Arc, Mutex};
use tracing::{debug, info};
use tracing_subscriber::FmtSubscriber;
use std::io::Write; // Bring the Write trait into scope
use viz::{handler::ServiceHandler, serve, Result, Router};
use serde::{Serialize, Deserialize};

// Define a struct for Player
#[derive(Debug, Clone)]
struct Player {
    id: String,
    socket: SocketRef,
    location: Option<Location>, // Optional to handle players who haven't sent location updates yet
}

// Define a struct for Rotation
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Rotation {
    w: f64,
    x: f64,
    y: f64,
    z: f64,
}

// Define a struct for Scale
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Scale {
    x: f64,
    y: f64,
    z: f64,
}

// Define a struct for Translation
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Translation {
    x: f64,
    y: f64,
    z: f64,
}

// Define a struct for Location
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Location {
    rotation: Rotation,
    scale3D: Scale, // Update field name to match the JSON data
    translation: Translation,
}

fn on_connect(socket: SocketRef, Data(data): Data<Value>, players: Arc<Mutex<Vec<Player>>>) {
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

    socket.on(
        "UpdatePlayerLocation",
        move |socket: SocketRef, Data::<Value>(data), Bin(bin)| {
            info!("Received event: UpdatePlayerLocation with data: {:?} and bin: {:?}", data, bin);

            // Extract location from data
            match serde_json::from_value::<Location>(data.clone()) {
                Ok(location) => {
                    let mut players = players_clone.lock().unwrap();
                    if let Some(player) = players.iter_mut().find(|p| p.id == socket.id.to_string()) {
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

    let players_clone = Arc::clone(&players);
    socket.on(
        "message-with-ack",
        move |Data::<Value>(data), ack: AckSender, Bin(bin)| {
            info!("Received event: message-with-ack with data: {:?} and bin: {:?}", data, bin);
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
                players.iter().map(|player| json!({ "id": player.id })).collect::<Vec<_>>(),
            ).unwrap();

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
                players.iter().map(|player| {
                    json!({ "id": player.id, "location": player.location })
                }).collect::<Vec<_>>(),
            ).unwrap();

            info!("Players with Locations as JSON: {}", players_with_locations_json);
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::io::stdout().flush().unwrap();
    
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    println!("Starting Horizon Server...");
    println!("");
    println!("+------------------------------------------------------------------------------------------------------------------------------------+");
    println!("|  __    __                      __                                       ______                                                     |");
    println!("| |  |  |  |                    |  |                                     /      |                                                    |");
    println!("| | $$  | $$  ______    ______   |$$ ________   ______   _______        |  $$$$$$|  ______    ______  __     __   ______    ______   |");
    println!("| | $$__| $$ /      |  /      | |  ||        | /      | |       |       | $$___|$$ /      |  /      ||  |   /  | /      |  /      |  |");
    println!("| | $$    $$|  $$$$$$||  $$$$$$|| $$ |$$$$$$$$|  $$$$$$|| $$$$$$$|       |$$    | |  $$$$$$||  $$$$$$||$$| /  $$|  $$$$$$||  $$$$$$| |");
    println!("| | $$$$$$$$| $$  | $$| $$   |$$| $$  /    $$ | $$  | $$| $$  | $$       _|$$$$$$|| $$    $$| $$   |$$ |$$|  $$ | $$    $$| $$   |$$ |");
    println!("| | $$  | $$| $$__/ $$| $$      | $$ /  $$$$_ | $$__/ $$| $$  | $$      |  |__| $$| $$$$$$$$| $$        |$$ $$  | $$$$$$$$| $$       |");
    println!("| | $$  | $$ |$$    $$| $$      | $$|  $$    | |$$    $$| $$  | $$       |$$    $$ |$$     || $$         |$$$    |$$     || $$       |");
    println!("|  |$$   |$$  |$$$$$$  |$$       |$$ |$$$$$$$$  |$$$$$$  |$$   |$$        |$$$$$$   |$$$$$$$ |$$          |$      |$$$$$$$ |$$       |");
    println!("|                                                                 V: 0.0.1-A                                                         |");
    println!("+------------------------------------------------------------------------------------------------------------------------------------+");
    println!("");
    println!("+-----------------------------------------------------------------------------------------+");
    println!("|  ,---.   ,--.                            ,-----.                                   ,--. |");
    println!("| (   .-',-'  '-. ,--,--.,--.--. ,---.     |  |) /_  ,---. ,--. ,--.,---. ,--,--,  ,-|  | |");
    println!("|  `  `-.'-.  .-'| ,-.  ||  .--'(  .-'     |  .-.  || (===) |  '  /| .-. ||  ,,  |' .-. | |");
    println!("|  _)   |  |  |  | '-'  ||  | .-'  `)      |  '--' /|   --.  |   / ' '-' '|  ||  || `-' | |");
    println!("| (____/   `--'   `--`--'`--  `----'       `------'  `----'.-'  /   `---' `--''--' `---'  |");
    println!("|                                    V: 0.0.1-A            `---'                          |");
    println!("+-----------------------------------------------------------------------------------------+");
    println!("");

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
