// Import required dependencies
use horizon_data_types::*;
use serde_json::Value;
use socketioxide::extract::{Data, SocketRef};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Instant;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use tracing::info;
use uuid::Uuid;
use viz::{handler::ServiceHandler, serve, Body, Request, Response, Result, Router};
use std::backtrace::Backtrace;

mod players;

// Server configuration constants
// These values can be adjusted based on expected server load and available resources
const PLAYERS_PER_POOL: usize = 10;  // Maximum number of players per thread pool
const NUM_THREAD_POOLS: usize = 4;     // Number of thread pools to create

/// Represents a thread pool that handles a specific range of players.
/// This structure enables horizontal scaling by distributing player load across multiple threads.
#[derive(Clone)]
struct PlayerThreadPool {
    start_index: usize,         // Starting index of player range for this pool
    end_index: usize,          // Ending index of player range for this pool
    players: Arc<RwLock<Vec<Player>>>,  // Thread-safe vector of players managed by this pool
    sender: mpsc::Sender<PlayerMessage>,  // Channel for sending messages to this pool's worker thread
}

/// Defines the types of messages that can be sent between threads.
/// This enum facilitates type-safe communication between the main server and worker threads.
enum PlayerMessage {
    NewPlayer(SocketRef, Value),  // Message for adding a new player with their socket and data
    RemovePlayer(Uuid),          // Message for removing a player by their UUID
}

/// Main server structure that manages all player connections and game state.
/// Uses Arc (Atomic Reference Counting) to safely share the server state across threads.
#[derive(Clone)]
struct HorizonServer {
    thread_pools: Arc<Vec<Arc<PlayerThreadPool>>>,  // Vector of thread pools for handling players
    runtime: Arc<Runtime>,                          // Tokio runtime for async operations
}

impl HorizonServer {
    /// Creates a new instance of the game server with initialized thread pools.
    /// This function sets up the entire server infrastructure before any players connect.
    fn new() -> Self {
        let runtime = Arc::new(Runtime::new().unwrap());
        let mut thread_pools = Vec::new();

        // Initialize multiple thread pools for handling player connections
        // This improves performance by distributing player load across multiple threads
        for i in 0..NUM_THREAD_POOLS {
            let start_index = i * PLAYERS_PER_POOL;
            let end_index = start_index + PLAYERS_PER_POOL;
            
            // Create a channel for sending messages to this pool's worker thread
            let (sender, mut receiver) = mpsc::channel(100);
            let players = Arc::new(RwLock::new(Vec::new()));
            
            let pool = Arc::new(PlayerThreadPool {
                start_index,
                end_index,
                players: players.clone(),
                sender,
            });

            // Spawn a dedicated worker thread for this pool
            // Each worker thread processes messages for its assigned players
            let pool_clone = pool.clone();
            thread::spawn(move || {
                let rt = Runtime::new().unwrap();
                rt.block_on(async move {
                    while let Some(msg) = receiver.recv().await {
                        Self::handle_message(msg, &pool_clone).await;
                    }
                });
            });

            thread_pools.push(pool);
        }

        HorizonServer {
            thread_pools: Arc::new(thread_pools),
            runtime: runtime,
        }
    }

    /// Handles incoming messages for player management within each thread pool.
    /// This function processes both new player connections and player removals.
    async fn handle_message(msg: PlayerMessage, pool: &PlayerThreadPool) {
        match msg {
            PlayerMessage::NewPlayer(socket, data) => {
                // Notify client that connection was successful
                socket.emit("connected", &true).ok();
                println!("Sent player connected to client!");

                let id = socket.id.as_str();
                println!("Welcome player {} to the game!", id);
                
                // Create and initialize new player
                let player = Player::new(socket.clone(), Uuid::new_v4());
                players::init(socket.clone(), pool.players.clone());
                pool.players.write().unwrap().push(player.clone());

                // Log debug information for troubleshooting
                // This includes a backtrace to help diagnose any connection issues
                //    let msg = Backtrace::force_capture();
                //    println!("-----BACKTRACE-----");
                //    println!("{}", msg);
                //    println!("---END BACKTRACE---");

                println!("Player {} added to players list for socket id: {}", 
                    player.id.to_string(), id);
                println!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);

                // Send initialization events to client
                // These events trigger the client to start game setup
                let _ = socket.emit("preplay", &true);
                socket.emit("beginplay", &true).ok();
            },
            PlayerMessage::RemovePlayer(player_id) => {
                // Remove player from the pool when they disconnect
                let mut players = pool.players.write().unwrap();
                if let Some(pos) = players.iter().position(|p| p.id == player_id) {
                    players.remove(pos);
                }
            }
        }
    }

    /// Handles new player connections by assigning them to an appropriate thread pool.
    /// This function implements basic load balancing across thread pools.
    async fn handle_new_connection(&self, socket: SocketRef, data: Data<Value>) {
        // Find the first thread pool that isn't at capacity
        let selected_pool = self.thread_pools
            .iter()
            .find(|pool| {
                let players = pool.players.read().unwrap();
                players.len() < PLAYERS_PER_POOL
            })
            .expect("All thread pools are full");

        // Send the new player message to the selected pool
        println!("Assigning connection: {} to thread pool: {}", socket.id.to_string(), (selected_pool.start_index / PLAYERS_PER_POOL).to_string());
        selected_pool.sender.send(PlayerMessage::NewPlayer(socket, data.0)).await.ok(); 
    }

    /// Starts the game server and begins listening for connections.
    /// This function initializes the Socket.IO service and HTTP router.
    async fn start(self) {
        // Initialize Socket.IO service
        let (svc, io) = socketioxide::SocketIo::new_svc();
        
        // Set up connection handler
        let server = self.clone();
        io.ns("/", move |socket: SocketRef, data: Data<Value>| {
            let server = server.clone();
            async move {
                server.handle_new_connection(socket, data).await;
            }
        });

        // Set up HTTP router with redirect for browser access
        let app = Router::new()
            .get("/", redirect_to_master_panel)
            .any("/*", ServiceHandler::new(svc));

        // Start server on port 3000
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        println!("Multithreaded server listening on 0.0.0.0:3000");
        
        if let Err(e) = serve(listener, app).await {
            println!("Server error: {}", e);
        }
    }
}

/// Redirects browser users to the master server dashboard.
/// This provides a friendly redirect instead of showing a blank page or error.
async fn redirect_to_master_panel(_req: Request) -> Result<Response> {
    let response = Response::builder()
        .status(302)
        .header("Location", "https://youtu.be/dQw4w9WgXcQ")
        .body(Body::empty())
        .unwrap();
    println!("Browser access redirected to master dashboard");
    Ok(response)
}

/// Main entry point for the game server.
/// Initializes and starts the server, measuring startup time for monitoring purposes.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let init_time = Instant::now();

    // Create and start the server
    let server = HorizonServer::new();
    println!("Server startup took: {:?}", init_time.elapsed());
    server.start().await;
    
    Ok(())
}