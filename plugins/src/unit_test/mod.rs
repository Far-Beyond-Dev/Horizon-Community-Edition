use plugin_test_api::{PluginInformation, SayHello, BaseAPI, GameEvent, PluginContext, Plugin, PluginMetadata, PLUGIN_API_VERSION, CustomEvent};
use std::sync::Arc;
use async_trait::async_trait;
use tokio::time::{interval, Duration};
use horizon_data_types::Player;
use socketioxide::extract::SocketRef;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::sync::Mutex;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct UnitTestPlugin {
    player_count: usize,
    event_sender: Arc<Mutex<Option<tokio::sync::mpsc::Sender<GameEvent>>>>,
    active_players: Arc<Mutex<Vec<Player>>>,
}

impl UnitTestPlugin {
    pub fn new(player_count: usize) -> Self {
        println!("UnitTestPlugin::new called with player_count: {}", player_count);
        UnitTestPlugin {
            player_count,
            event_sender: Arc::new(Mutex::new(None)),
            active_players: Arc::new(Mutex::new(Vec::new())),
        }
    }

    async fn generate_random_event(&self) -> GameEvent {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let count = COUNTER.fetch_add(1, Ordering::SeqCst);
        let mut rng = rand::thread_rng();

        let active_players = self.active_players.lock().await;
        let player_count = active_players.len();

        if player_count == 0 || (player_count < self.player_count && rng.gen_bool(0.1)) {
            // Create a new player
            let dummy_socket = unsafe { std::mem::zeroed::<SocketRef>() };
            let new_player = Player::new(dummy_socket, format!("Player{}", count));
            drop(active_players);
            self.active_players.lock().await.push(new_player.clone());
            return GameEvent::PlayerJoined(new_player);
        }

        let player_index = rng.gen_range(0..player_count);
        let player = active_players[player_index].clone();

        match rng.gen_range(0..5) {
            0 if player_count > self.player_count / 2 => {
                // Remove a player
                drop(active_players);
                let mut active_players = self.active_players.lock().await;
                active_players.remove(player_index);
                GameEvent::PlayerLeft(player)
            }
            1 => GameEvent::ChatMessage {
                sender: player,
                content: format!("Random message {}", count),
            },
            2 => GameEvent::PlayerMoved {
                player,
                new_position: (rng.gen(), rng.gen(), rng.gen()),
            },
            3 => GameEvent::Custom(CustomEvent {
                event_type: format!("custom_event_{}", count),
                data: Arc::new(format!("Custom data {}", count)),
            }),
            _ => GameEvent::PlayerMoved {
                player,
                new_position: (rng.gen(), rng.gen(), rng.gen()),
            },
        }
    }
}

#[async_trait]
impl BaseAPI for UnitTestPlugin {
    async fn on_game_event(&self, event: &GameEvent) {
        println!("UnitTestPlugin received event: {:?}", event);
    }

    async fn on_game_tick(&self, delta_time: f64) {
        println!("UnitTestPlugin tick: {}", delta_time);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Plugin for UnitTestPlugin {
    fn on_load(&self) {
        println!("UnitTestPlugin on_load called");
    }

    fn on_unload(&self) {
        println!("UnitTestPlugin on_unload called");
    }

    fn execute(&self) {
        println!("UnitTestPlugin execute called");
    }

    fn initialize(&self, context: &mut PluginContext) {
        println!("UnitTestPlugin initialize called");
        let (tx, mut rx) = tokio::sync::mpsc::channel(100);
        let event_sender = self.event_sender.clone();
        
        tokio::spawn(async move {
            println!("Setting up event sender");
            *event_sender.lock().await = Some(tx);
        });

        let plugin = self.clone();
        
        tokio::spawn(async move {
            println!("Starting event generation loop");
            let mut interval = interval(Duration::from_millis(100));
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        let event = plugin.generate_random_event().await;
                        if let Some(tx) = &*plugin.event_sender.lock().await {
                            println!("Sending event: {:?}", event);
                            let _ = tx.send(event).await;
                        }
                    }
                    Some(event) = rx.recv() => {
                        println!("Received event to process: {:?}", event);
                    }
                }
            }
        });

        // Set up a separate task for broadcasting messages
        let server = context.server.clone();
        tokio::spawn(async move {
            println!("Starting broadcast loop");
            let mut interval = interval(Duration::from_secs(5));
            loop {
                interval.tick().await;
                println!("Broadcasting message");
                server.broadcast_message("UnitTestPlugin is generating events").await;
            }
        });
    }

    fn shutdown(&self, _context: &mut PluginContext) {
        println!("UnitTestPlugin shutdown called");
    }

    fn on_enable(&self, _context: &mut PluginContext) {
        println!("UnitTestPlugin on_enable called");
    }

    fn on_disable(&self, _context: &mut PluginContext) {
        println!("UnitTestPlugin on_disable called");
    }
}

impl PluginInformation for UnitTestPlugin {
    fn name(&self) -> String {
        "UnitTestPlugin".to_string()
    }

    fn get_instance(&self) -> Box<dyn SayHello> {
        Box::new(self.clone())
    }
}

impl SayHello for UnitTestPlugin {
    fn say_hello(&self) -> String {
        println!("UnitTestPlugin say_hello called");
        format!("Hello from UnitTestPlugin! Simulating events for up to {} players.", self.player_count)
    }
}

pub fn get_plugin(player_count: usize) -> UnitTestPlugin {
    println!("get_plugin called with player_count: {}", player_count);
    UnitTestPlugin::new(player_count)
}

pub fn get_plugin_metadata() -> PluginMetadata {
    println!("get_plugin_metadata called");
    PluginMetadata {
        name: "UnitTestPlugin".to_string(),
        version: "1.0.0".to_string(),
        description: "A plugin for simulating player events and testing plugin interactions".to_string(),
        api_version: PLUGIN_API_VERSION,
    }
}