// Welcome to the Horizon Plugin Template! This is the entry point for your plugin.
// Things to keep in mind:
// - The PluginAPI trait is the interface between your plugin and the server, it can be customized to your needs
// - The PluginConstruct trait is the constructor for your plugin, this is pre-defined and should not ever be changed
// - The Plugin struct is the main struct for your plugin
// - The PluginAPI trait is implemented for the Plugin struct
// - The PluginConstruct trait is implemented for the Plugin struct


use horizon_data_types::Player;
use socketioxide::extract::SocketRef;
pub use horizon_plugin_api::{Plugin, Pluginstate, LoadedPlugin};
use parking_lot::RwLock;
use std::sync::Arc;
use std::collections::HashMap;

// Define the trait properly
pub trait PluginAPI {    
    fn player_joined(&self, socket: SocketRef, player: Arc<horizon_data_types::Player>);   
}

pub trait PluginConstruct {
    // If you want default implementations, mark them with 'default'
    fn new(plugins: HashMap<String, (Pluginstate, Plugin)>) -> Plugin;    
    
}

// Implement constructor separately
impl PluginConstruct for Plugin {
    fn new(plugins: HashMap<String, (Pluginstate, Plugin)>) -> Plugin {        
        Plugin {}
    }
}

// Implement the trait for Plugin
impl PluginAPI for Plugin {
    fn player_joined(&self, socket: SocketRef, player: Arc<Player>) {        
        println!("Welcome Player {} to Unreal Engine Server!", socket.id.to_string());
        setup_listeners(socket.clone(), player);
    }
}


fn setup_listeners(socket: SocketRef, player: Arc<Player>) {
    socket.on("foo", || println!("bar"));
}