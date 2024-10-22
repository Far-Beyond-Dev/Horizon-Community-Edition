extern crate tokio;
extern crate async_trait;

use std::fmt::Debug;
use std::fmt;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use async_trait::async_trait;
use horizon_data_types::{ Player, PlayerManager };
use ez_logging::println;
use uuid::Uuid;

// Basic types
pub type PlayerId = u64;
pub type ItemId = u32;
pub type Position = (f64, f64, f64);
pub mod components;
pub use components::{Plugin, PluginCreateFn, PluginMetadata};

/// Represents the version of the plugin API.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApiVersion {
    pub major: u32,
    pub minor: u32,
    pub hotfix: u32,
}

impl ApiVersion {
    pub const fn new(major: u32, minor: u32, hotfix: u32) -> Self {
        Self { major, minor, hotfix }
    }
}

/// The current version of the plugin API.
/// Plugins must specify this version in their metadata to ensure compatibility.
pub const PLUGIN_API_VERSION: ApiVersion = ApiVersion::new(0, 1, 0);

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Plugin + 'static> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Event types
pub enum GameEvent {
    None,
    PlayerJoined(Player),
    PlayerLeft(Player),
    ChatMessage { sender: Player, content: String },
    ItemPickup { player: Player, item: ItemId },
    PlayerMoved { player: Player, new_position: Position },
    DamageDealt { attacker: Player, target: Player, amount: f32 },
    Custom(CustomEvent),
}

impl fmt::Debug for GameEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameEvent::None => write!(f, "None"),
            GameEvent::PlayerJoined(player) => write!(f, "PlayerJoined(Player {})", player.id),
            GameEvent::PlayerLeft(player) => write!(f, "PlayerLeft(Player {})", player.id),
            GameEvent::ChatMessage { sender, content } => write!(f, "ChatMessage(Player {}: {})", sender.id, content),
            GameEvent::ItemPickup { player, item } => write!(f, "ItemPickup(Player {} picked up item {})", player.id, item),
            GameEvent::PlayerMoved { player, new_position } => write!(f, "PlayerMoved(Player {} to {:?})", player.id, new_position),
            GameEvent::DamageDealt { attacker, target, amount } => write!(f, "DamageDealt(Attacker {} dealt {} damage to Target {})", attacker.id, amount, target.id),
            GameEvent::Custom(custom_event) => write!(f, "CustomEvent({})", custom_event.event_type),
        }
    }
}

pub struct CustomEvent {
    pub event_type: String,
    pub data: Arc<dyn Any + Send + Sync>,
}

impl Clone for CustomEvent {
    fn clone(&self) -> Self {
        CustomEvent {
            event_type: self.event_type.clone(),
            data: Arc::clone(&self.data),
        }
    }
}

pub trait SayHello {
    fn say_hello(&self) -> String;
}

pub trait PluginInformation {
    fn name(&self) -> String;
    fn get_instance(&self) -> Box<dyn SayHello>;
}

// Configuration trait for plugins
pub trait PluginConfig: Send + Sync {
    fn load(&mut self, config: &str) -> Result<(), String>;
    fn save(&self) -> Result<String, String>;
}

// Logging trait for plugins
pub trait PluginLogger: Send + Sync {
    fn log(&self, level: LogLevel, message: &str);
}

// Log levels
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

// Main plugin trait
#[async_trait]
pub trait BaseAPI: Send + Sync {
    // Define Async methods
    async fn on_game_event(&self, event: &GameEvent);

    async fn on_game_tick(&self, delta_time: f64);

    // Define optional methods with default implementations
    fn get_config(&self) -> Option<&dyn PluginConfig> { None }
    fn get_logger(&self) -> Option<&dyn PluginLogger> { None }

    // Method for dynamic casting
    fn as_any(&self) -> &dyn Any;

    // New methods for custom event support (with default implementations for backward compatibility)
    async fn register_custom_event(&self, _event_type: &str, _context: &mut PluginContext) {}
    async fn emit_custom_event(&self, _event: CustomEvent, _context: &mut PluginContext) {}
}

pub trait PlayersAPI: Send + Sync {
//    async fn get_online_players() -> Vec<Player> {
//        get_online_players().await
//    }
}

// Command handler trait
#[async_trait]
pub trait CommandHandler: Send + Sync {
    async fn handle_command(&self, sender: Player, command: &str, args: Vec<String>, context: &mut PluginContext) -> bool;
}

// Context provided to plugins for interacting with the game server
pub struct PluginContext {
    pub server: Arc<GameServer>,
    pub shared_data: Arc<RwLock<HashMap<String, Box<dyn Any + Send + Sync>>>>,
    pub config: Arc<RwLock<HashMap<String, String>>>,
    pub custom_events: Arc<RwLock<HashMap<String, Vec<Arc<dyn BaseAPI>>>>>,
}

