use horizon_data_types::Player;
use socketioxide::extract::SocketRef;
pub use horizon_plugin_api::Plugin;
use std::sync::RwLock;
use std::sync::Arc;

// Define the trait properly
pub trait Plugin_API {    
    fn thing(&self) -> String;
}

pub trait Plugin_Construct {
    // If you want default implementations, mark them with 'default'
    fn new(socket: SocketRef, players: Arc<RwLock<Vec<horizon_data_types::Player>>>) -> Plugin;
}

// Implement constructor separately
impl Plugin_Construct for Plugin {
    fn new(socket: SocketRef, players: Arc<RwLock<Vec<horizon_data_types::Player>>>) -> Plugin {
        println!("Hello from the test plugin!!!!!");
        setup_listeners(socket, players);

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