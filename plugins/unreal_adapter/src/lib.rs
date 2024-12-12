use horizon_data_types::Player;
use socketioxide::extract::SocketRef;
pub use horizon_plugin_api::{Plugin, Pluginstate, LoadedPlugin, get_plugin, get_type_from_plugin};

use std::sync::RwLock;
use std::sync::Arc;
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref PLUGINS: RwLock<HashMap<&'static str, LoadedPlugin>> = {
        let m = HashMap::new();
        RwLock::new(m)
    };

    static ref PLAYER_LIB: Plugin = {
        get_plugin!(player_lib)
    };
}

// Define the trait properly
pub trait PluginAPI {    
    fn thing(&self) -> String;
    fn player_joined(&self, socket: SocketRef, players: Arc<RwLock<Vec<horizon_data_types::Player>>>);   
}

pub trait PluginConstruct {
    // If you want default implementations, mark them with 'default'
    fn new(plugins: HashMap<&'static str, LoadedPlugin>) -> Plugin;
    
}

// Implement constructor separately
impl PluginConstruct for Plugin {
    fn new(plugins: HashMap<&'static str, LoadedPlugin>) -> Plugin {  
        *PLUGINS.write().unwrap() = plugins;      
        Plugin {}
    }
    
}

// Implement the trait for Plugin
impl PluginAPI for Plugin {
    // Add the thing() method implementation
    fn thing(&self) -> String {
        "Hello from specific plugin implementation!".to_string()
    }

    fn player_joined(&self, socket: SocketRef, players: Arc<RwLock<Vec<horizon_data_types::Player>>>) {        
        setup_listeners(socket, players, PLUGINS.read().unwrap().clone());
    }
}


fn setup_listeners(socket: SocketRef, players: Arc<RwLock<Vec<horizon_data_types::Player>>>, plugins: HashMap<&'static str, LoadedPlugin>) {
    socket.on("player_joined", move || {
        
    });
}