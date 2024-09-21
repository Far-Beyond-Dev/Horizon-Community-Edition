use plugin_test_api::{PluginInformation, SayHello};

pub struct PluginMetadataType;

pub const PLUGIN_METADATA: PluginMetadataType = PluginMetadataType;

impl PluginInformation for PluginMetadataType {
    fn name(&self) -> String {
        "english".to_string()
    }

    fn get_instance(&self) -> Box<SayHello> {
        Box::new(English)
    }
}

pub struct English;

impl SayHello for English {
    fn say_hello(&self) -> String {
        "hello, world".to_string()
    }
}
