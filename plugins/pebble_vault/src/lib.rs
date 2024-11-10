use std::collections::HashMap;
use horizon_data_types::Player;
use socketioxide::extract::SocketRef;
pub use horizon_plugin_api::{Plugin, Pluginstate, LoadedPlugin};
use std::sync::RwLock;
use std::sync::Arc;
use PebbleVault::{VaultManager, SpatialObject, VaultRegion};

// Define the trait properly
pub trait Plugin_API {    
    fn thing(&self) -> String;
}

pub trait Plugin_Construct {
    // If you want default implementations, mark them with 'default'
    fn new(plugins: HashMap<&'static str, LoadedPlugin>) -> Plugin;
}

// Implement constructor separately
impl Plugin_Construct for Plugin {
    fn new(plugins: HashMap<&'static str, LoadedPlugin>) -> Plugin {
        println!("Hello from the PebbleVault plugin!!!!!");
//        setup_listeners(socket, players);

        Plugin {}
    }
}

// Implement the trait for Plugin
impl Plugin_API for Plugin {    
    // Add the thing() method implementation
    fn thing(&self) -> String {
        "Hello from specific plugin implementation!".to_string()
    }
}


fn setup_listeners(socket: SocketRef, players: Arc<RwLock<Vec<horizon_data_types::Player>>>) {
    socket.on("foo", || println!("bar"));
}