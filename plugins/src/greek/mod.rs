use plugin_test_api::{PluginInformation, SayHello, BaseAPI, GameEvent};

pub struct PluginMetadataType;

pub const PLUGIN_METADATA: PluginMetadataType = PluginMetadataType;

impl PluginInformation for PluginMetadataType {
    fn name(&self) -> String {
        "ελληνικά".to_string()
    }

    fn get_instance(&self) -> Box<dyn SayHello> {
        Box::new(Greek)
    }

    fn broadcast_game_event(&self, plugin: & &Box<dyn BaseAPI> ,event:GameEvent) {}
    
    fn get_pluginmetadatatype(&self) -> Box<dyn BaseAPI>  {
        todo!()
    }
}

pub struct Greek;

impl SayHello for Greek {
    fn say_hello(&self) -> String {
        "Γεια σε όλους".to_string()
    }
}