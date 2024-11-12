use horizon_data_types::Player;
use socketioxide::extract::SocketRef;
pub use horizon_plugin_api::{Plugin, Pluginstate, LoadedPlugin};
use std::sync::RwLock;
use std::sync::Arc;
use std::collections::HashMap;

// Define the trait properly
pub trait Plugin_API {    
    fn thing(&self) -> String;
    fn player_joined(&self, socket: SocketRef, players: Arc<RwLock<Vec<horizon_data_types::Player>>>);   
}

pub trait Plugin_Construct {
    // If you want default implementations, mark them with 'default'
    fn new(plugins: HashMap<&'static str, LoadedPlugin>) -> Plugin;
    
}

// Implement constructor separately
impl Plugin_Construct for Plugin {
    fn new(plugins: HashMap<&'static str, LoadedPlugin>) -> Plugin {
        println!("Hello from the test plugin!!!!!");
        
        Plugin {}
    }
    
}

// Implement the trait for Plugin
impl Plugin_API for Plugin {
    // Add the thing() method implementation
    fn thing(&self) -> String {
        "Hello from specific plugin implementation!".to_string()
    }

    fn player_joined(&self, socket: SocketRef, players: Arc<RwLock<Vec<horizon_data_types::Player>>>) {        
        setup_listeners(socket, players);
    }
}


fn setup_listeners(socket: SocketRef, players: Arc<RwLock<Vec<horizon_data_types::Player>>>) {
    socket.on("foo", || println!("bar"));
}