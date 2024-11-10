//==============================================================================
// Horizon Game Server - Core Implementation
//==============================================================================
// A high-performance, multithreaded game server using Socket.IO for real-time 
// communication. Features include:
//
// - Scalable thread pool architecture supporting up to 32,000 concurrent players
// - Dynamic player connection management with automatic load balancing
// - Integrated plugin system for extensible functionality
// - Comprehensive logging and monitoring
// - Real-time Socket.IO event handling
// - Graceful error handling and connection management
//
// Structure:
// - Player connections are distributed across multiple thread pools
// - Each pool manages up to 1000 players independently
// - Message passing system for inter-thread communication
// - Asynchronous event handling using Tokio runtime
//
// Authors: Tristan James Poland, Thiago M. R. Goulart, Michael Houston
// License: Apache-2.0
//==============================================================================

use horizon_data_types::*;
use horizon_logger::{HorizonLogger, log_info, log_debug, log_warn, log_error, log_critical};
use serde_json::Value;
use socketioxide::extract::{Data, SocketRef};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Instant;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use uuid::Uuid;
use viz::{handler::ServiceHandler, serve, Body, Request, Response, Result, Router};
use once_cell::sync::Lazy;
use plugin_test_api;

mod players;

//------------------------------------------------------------------------------
// Configuration Constants
//------------------------------------------------------------------------------

/// Maximum number of players that can be managed by a single thread pool
const PLAYERS_PER_POOL: usize = 1000;

/// Number of thread pools to create for managing player connections
/// This allows for a total capacity of PLAYERS_PER_POOL * NUM_THREAD_POOLS players
const NUM_THREAD_POOLS: usize = 32;

//------------------------------------------------------------------------------
// Global Logger Configuration
//------------------------------------------------------------------------------

/// Global logger instance using lazy initialization
/// This ensures the logger is only created when first accessed
static LOGGER: Lazy<HorizonLogger> = Lazy::new(|| {
    let logger = HorizonLogger::new();
    log_info!(logger, "INIT", "Horizon logger initialized");
    logger
});

//------------------------------------------------------------------------------
// Thread Pool Structure
//------------------------------------------------------------------------------

/// Represents a thread pool that manages a subset of connected players
/// Uses Arc and RwLock for safe concurrent access across threads
#[derive(Clone)]
struct PlayerThreadPool {
    /// Starting index for this pool's player range
    start_index: usize,
    /// Ending index for this pool's player range
    end_index: usize,
    /// Thread-safe vector containing the players managed by this pool
    players: Arc<RwLock<Vec<Player>>>,
    /// Channel sender for sending messages to the pool's message handler
    sender: mpsc::Sender<PlayerMessage>,
    /// Thread-safe logger instance for this pool
    logger: Arc<HorizonLogger>,
}

/// Messages that can be processed by the player thread pools
enum PlayerMessage {
    /// Message for adding a new player with their socket and initial data
    NewPlayer(SocketRef, Value),
    /// Message for removing a player using their UUID
    RemovePlayer(Uuid),
}

//------------------------------------------------------------------------------
// Main Server Structure
//------------------------------------------------------------------------------

/// Main server structure that manages multiple player thread pools
/// Handles incoming connections and distributes them across available pools
#[derive(Clone)]
struct HorizonServer {
    /// Vector of thread pools, wrapped in Arc for thread-safe sharing
    thread_pools: Arc<Vec<Arc<PlayerThreadPool>>>,
    /// Tokio runtime for handling async operations
    runtime: Arc<Runtime>,
    /// Server-wide logger instance
    logger: Arc<HorizonLogger>,
}

impl HorizonServer {
    /// Creates a new instance of the Horizon Server
    /// Initializes the thread pools and sets up message handling for each
    fn new() -> Self {
        let runtime = Arc::new(Runtime::new().unwrap());
        let mut thread_pools = Vec::new();
        let logger = Arc::new(HorizonLogger::new());

        log_info!(logger, "SERVER", "Initializing Horizon Server");
        
        // Initialize thread pools
        for i in 0..NUM_THREAD_POOLS {
            let start_index = i * PLAYERS_PER_POOL;
            let end_index = start_index + PLAYERS_PER_POOL;
            
            // Create message channel for this pool
            let (sender, mut receiver) = mpsc::channel(100);
            let players = Arc::new(RwLock::new(Vec::new()));
            
            let pool = Arc::new(PlayerThreadPool {
                start_index,
                end_index,
                players: players.clone(),
                sender,
                logger: logger.clone(),
            });

            // Initialize plugin system for this pool
            let my_manager = plugin_test_api::PluginManager::new();
            my_manager.load_all();

            // Spawn dedicated thread for handling this pool's messages
            let pool_clone = pool.clone();
            thread::spawn(move || {
                let rt = Runtime::new().unwrap();
                rt.block_on(async move {
                    while let Some(msg) = receiver.recv().await {
                        Self::handle_message(msg, &pool_clone).await;
                    }
                });
            });

            log_debug!(logger, "THREAD_POOL", "Initialized pool {} with range {}-{}", 
                i, start_index, end_index);
            
            thread_pools.push(pool);
        }

        HorizonServer {
            thread_pools: Arc::new(thread_pools),
            runtime,
            logger,
        }
    }

