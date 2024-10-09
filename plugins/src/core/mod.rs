// Import plugins API components
use plugin_test_api::{BaseAPI, PlayersAPI, GameEvent, Plugin, PluginContext, PluginInformation, SayHello};
use async_trait::async_trait;
use socketioxide::socket;
use std::{any::Any, task::Context};
use horizon_data_types::Player;

// Metadata type for the plugin, providing plugin-specific information.
pub struct PluginMetadataType;

// Register Custom Events
pub mod horizon_core;
pub mod chat;

// Define and expose plugin meta
pub const PLUGIN_METADATA: PluginMetadataType = PluginMetadataType;

// Implement the plugin API calls for Horizon Core

impl PluginInformation for PluginMetadataType {
    fn name(&self) -> String {
        "Horizon Core".to_string()
    }

    fn get_instance(&self) -> Box<dyn SayHello + 'static> {
        Box::new(Horizon_Core)
    }
}

#[async_trait]
impl BaseAPI for PluginMetadataType {
    async fn on_game_event(&self, event: &GameEvent) {
        match event {   
            // Set up listeners in all modules
            GameEvent::PlayerJoined(player) => {
                let players = PlayersAPI::get_online_players().await;
                horizon_core::init_all(player.socket, players);
            }
            _=> {
                // Unhandled events ignored
            }
        }
    }

    async fn on_game_tick(&self, delta_time: f64) {
        println!("Game tick with delta time: {:.2} secums", delta_time);
    }

    fn as_any(&self) ->  &dyn Any {
        self
    }
}

pub struct Horizon_Core;

impl SayHello for Horizon_Core {
    fn say_hello(&self) -> String {
        "Hello World!".to_string()
    }
}