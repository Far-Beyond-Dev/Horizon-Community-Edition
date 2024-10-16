use plugin_test_api::{PluginInformation, SayHello};

pub struct PluginMetadataType;

pub const PLUGIN_METADATA: PluginMetadataType = PluginMetadataType;

impl PluginInformation for PluginMetadataType {
    fn name(&self) -> String {
        "español".to_string()
    }

    fn get_instance(&self) -> Box<SayHello> {
        Box::new(Spanish)
    }
}

pub struct Spanish;

impl SayHello for Spanish {
    fn say_hello(&self) -> String {
        "¡Hola a todos!".to_string()
    }
}
