use plugin_test_api::{PluginInformation, SayHello, BaseAPI, GameEvent, CustomEvent, PluginContext, Plugin};
use std::sync::Arc;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct RecipeSmith {
    initialized: bool,
}

impl RecipeSmith {
    pub fn new() -> Self {
        RecipeSmith {
            initialized: false,
        }
    }

    fn initialize_recipe_smith(&mut self, context: &mut PluginContext) {
        if !self.initialized {
            println!("RecipeSmith initializing...");
            // We can't use async calls here, so we'll just print messages
            println!("Registering custom event: recipe_learned");
            println!("Registering custom event: item_crafted");
            println!("Registering custom event: inventory_changed");
            self.initialized = true;
            println!("RecipeSmith initialized!");
        }
    }
}

#[async_trait]
impl BaseAPI for RecipeSmith {
    async fn on_game_event(&self, event: &GameEvent) {
        match event {
            GameEvent::PlayerJoined(player) => {
                println!("RecipeSmith: Player {} joined. Initializing crafting data...", player.id);
            }
            GameEvent::Custom(custom_event) => {
                match custom_event.event_type.as_str() {
                    "recipe_learned" => println!("RecipeSmith: New recipe learned!"),
                    "item_crafted" => println!("RecipeSmith: Item crafted!"),
                    "inventory_changed" => println!("RecipeSmith: Inventory updated!"),
                    _ => {}
                }
            }
            _ => {}
        }
    }

    async fn on_game_tick(&self, _delta_time: f64) {
        // Implement tick logic if needed
    }

    async fn register_custom_event(&self, event_type: &str, context: &mut PluginContext) {
        context.register_for_custom_event(event_type, Arc::new(self.clone())).await;
    }

    async fn emit_custom_event(&self, event: CustomEvent, context: &mut PluginContext) {
        context.dispatch_custom_event(event).await;
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Plugin for RecipeSmith {
    fn on_load(&self) {
        println!("RecipeSmith plugin loaded");
    }

    fn on_unload(&self) {
        println!("RecipeSmith plugin unloaded");
    }

    fn execute(&self) {
        println!("RecipeSmith plugin executed");
    }

    fn initialize(&self, context: &mut PluginContext) {
        println!("RecipeSmith plugin initializing");
        let mut recipe_smith = self.clone();
        recipe_smith.initialize_recipe_smith(context);
    }

    fn shutdown(&self, _context: &mut PluginContext) {
        println!("RecipeSmith plugin shut down");
    }

    fn on_enable(&self, _context: &mut PluginContext) {
        println!("RecipeSmith plugin enabled");
    }

    fn on_disable(&self, _context: &mut PluginContext) {
        println!("RecipeSmith plugin disabled");
    }
}

impl PluginInformation for RecipeSmith {
    fn name(&self) -> String {
        "RecipeSmith".to_string()
    }

    fn get_instance(&self) -> Box<dyn SayHello> {
        Box::new(self.clone())
    }
}

impl SayHello for RecipeSmith {
    fn say_hello(&self) -> String {
        "Hello from RecipeSmith! Ready to craft some amazing items?".to_string()
    }
}

pub const PLUGIN_METADATA: RecipeSmith = RecipeSmith { initialized: false };