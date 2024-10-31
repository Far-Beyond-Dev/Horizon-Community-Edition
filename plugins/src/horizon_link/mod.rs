use plugin_test_api::{BaseAPI, GameEvent, Plugin, PluginContext, PluginInformation, RpcPlugin, RpcFunction, SayHello};
use serde_json::Value;
use socketioxide::extract::Data;
use async_trait::async_trait;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::any::Any;
use std::error::Error as StdError;

// Type alias for our event handler function
type EventHandler = dyn Fn(Value) -> Result<(), Box<dyn StdError>> + Send + Sync;
type HandlerFn = Arc<EventHandler>;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct EventWrapper {
    #[serde(rename = "event-name")]
    event_name: String,
    #[serde(rename = "event-origin")]
    event_origin: Vec<i32>,
    #[serde(rename = "event-propagation")]
    event_propagation: i32,
    data: Value,
}

#[derive(Debug, Clone)]
struct EventMetadata {
    name: String,
    origin: Vec<i32>,
    propagation: i32,
}

#[derive(Default)]
struct MetadataStore {
    events: RwLock<HashMap<String, Vec<EventMetadata>>>,
}

impl MetadataStore {
    fn new() -> Self {
        Self {
            events: RwLock::new(HashMap::new()),
        }
    }

    fn store_metadata(&self, metadata: EventMetadata) {
        let mut events = self.events.write().unwrap();
        println!("Intercepted Event Meta!");
        println!("  Event Name:   {}", metadata.name);
        println!("  Event Origin: X:{} y:{} z:{}", metadata.origin.get(0).unwrap().to_string(), metadata.origin.get(1).unwrap().to_string(), metadata.origin.get(2).unwrap().to_string());
        println!("  Event Distance: {}", metadata.propagation.to_string());
        println!("-------------------------- End Event Data -------------------------");
    }
}

// Type alias for RPC parameters to ensure consistency
type RpcParams = (String, Arc<EventHandler>);

#[derive(Clone)]
pub struct HorizonLinkPlugin {
    id: Uuid,
    name: String,
    rpc_functions: HashMap<String, RpcFunction>,
    metadata_store: Arc<MetadataStore>,
    message_handlers: Arc<RwLock<HashMap<String, HandlerFn>>>,
}

impl HorizonLinkPlugin {
    pub fn new() -> Self {
        let mut plugin = Self {
            id: Uuid::new_v4(),
            name: "Horizon Link".to_string(),
            rpc_functions: HashMap::new(),
            metadata_store: Arc::new(MetadataStore::new()),
            message_handlers: Arc::new(RwLock::new(HashMap::new())),
        };

        let plugin_clone = plugin.clone();
        // Inside the rpc_fn
        let rpc_fn: RpcFunction = Arc::new(move |params| {
            if let Some(params_tuple) = params.downcast_ref::<RpcParams>() {
                let event_name = params_tuple.0.clone();
                let handler = Arc::clone(&params_tuple.1);
                plugin_clone.rpc_listen(event_name, handler);
            }
            Box::new(())
        });

        plugin.register_rpc("listen", rpc_fn);
        plugin
    }

    fn rpc_listen(&self, event: String, handler: HandlerFn) {
        let metadata_store = self.metadata_store.clone();
        let wrapped_handler: HandlerFn = Arc::new(move |data: Value| {
            if let Ok(event_data) = serde_json::from_value::<EventWrapper>(data.clone()) {
                let metadata = EventMetadata {
                    name: event_data.event_name.clone(),
                    origin: event_data.event_origin.clone(),
                    propagation: event_data.event_propagation,
                };

                metadata_store.store_metadata(metadata);
                handler(event_data.data)?;
            }
            Ok(())
        });

        self.message_handlers
            .write()
            .unwrap()
            .insert(event, wrapped_handler);
    }

    fn process_event(&self, event_name: &str, data: Value) -> Result<(), Box<dyn StdError>> {
        if let Some(handler) = self.message_handlers.read().unwrap().get(event_name) {
            handler(data)?;
        }
        Ok(())
    }
}

#[async_trait]
impl BaseAPI for HorizonLinkPlugin {
    fn on_game_event(&self, event: &GameEvent) {
        match event {
            GameEvent::PlayerJoined(player) => {
                let s = player.socket.clone();
                let plugin = self.clone();

                s.on("link_echo", move |d: Data<Value>| {
                    if let Ok(event_data) = serde_json::from_value::<EventWrapper>(d.0.clone()) {
                        if let Err(e) = plugin.process_event(&event_data.event_name, d.0) {
                            eprintln!("Error processing event: {}", e);
                        }
                    }
                });
            }
            _ => {}
        }
    }

    async fn on_game_tick(&self, _delta_time: f64) {}

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[async_trait]
impl RpcPlugin for HorizonLinkPlugin {
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

impl Plugin for HorizonLinkPlugin {
    fn on_load(&self) {}
    fn on_unload(&self) {}
    fn execute(&self) {}
    fn initialize(&self, _context: &mut PluginContext) {}
    fn shutdown(&self, _context: &mut PluginContext) {}
    fn on_enable(&self, _context: &mut PluginContext) {}
    fn on_disable(&self, _context: &mut PluginContext) {}
}

impl PluginInformation for HorizonLinkPlugin {
    fn name(&self) -> String {
        "Horizon Link".to_string()
    }

    fn get_instance(&self) -> Box<dyn SayHello> {
        Box::new(self.clone())
    }
    
    fn get_plugin(&self) -> Box<dyn BaseAPI> {
        Box::new(HorizonLinkPlugin::new())
    }
    
    fn broadcast_game_event(&self, _plugin: &&Box<dyn BaseAPI>, _event: GameEvent) {
    }
}

impl SayHello for HorizonLinkPlugin {
    fn say_hello(&self) -> String {
        "Preparing to link all the things...".to_string()
    }
}

pub fn get_plugin() -> HorizonLinkPlugin {
    HorizonLinkPlugin::new()
}