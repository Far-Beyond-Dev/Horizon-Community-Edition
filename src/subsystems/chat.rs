//    //////////////////////////////////////
//    // Lets import some tools we need.  //
//    //////////////////////////////////////
//    
//    use socketioxide::extract::{SocketRef, Data};
//    use serde::Deserialize;
//    use tracing::info;
//    use crate::define_event;
//    
//    ////////////////////////////////////////////////
//    // Next we will define some structs our code  //
//    // will use to store and organize data.       //
//    ////////////////////////////////////////////////
//    
//    #[derive(Deserialize)]
//    struct ChatMessage {
//        sender: String,
//        recipient: Option<String>,
//        message: String,
//    }
//    
//    /////////////////////////////////////////////////
//    //  Next we will define what happens when our  //
//    //  subsystem starts, this usually happens     //
//    //  when a new player connects, so we will     //
//    //  define our event listeners here.           //
//    /////////////////////////////////////////////////
//    
//    pub fn init(socket: SocketRef) {
//        info!("Starting chat subsystem...");
//    
//        define_event!(socket, "whisper", handle_whisper(socket, "", "", ""));
//        define_event!(socket, "broadcast", handle_broadcast(socket, "", ""));
//        define_event!(socket, "command", handle_help(socket, ""));
//    
//    }
//    
//    ////////////////////////////////////////////////////
//    //  Finally we define the functions that will be  //
//    //  called by our event listeners when they are   //
//    //  triggered.                                    //
//    ////////////////////////////////////////////////////
//    
//    fn handle_whisper(socket: SocketRef, sender: &str, recipient: &str, message: &str) {
//        info!("Whisper from {} to {}: {}", sender, recipient, message);
//        // Example of emitting a message to a specific recipient
//        socket.emit("whisper", (recipient, message)).ok();
//    }
//    
//    fn handle_help(socket: SocketRef, sender: &str) {
//        info!("Help requested by {}", sender);
//        // Example of emitting a help message to the sender
//        socket.emit("help", "Here's some help!").ok();
//    }
//    
//    fn handle_broadcast(socket: SocketRef, sender: &str, message: &str) {
//        info!("Broadcast from {}: {}", sender, message);
//        // Example of broadcasting a message to all connected clients
//        socket.broadcast().emit("broadcast", message).ok();
//    }