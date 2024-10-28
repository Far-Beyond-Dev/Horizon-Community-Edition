use horizon_data_types::Player;
use plugin_test_api::{BaseAPI, GameEvent, PluginInformation, SayHello};
use async_trait::async_trait;
use std::any::Any;

// Metadata type for the plugin, providing plugin-specific information.
pub struct PluginMetadataType;

// Constant holding the metadata object for easy access.
pub const PLUGIN_METADATA: PluginMetadataType = PluginMetadataType;

impl PluginInformation for PluginMetadataType {
    fn name(&self) -> String {
        "English".to_string()
    }

    fn get_instance(&self) -> Box<dyn SayHello> {
        Box::new(English)
    }

    fn broadcast_game_event(&self, plugin: & &Box<dyn BaseAPI> ,event:GameEvent) {}
    
    fn get_pluginmetadatatype(&self) -> Box<dyn BaseAPI>  {
        todo!()
    }
}

#[async_trait]
impl BaseAPI for PluginMetadataType {
    async fn on_game_event(&self, event: &GameEvent) {
        match event {
            GameEvent::PlayerJoined(player) => {
                println!("Player {} has joined the game. Hello!!", player.id);
            }
            GameEvent::ChatMessage {sender, content } => {
                //println!("{} says: {} (in English, we'd say: {}", sender, content, "Hello World");
            }
            GameEvent::PlayerMoved { player, new_position } => {
                //println!("Player {} moved to postion {:?}", player , new_position);
            }
            _=> {
//              println!("Unhandled game event.");
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

pub struct English;

impl SayHello for English {
    fn say_hello(&self) -> String {
        "Hello World!".to_string()
    }
}
