use plugin_test_api::{BaseAPI, GameEvent, Plugin, PluginContext, PluginInformation, PluginMetadata, RpcPlugin, SayHello, PLUGIN_API_VERSION};
use std::{any::Any, sync::Arc};
use async_trait::async_trait;
use crate::{core::PLUGIN_METADATA, recipe_smith::{self, RecipeSmith}};


#[derive(Debug, Clone)]
pub struct UnrealEngine {
}

impl UnrealEngine {
    pub fn new() -> Self {
        UnrealEngine {
        }
    }
}

#[async_trait]
impl BaseAPI for UnrealEngine {
    fn on_game_event(&self, event: &GameEvent) {
        match event {
            GameEvent::PlayerJoined(player) => {
                println!("A new unreal client has joined the game: {}", player.id);
                player.socket.on("ue_move", move || println!("Player moved in Unreal!"));
            }
//            GameEvent::PlayerMoved { player, new_position } => {
//                println!("Stars Beyond: Explorer {} moved to {:?}", player.id, new_position);
//            }
            _ => {}
        }
    }

    async fn on_game_tick(&self, _delta_time: f64) {}

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Plugin for UnrealEngine {
    fn on_load(&self) {
    }

    fn on_unload(&self) {
    }

    fn execute(&self) {
    }

    fn initialize(&self, context: &mut PluginContext) {
    }

    fn shutdown(&self, context: &mut PluginContext) {
    }

    fn on_enable(&self, context: &mut PluginContext) {
    }

    fn on_disable(&self, context: &mut PluginContext) {
    }
}

impl PluginInformation for UnrealEngine {
    fn name(&self) -> String {
        "Stars Beyond".to_string()
    }

    fn get_instance(&self) -> Box<dyn SayHello> {
        Box::new(self.clone())
    }

    fn broadcast_game_event(&self, plugin: & &Box<dyn BaseAPI> ,event:GameEvent) {
        plugin.on_game_event(&event);
    }
    
    fn get_plugin(&self) -> Box<dyn BaseAPI>  {
        Box::new(UnrealEngine::new())
    }
}

impl SayHello for UnrealEngine {
    fn say_hello(&self) -> String {
        format!("Ready for Unreal Engine Integration!")
    }
}

// Instead of a const PLUGIN_METADATA, we'll use a function to create it
pub fn get_plugin() -> UnrealEngine {
    UnrealEngine::new()
}

pub fn get_plugin_metadata() -> PluginMetadata {
    PluginMetadata {
        name: "Unreal Engine".to_string(),
        version: "5.4.1".to_string(),
        description: "A plugin for integrating with the Unreal game engine".to_string(),
        api_version: PLUGIN_API_VERSION,
    }
}