impl Default for PluginContext {
    fn default() -> Self {
        PluginContext {
            server: Arc::new(GameServer::default()),
            shared_data: Arc::new(RwLock::new(HashMap::new())),
            config: Arc::new(RwLock::new(HashMap::new())),
            custom_events: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}


impl PluginContext {
    // Method to register a plugin for a custom event
    pub async fn register_for_custom_event(&mut self, event_type: &str, plugin: Arc<dyn BaseAPI>) {
        let mut custom_events = self.custom_events.write().await;
        custom_events.entry(event_type.to_string())
            .or_insert_with(Vec::new)
            .push(plugin);
    }

    // Method to dispatch a custom event
    pub async fn dispatch_custom_event(&self, event: CustomEvent) {
        let custom_events = self.custom_events.read().await;
        if let Some(handlers) = custom_events.get(&event.event_type) {
            for handler in handlers {
                handler.on_game_event(&GameEvent::Custom(event.clone())).await;
            }
        }
    }
}

// Game server struct (placeholder for actual implementation)
pub struct GameServer {
    // Add relevant game server fields here
}

impl Default for GameServer {
    fn default() -> Self {
        GameServer {}
    }
}


impl GameServer {
    pub async fn broadcast_message(&self, _message: &str) {
        // Implementation for broadcasting a message to all players
    }

    pub async fn get_player(&self, _id: Player) -> Option<Player> {
        // Implementation for retrieving a player by ID
        None
    }

    pub async fn spawn_item(&self, _item: ItemId, _position: Position) {
        // Implementation for spawning an item in the game world
    }

    pub async fn apply_damage(&self, _target: Player, _amount: f32) {
        // Implementation for applying damage to a player
    }

    // New methods for custom event support
    pub async fn register_custom_event(&self, plugin: Arc<dyn BaseAPI>, event_type: &str, context: &mut PluginContext) {
        context.register_for_custom_event(event_type, plugin).await;
    }

    pub async fn emit_custom_event(&self, event: CustomEvent, context: &mut PluginContext) {
        context.dispatch_custom_event(event).await;
    }
}

// Player struct use horizon_data_types::Player in the future
pub struct PlayerDetails {
    pub player: Player,
    pub name: String,
    pub position: Position,
    pub health: f32,
}

// New type alias for RPC functions
pub type RpcFunction = Arc<dyn Fn(&(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync> + Send + Sync>;

#[async_trait]
pub trait RpcPlugin: Send + Sync {
    fn get_id(&self) -> Uuid;
    fn get_name(&self) -> String;
    fn register_rpc(&mut self, name: &str, func: RpcFunction);
    async fn call_rpc(&self, rpc_name: &str, params: &(dyn Any + Send + Sync)) -> Option<Box<dyn Any + Send + Sync>>;
}

pub struct RpcEnabledPlugin {
    id: Uuid,
    name: String,
    rpc_functions: HashMap<String, RpcFunction>,
}

impl RpcEnabledPlugin {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            rpc_functions: HashMap::new(),
        }
    }
}

#[async_trait]
impl RpcPlugin for RpcEnabledPlugin {
    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn register_rpc(&mut self, name: &str, func: RpcFunction) {
        self.rpc_functions.insert(name.to_string(), func);
    }

    async fn call_rpc(&self, rpc_name: &str, params: &(dyn Any + Send + Sync)) -> Option<Box<dyn Any + Send + Sync>> {
        self.rpc_functions.get(rpc_name).map(|func| func(params))
    }
}

// Extension to PluginContext to support RPC plugins
impl PluginContext {
    pub async fn register_rpc_plugin(&mut self, plugin: Arc<RwLock<dyn RpcPlugin>>) {
        let plugin_id = plugin.read().await.get_id();
        let mut shared_data = self.shared_data.write().await;
        shared_data.insert(format!("rpc_plugin_{}", plugin_id), Box::new(plugin));
    }

    pub async fn call_rpc_plugin(&self, plugin_id: Uuid, rpc_name: &str, params: &(dyn Any + Send + Sync)) -> Option<Box<dyn Any + Send + Sync>> {
        let shared_data = self.shared_data.read().await;
        if let Some(plugin) = shared_data.get(&format!("rpc_plugin_{}", plugin_id)) {
            if let Some(rpc_plugin) = plugin.downcast_ref::<Arc<RwLock<dyn RpcPlugin>>>() {
                let plugin = rpc_plugin.read().await;
                return plugin.call_rpc(rpc_name, params).await;
            }
        }
        None
    }

    pub async fn get_rpc_plugin_id_by_name(&self, name: &str) -> Option<Uuid> {
        let shared_data = self.shared_data.read().await;
        for (key, value) in shared_data.iter() {
            if key.starts_with("rpc_plugin_") {
                if let Some(rpc_plugin) = value.downcast_ref::<Arc<RwLock<dyn RpcPlugin>>>() {
                    let plugin = rpc_plugin.read().await;
                    if plugin.get_name() == name {
                        return Some(plugin.get_id());
                    }
                }
            }
        }
        None
    }
}