    /// Handles incoming messages for a specific thread pool
    /// Processes player connections and disconnections
    async fn handle_message(msg: PlayerMessage, pool: &PlayerThreadPool) {
        match msg {
            // Handle new player connection
            PlayerMessage::NewPlayer(socket, data) => {
                // Confirm connection to client
                socket.emit("connected", &true).ok();
                log_info!(pool.logger, "CONNECTION", "Player {} connected successfully", 
                    socket.id.as_str());

                let id = socket.id.as_str();
                let player: Player = Player::new(socket.clone(), Uuid::new_v4());
                
                // Initialize player-specific handlers
                players::init(socket.clone(), pool.players.clone());

                // Add player to pool
                pool.players.write().unwrap().push(player.clone());

                log_debug!(pool.logger, "PLAYER", "Player {} (UUID: {}) added to pool", 
                    id, player.id);
                log_debug!(pool.logger, "SOCKET", "Socket.IO namespace: {:?}, id: {:?}", 
                    socket.ns(), socket.id);

                // Send initialization events to client
                if let Err(e) = socket.emit("preplay", &true) {
                    log_warn!(pool.logger, "EVENT", "Failed to emit preplay event: {}", e);
                }
                
                if let Err(e) = socket.emit("beginplay", &true) {
                    log_warn!(pool.logger, "EVENT", "Failed to emit beginplay event: {}", e);
                }
            },
            // Handle player removal
            PlayerMessage::RemovePlayer(player_id) => {
                let mut players = pool.players.write().unwrap();
                if let Some(pos) = players.iter().position(|p| p.id == player_id) {
                    players.remove(pos);
                    log_info!(pool.logger, "PLAYER", "Player {} removed from pool", player_id);
                } else {
                    log_warn!(pool.logger, "PLAYER", "Failed to find player {} for removal", 
                        player_id);
                }
            }
        }
    }

    /// Handles new incoming socket connections
    /// Assigns the connection to the first available thread pool
    async fn handle_new_connection(&self, socket: SocketRef, data: Data<Value>) {
        match self.thread_pools.iter().find(|pool| {
            let players = pool.players.read().unwrap();
            players.len() < PLAYERS_PER_POOL
        }) {
            Some(selected_pool) => {
                log_info!(self.logger, "CONNECTION", 
                    "Assigning connection {} to thread pool {}", 
                    socket.id.to_string(), 
                    selected_pool.start_index / PLAYERS_PER_POOL);

                if let Err(e) = selected_pool.sender
                    .send(PlayerMessage::NewPlayer(socket, data.0)).await {
                    log_error!(self.logger, "CONNECTION", 
                        "Failed to assign player to pool: {}", e);
                }
            },
            None => {
                log_critical!(self.logger, "CAPACITY", 
                    "All thread pools are full! Cannot accept new connection");
            }
        }
    }

    /// Starts the server and begins listening for connections
    /// Sets up Socket.IO and HTTP routing
    async fn start(self) {
        // Initialize Socket.IO service
        let (svc, io) = socketioxide::SocketIo::new_svc();
        
        let server = self.clone();
        // Configure root namespace handler
        io.ns("/", move |socket: SocketRef, data: Data<Value>| {
            let server = server.clone();
            async move {
                server.handle_new_connection(socket, data).await;
            }
        });

        // Set up HTTP routing
        let app = Router::new()
            .get("/", redirect_to_master_panel)
            .any("/*", ServiceHandler::new(svc));

        // Start server on port 3000
        match tokio::net::TcpListener::bind("0.0.0.0:3000").await {
            Ok(listener) => {
                log_info!(self.logger, "SERVER", 
                    "Multithreaded server listening on 0.0.0.0:3000");
                
                if let Err(e) = serve(listener, app).await {
                    log_critical!(self.logger, "SERVER", "Server error: {}", e);
                }
            },
            Err(e) => {
                log_critical!(self.logger, "SERVER", 
                    "Failed to bind to port 3000: {}", e);
            }
        }
    }
}

/// HTTP handler for redirecting browser access to the master panel
async fn redirect_to_master_panel(_req: Request) -> Result<Response> {
    let response = Response::builder()
        .status(302)
        .header("Location", "https://youtu.be/dQw4w9WgXcQ")
        .body(Body::empty())
        .unwrap();
    
    log_info!(LOGGER, "HTTP", "Browser access redirected to master dashboard");
    Ok(response)
}

/// Main entry point for the Horizon Server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let init_time = Instant::now();

    // Initialize logging system
    horizon_logger::init();
    log_info!(LOGGER, "STARTUP", "Horizon Server starting...");

    // Create and start server instance
    let server = HorizonServer::new();
    log_info!(LOGGER, "STARTUP", "Server startup completed in {:?}", init_time.elapsed());
    
    server.start().await;
    
    Ok(())
}