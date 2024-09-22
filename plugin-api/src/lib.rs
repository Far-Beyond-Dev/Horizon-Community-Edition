extern crate tokio;
extern crate async_trait;

use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use async_trait::async_trait;

// Basic types
pub type PlayerId = u64;
pub type ItemId = u32;
pub type Position = (f32, f32, f32);

// Event types
pub enum GameEvent {
    PlayerJoined(PlayerId),
    PlayerLeft(PlayerId),
    ChatMessage { sender: PlayerId, content: String },
    ItemPickup { player: PlayerId, item: ItemId },
    PlayerMoved { player: PlayerId, new_position: Position },
    DamageDealt { attacker: PlayerId, target: PlayerId, amount: f32 },
    // Add more event types as needed
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
pub trait Plugin: Send + Sync {
    fn name(&self) -> &'static str;
    fn version(&self) -> &'static str;
    fn description(&self) -> &'static str;
    
    async fn initialize(&self, context: &mut PluginContext);
    async fn shutdown(&self, context: &mut PluginContext);
    
    async fn on_enable(&self, context: &mut PluginContext);
    async fn on_disable(&self, context: &mut PluginContext);
    
    async fn on_game_event(&self, event: &GameEvent, context: &mut PluginContext);
    
    async fn on_game_tick(&self, delta_time: f32, context: &mut PluginContext);
    
    fn get_config(&self) -> Option<&dyn PluginConfig> { None }
    fn get_logger(&self) -> Option<&dyn PluginLogger> { None }
    
    fn as_any(&self) -> &dyn Any;
}

// Command handler trait
#[async_trait]
pub trait CommandHandler: Send + Sync {
    async fn handle_command(&self, sender: PlayerId, command: &str, args: Vec<String>, context: &mut PluginContext) -> bool;
}

// Metadata for describing a plugin
#[derive(Debug, Clone)]
pub struct PluginMetadata {
    pub name: &'static str,
    pub version: &'static str,
    pub description: &'static str,
    pub dependencies: Vec<&'static str>,
    pub commands: Vec<&'static str>,
}

// Context provided to plugins for interacting with the game server
pub struct PluginContext {
    pub server: Arc<GameServer>,
    pub shared_data: Arc<RwLock<HashMap<String, Box<dyn Any + Send + Sync>>>>,
    pub config: Arc<RwLock<HashMap<String, String>>>,
}

// Game server struct (placeholder for actual implementation)
pub struct GameServer {
    // Add relevant game server fields here
}

impl GameServer {
    pub async fn broadcast_message(&self, _message: &str) {
        // Implementation for broadcasting a message to all players
    }

    pub async fn get_player(&self, _id: PlayerId) -> Option<Player> {
        // Implementation for retrieving a player by ID
        None
    }

    pub async fn spawn_item(&self, _item: ItemId, _position: Position) {
        // Implementation for spawning an item in the game world
    }

    pub async fn apply_damage(&self, _target: PlayerId, _amount: f32) {
        // Implementation for applying damage to a player
    }
}

// Player struct (placeholder for actual implementation)
pub struct Player {
    pub id: PlayerId,
    pub name: String,
    pub position: Position,
    pub health: f32,
    // Add more player fields as needed
}


/////////////////////////////////////////////////////////
//                      WARNING                        //
//  The components section is depricated and will be   //
//  removed in a future version of the API             //
/////////////////////////////////////////////////////////

pub mod components {
    use std::any::Any;

    pub trait Plugin: Any + Send + Sync {
        fn name(&self) -> &'static str;

        fn version(&self) -> &'static str;

        fn initialize(&self);

        fn execute(&self);
    }

    pub trait AsAny {
        fn as_any(&self) -> &dyn Any;
    }

    impl<T: Plugin+ 'static> AsAny for T {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    // Plugin metadata to descript the plguin.
    #[derive(Debug, Clone)]
    pub struct PluginMetadata {
        pub name: &'static str,
        pub version: &'static str,
        pub description: &'static str,
    }

    pub type PluginCreateFn = fn() -> Box<dyn Plugin>;
    
    #[macro_export]
    macro_rules! declare_plugin {
        ($metadata:expr, %create_fn:expr) => {
            pub fn get_plugin_metadata() -> plugin_api::PluginMetadata {
                metadata
            }

            pub fn create_plugin() -> Box<dyn plugin_api::Plugin> {
                create_fn()
            }
        };
    }
}

