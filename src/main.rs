/*!
 * Horizon Child Server
 *
 * This server software is part of a distributed system designed to facilitate communication
 * and data transfer between multiple child servers and a master server. Each child server
 * operates within a "Region map" managed by the master server, which keeps track of their
 * coordinates in a relative cubic light-year space. The coordinates are stored in 64-bit floats
 * to avoid coordinate overflow and to ensure high precision.
 *
 * The system works as follows:
 *
 * 1. Event Handling:
 *    - The child server receives events from the master server. Each event contains its origin,
 *      data, and a propagation distance, which determines how far the event should spread.
 *
 * 2. Event Propagation:
 *    - Upon receiving an event, the child server calculates which neighboring child servers
 *      should receive the event based on the event's origin and the specified propagation distance.
 *    - This calculation considers all adjacent coordinates within a 3x3x3 cube centered on the
 *      server's coordinate, ensuring that all relevant neighbors are included.
 *
 * 3. Event Transmission:
 *    - After determining the target neighbors, the child server sends the event to the master server.
 *      The master server then multicasts the event to the appropriate neighboring child servers.
 *
 * 4. Coordinate Management:
 *    - Each child server maintains its position in the region map, identified by a coordinate
 *      (x, y, z). The coordinates are managed as integers, representing the position in the cubic
 *      light-year space.
 *    - The child server can calculate the network address of neighboring servers based on their
 *      coordinates, allowing for direct communication.
 *
 * The key components of this system include:
 *
 * - Event: Represents an event with an origin, data, and propagation distance.
 * - Coordinate: Represents a position in the region map.
 * - ChildServer: Represents a child server with methods to receive, handle, and send events.
 *
 *
 * Usage:
 * - The child server is initialized with a unique ID, its coordinate, the master server's address,
 *   and its own local address.
 * - The server then enters a loop, continuously receiving and handling events.
 *
 * This implementation uses `serde` and `bincode` crates for serialization and deserialization of
 * events to ensure efficient data transfer.
 */

///////////////////////////////////////////
// Import a few things to get us started //
///////////////////////////////////////////

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use socketioxide::extract::{AckSender, Bin, Data, SocketRef};
use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex};
use tokio::main;
use tracing::{debug, info};
use tracing_subscriber::FmtSubscriber;
use viz::{handler::ServiceHandler, serve, Result, Router};

mod events;
mod macros;
mod utilities;

/////////////////////////////////////////////
// Define some structs to be used later on //
/////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug)]
struct Event {
    origin: (f64, f64, f64),
    data: String,
    propagation_distance: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Coordinate {
    x: i64,
    y: i64,
    z: i64,
}

struct ChildServer {
    id: u64,
    coordinate: Coordinate,
    parent_addr: SocketAddr,
    socket: UdpSocket,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// * The `ChildServer` struct contains methods for:                                               //
// - Initializing the server with its ID, coordinate, parent server address, and local address.   //
// - Receiving events from the master server.                                                     //
// - Determining which neighboring servers should receive an event.                               //
// - Sending events to the parent server for further multicast.                                   //
// - Running the server to continuously listen for and handle events.                             //
////////////////////////////////////////////////////////////////////////////////////////////////////

impl ChildServer {
    fn new(
        id: u64,
        coordinate: Coordinate,
        parent_addr: SocketAddr,
        local_addr: SocketAddr,
    ) -> Self {
        let socket = UdpSocket::bind(local_addr).expect("Couldn't bind to address");
        ChildServer {
            id,
            coordinate,
            parent_addr,
            socket,
        }
    }

    fn receive_event(&self) -> Event {
        let mut buf = [0u8; 1024];
        let (amt, _src) = self
            .socket
            .recv_from(&mut buf)
            .expect("Didn't receive data");
        let event: Event = bincode::deserialize(&buf[..amt]).expect("Failed to deserialize event");
        event
    }

    fn determine_propagation(&self, event: &Event) -> Vec<Coordinate> {
        let mut neighbors = Vec::new();
        let max_distance = event.propagation_distance;

        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    let neighbor = Coordinate {
                        x: self.coordinate.x + x,
                        y: self.coordinate.y + y,
                        z: self.coordinate.z + z,
                    };

                    let distance = ((neighbor.x as f64 - event.origin.0).powi(2)
                        + (neighbor.y as f64 - event.origin.1).powi(2)
                        + (neighbor.z as f64 - event.origin.2).powi(2))
                    .sqrt();

                    if distance <= max_distance {
                        neighbors.push(neighbor);
                    }
                }
            }
        }
        neighbors
    }

    fn send_event(&self, event: &Event, target: &Coordinate) {
        let msg = bincode::serialize(event).expect("Failed to serialize event");
        let addr = self.calculate_addr(target);
        self.socket
            .send_to(&msg, addr)
            .expect("Failed to send event");
    }

    fn calculate_addr(&self, target: &Coordinate) -> SocketAddr {
        // Implement your logic to calculate the socket address of the target coordinate
        // This is a placeholder, you need to provide the actual mapping from coordinate to address
        SocketAddr::new("127.0.0.1".parse().unwrap(), 8080)
    }

    fn handle_event(&self, event: Event) {
        let neighbors = self.determine_propagation(&event);

        for neighbor in neighbors {
            self.send_event(&event, &neighbor);
        }
    }

    fn run(&self) {
        loop {
            let event = self.receive_event();
            self.handle_event(event);
        }
    }
}

// Define a struct for Player
#[derive(Debug, Clone)]
struct Player {
    id: String,
    socket: SocketRef,
    location: Option<Location>, // Optional to handle players who haven't sent location updates yet
}

////////////////////////////////////////////////////
//            World object structs:               //
// These Structs help store an object's location  //
// this server's coordanites in the instance grid //
// Define a struct for Rotation of objects        //
////////////////////////////////////////////////////

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Rotation {
    w: f64,
    x: f64,
    y: f64,
    z: f64,
}

// Define a struct for Scale of objects
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Scale {
    x: f64,
    y: f64,
    z: f64,
}

// Define a struct for Translation of objects
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Translation {
    x: f64,
    y: f64,
    z: f64,
}

// Define a struct for Location of objects
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Location {
    rotation: Rotation,
    scale3D: Scale, // Update field name to match the JSON data
    translation: Translation,
}

//////////////////////////////////////////////////////////////
//                         WARNING                          //
// on_connect runs every time a new player connects to the  //
// avoid putting memory hungry code here if possible!       //
//////////////////////////////////////////////////////////////
///
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
