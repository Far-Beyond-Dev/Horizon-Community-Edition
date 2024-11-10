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

// Server configuration constants
const PLAYERS_PER_POOL: usize = 1000;
const NUM_THREAD_POOLS: usize = 32;

// Initialize global logger
static LOGGER: Lazy<HorizonLogger> = Lazy::new(|| {
    let logger = HorizonLogger::new();
    log_info!(logger, "INIT", "Horizon logger initialized");
    logger
});

#[derive(Clone)]
struct PlayerThreadPool {
    start_index: usize,
    end_index: usize,
    players: Arc<RwLock<Vec<Player>>>,
    sender: mpsc::Sender<PlayerMessage>,
    logger: Arc<HorizonLogger>,  // Added logger
}

enum PlayerMessage {
    NewPlayer(SocketRef, Value),
    RemovePlayer(Uuid),
}

#[derive(Clone)]
struct HorizonServer {
    thread_pools: Arc<Vec<Arc<PlayerThreadPool>>>,
    runtime: Arc<Runtime>,
    logger: Arc<HorizonLogger>,  // Added logger
}

impl HorizonServer {
    fn new() -> Self {
        let runtime = Arc::new(Runtime::new().unwrap());
        let mut thread_pools = Vec::new();
        let logger = Arc::new(HorizonLogger::new());

        log_info!(logger, "SERVER", "Initializing Horizon Server");
        
        for i in 0..NUM_THREAD_POOLS {
            let start_index = i * PLAYERS_PER_POOL;
            let end_index = start_index + PLAYERS_PER_POOL;
            
            let (sender, mut receiver) = mpsc::channel(100);
            let players = Arc::new(RwLock::new(Vec::new()));
            
            let pool = Arc::new(PlayerThreadPool {
                start_index,
                end_index,
                players: players.clone(),
                sender,
                logger: logger.clone(),
            });

            // Load all plugins for this pool
            let my_manager = plugin_test_api::PluginManager::new();
            my_manager.load_all();

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

    async fn handle_message(msg: PlayerMessage, pool: &PlayerThreadPool) {
        match msg {
            PlayerMessage::NewPlayer(socket, data) => {
                socket.emit("connected", &true).ok();
                log_info!(pool.logger, "CONNECTION", "Player {} connected successfully", 
                    socket.id.as_str());

                let id = socket.id.as_str();
                let player: Player = Player::new(socket.clone(), Uuid::new_v4());
                
                players::init(socket.clone(), pool.players.clone());

                pool.players.write().unwrap().push(player.clone());

                log_debug!(pool.logger, "PLAYER", "Player {} (UUID: {}) added to pool", 
                    id, player.id);
                log_debug!(pool.logger, "SOCKET", "Socket.IO namespace: {:?}, id: {:?}", 
                    socket.ns(), socket.id);

                if let Err(e) = socket.emit("preplay", &true) {
                    log_warn!(pool.logger, "EVENT", "Failed to emit preplay event: {}", e);
                }
                
                if let Err(e) = socket.emit("beginplay", &true) {
                    log_warn!(pool.logger, "EVENT", "Failed to emit beginplay event: {}", e);
                }
            },
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

    async fn start(self) {
        let (svc, io) = socketioxide::SocketIo::new_svc();
        
        let server = self.clone();
        io.ns("/", move |socket: SocketRef, data: Data<Value>| {
            let server = server.clone();
            async move {
                server.handle_new_connection(socket, data).await;
            }
        });

        let app = Router::new()
            .get("/", redirect_to_master_panel)
            .any("/*", ServiceHandler::new(svc));

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

async fn redirect_to_master_panel(_req: Request) -> Result<Response> {
    let response = Response::builder()
        .status(302)
        .header("Location", "https://youtu.be/dQw4w9WgXcQ")
        .body(Body::empty())
        .unwrap();
    
    log_info!(LOGGER, "HTTP", "Browser access redirected to master dashboard");
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let init_time = Instant::now();

    // Initialize logger first
    horizon_logger::init();
    log_info!(LOGGER, "STARTUP", "Horizon Server starting...");

    // Create and start server
    let server = HorizonServer::new();
    log_info!(LOGGER, "STARTUP", "Server startup completed in {:?}", init_time.elapsed());
    
    server.start().await;
    
    Ok(())
}