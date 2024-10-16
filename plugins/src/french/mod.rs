use plugin_test_api::{BaseAPI, GameEvent, PluginInformation, SayHello};
use async_trait::async_trait;
use std::any::Any;

// Metadata type for the plugin, providing plugin-specific information.
pub struct PluginMetadataType;

// Constant holding the metadata object for easy access.
pub const PLUGIN_METADATA: PluginMetadataType = PluginMetadataType;

impl PluginInformation for PluginMetadataType {
    fn name(&self) -> String {
        "français".to_string()
    }

    fn get_instance(&self) -> Box<dyn SayHello> {
        Box::new(French)
    }
}

#[async_trait]
impl BaseAPI for PluginMetadataType {
    async fn on_game_event(&self, event: &GameEvent) {
        match event {
            GameEvent::PlayerJoined(player) => {
                println!("Player {} has joined the game. Bienvenue!", player.id);
            }
            GameEvent::ChatMessage {sender, content } => {
                //println!("{} says: {} (in French, we'd say: {}", sender, content, "Bonjour");
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

pub struct French;

impl SayHello for French {
    fn say_hello(&self) -> String {
        "Bonjour, tout le monde".to_string()
    }
}
