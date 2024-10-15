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

// Basic types
pub type PlayerId = u64;
pub type ItemId = u32;
pub type Position = (f32, f32, f32);
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