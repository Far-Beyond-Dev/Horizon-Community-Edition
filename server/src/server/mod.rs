//-----------------------------------------------------------------------------
// Multi-Threaded SocketIO Server Implementation
//   - Manages multiple threads for handling player connections
//   - Uses SocketIO for real-time communication with clients
//   - Configurable server settings
//   - Logging with Horizon Logger
//   - Server state management with lazy_static
//   - Horizon Server and Horizon Thread structs
//   - Socket event handlers for message and ack events
//   - Server startup with axum web framework
//   - Server configuration with config module
//
//-----------------------------------------------------------------------------
//   Written by: Tristan James Poland, and Caznix
//-----------------------------------------------------------------------------

use crate::LOGGER;
use anyhow::{Context, Result};
use axum::{routing::get, Router};
use config::ServerConfig;
use horizon_data_types::Player;
use horizon_plugin_api::LoadedPlugin;
use std::time::Duration;
use horizon_logger::{log_critical, log_debug, log_error, log_info, log_warn};
use parking_lot::RwLock;
use plugin_api::{Plugin, Pluginstate};
use socketioxide::{
    extract::{AckSender, Data, SocketRef},
    SocketIo,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
pub mod config;
pub mod event_rep;
pub mod vault_lib;

// Server state management

//-----------------------------------------------------------------------------
// Horizon Server Struct
//-----------------------------------------------------------------------------
pub struct HorizonServer {
    config: Arc<ServerConfig>,
    threads: RwLock<Vec<Arc<HorizonThread>>>,
    total_timings: RwLock<HashMap<String, Duration>>,
}

struct Server {
    instance: Arc<RwLock<HorizonServer>>,
}

impl Server {
    fn new() -> Result<Self> {
        Ok(Self {
            instance: Arc::new(RwLock::new(HorizonServer::new()?)),
        })
    }
    fn get_instance(&self) -> Arc<RwLock<HorizonServer>> {
        Arc::clone(&self.instance)
    }
}

impl HorizonServer {
    fn new() -> Result<Self> {
        Ok(Self {
            config: config::server_config().expect("Failed to load server config"),
            threads: RwLock::new(Vec::new()),
            total_timings: RwLock::new(HashMap::new()),
        })
    }

    fn spawn_thread(&self) -> Result<usize> {
        let thread = HorizonThread::new();
        let plugins_loaddata = thread.plugins_loaddata.clone();
        let thread_id = {
            let mut threads = self.threads.write();
            threads.push(thread.into());
            threads.len() - 1
        };

        plugins_loaddata.0.iter().for_each(|(name, _)| {
            plugins_loaddata.1.get(name).map(|duration| {
            let mut total_timings = self.total_timings.write();
            total_timings.entry(name.to_string())
                .and_modify(|e| *e += *duration)
                .or_insert(*duration);
            });
        });

        Ok(thread_id)
    }
}

//-----------------------------------------------------------------------------
// Horizon Thread Structhorizon_plugin_api::Plugin
//-----------------------------------------------------------------------------
struct HorizonThread {
    players: Mutex<Vec<Player>>,
    plugins_loaddata: (HashMap<&'static str, LoadedPlugin>, HashMap<&'static str, Duration>),
    plugins: HashMap<std::string::String, (Pluginstate, Plugin)>,
    handle: tokio::task::JoinHandle<()>,
}

impl HorizonThread {
    fn new() -> Self {
        let plugin_manager = plugin_api::PluginManager::new();
        let plugins_loaddata = &plugin_manager.load_all();
        let plugins = plugin_manager.get_plugins();
        Self {
            players: Mutex::new(Vec::new()),
            plugins: plugins,
            plugins_loaddata: plugins_loaddata.clone(),
            handle: tokio::spawn(async move {
                loop {
                    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
                }
            }),
        }
    }

    fn id(&self) -> usize {
        self.players
            .try_lock()
            .map(|players| players.len())
            .unwrap_or(usize::MAX)
    }

    async fn add_player(&self, player: Player) -> Result<()> {
        let mut players = self.players.lock().await;
        players.push(player);
        Ok(())
    }

    // async fn remove_player(&self, player_id: &str) -> Result<bool> {
    //     let mut players = self.players.lock().await;
    //     if let Some(pos) = players.iter().position(|p| p.id == player_id) {
    //         players.remove(pos);
    //         Ok(true)
    //     } else {
    //         Ok(false)
    //     }
    // }
}

//-----------------------------------------------------------------------------
// Socket event handlers
//-----------------------------------------------------------------------------
async fn handle_socket_message(socket: SocketRef, Data(data): Data<serde_json::Value>) {
    log_debug!(LOGGER, "SOCKET EVENT", "Received message");
    if let Err(e) = socket.emit("message-back", &data) {
        log_error!(LOGGER, "SOCKET EVENT", "Failed to send message back: {}", e);
    }
}

async fn handle_socket_ack(Data(data): Data<serde_json::Value>, ack: AckSender) {
    log_debug!(LOGGER, "SOCKET EVENT", "Received message with ack");
    if let Err(e) = ack.send(&data) {
        log_error!(LOGGER, "SOCKET EVENT", "Failed to send ack: {}", e);
    }
}

fn on_connect(socket: SocketRef, Data(data): Data<serde_json::Value>) {
    log_info!(LOGGER, "SOCKET NET", "New connection from {}", socket.id);

    if let Err(e) = socket.emit("auth", &data) {
        log_error!(LOGGER, "SOCKET NET", "Failed to send auth: {}", e);
        return;
    }

    socket.on("message", handle_socket_message);
    socket.on("message-with-ack", handle_socket_ack);
}

//-----------------------------------------------------------------------------
// Server startup
//-----------------------------------------------------------------------------
pub async fn start() -> anyhow::Result<()> {
    let start_time = std::time::Instant::now();

    let (layer, io) = SocketIo::new_layer();
    let server = Server::new()?;
    let thread_count = config::SERVER_CONFIG
        .get()
        .map(|config: &Arc<ServerConfig>| config.num_thread_pools)
        .unwrap();

    let address = config::SERVER_CONFIG
        .get()
        .map(|config: &Arc<ServerConfig>| config.address.clone())
        .unwrap();

    let port = config::SERVER_CONFIG
        .get()
        .map(|config: &Arc<ServerConfig>| config.port)
        .unwrap();

    // Create futures for spawning threads
    let spawn_futures: Vec<_> = (0..thread_count)
        .map(|_| {
            let server_instance = server.get_instance();
            async move {
                server_instance.read().spawn_thread()
            }
        })
        .collect();

    // Wait for all threads to complete spawning
    let thread_results = futures::future::join_all(spawn_futures).await;
    
    // Now that all threads have completed, we can safely access timing data
    {
        let total_timings = server.instance.read().total_timings.read().clone();
        total_timings.iter().for_each(|(name, duration)| {
            log_info!(LOGGER, "SERVER", "Plugin {} took {:?}", name, duration);
        });
    }

    log_info!(LOGGER, "SERVER", "Spawned {} threads", thread_count);
    let elapsed = start_time.elapsed();
    log_info!(LOGGER, "SERVER", "Server initialization took {:?}", elapsed);

    // Configure socket namespaces and start server
    io.ns("/", on_connect);
    io.ns("/custom", on_connect);
    
    let app = Router::new()
        .route("/", get(|| async { "Horizon Server Running" }))
        .layer(layer);

    let full_address = format!("{}:{}", address, port);
    log_info!(LOGGER, "SOCKET NET", "Starting server on {}", full_address);

    let listener = tokio::net::TcpListener::bind(&full_address)
        .await
        .context(format!("Failed to bind to {}", full_address))?;
    axum::serve(listener, app)
        .await
        .context("Failed to start server")?;
    Ok(())
}