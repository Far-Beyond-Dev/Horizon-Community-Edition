use std::error::Error;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use socketio_rs::Server;
use socketio_rs::Handshake;
use socketio_rs::Transport;

// Define a trait for the adapter function
trait AdapterTrait: Send + Sync + 'static {
    fn handle_data(&mut self, data: Vec<u8>, sid: &str) -> Result<(), Box<dyn Error>>;
}

// Implement the trait for a struct that holds the adapter logic
struct EchoAdapter;

impl AdapterTrait for EchoAdapter {
    fn handle_data(&mut self, data: Vec<u8>, sid: &str) -> Result<(), Box<dyn Error>> {
        // Echo back the received data
        println!("Received data from {}: {:?}", sid, data);
        Ok(())
    }
}

struct WebSocketServer {
    server: Server<Mutex<HashMap<String, Box<dyn AdapterTrait + 'static>>>>,
    max_clients: usize,
}

impl WebSocketServer {
    fn new(address: &str, max_clients: usize) -> Result<Self, Box<dyn Error>> {
        let mut server = Server::new(address);
        server.register_handler(|handshake, transport| {
            Self::handle_connection(handshake, transport)
        });

        Ok(WebSocketServer {
            server,
            max_clients,
        })
    }

    fn handle_connection(handshake: Handshake, transport: Transport) -> Result<(), Box<dyn Error>> {
        let mut adapters = transport.get_payload::<Mutex<HashMap<String, Box<dyn AdapterTrait + 'static>>>>().lock().unwrap();
        if adapters.len() >= transport.get_payload::<usize>() {
            return Ok(());
        }

        let sid = handshake.session_id().to_string();
        let adapter: Box<dyn AdapterTrait + 'static> = Box::new(EchoAdapter {});
        adapters.insert(sid.clone(), adapter);

        transport.on_data(move |data| {
            let mut adapters = transport.get_payload::<Mutex<HashMap<String, Box<dyn AdapterTrait + 'static>>>>().lock().unwrap();
            if let Some(adapter) = adapters.get_mut(&sid) {
                if let Err(err) = adapter.handle_data(data.to_vec(), &sid) {
                    eprintln!("Error handling data: {}", err);
                }
            }
        });

        Ok(())
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        let payload = Arc::new(Mutex::new(HashMap::new()));
        self.server.set_payload(payload.clone());
        self.server.set_payload(self.max_clients);
        self.server.run()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let server = WebSocketServer::new("127.0.0.1:8080", 10)?;
    server.run()?;
    Ok(())
}