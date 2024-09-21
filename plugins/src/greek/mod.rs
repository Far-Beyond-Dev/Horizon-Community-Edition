use plugin_test_api::{PluginInformation, SayHello};

pub struct PluginMetadataType;

pub const PLUGIN_METADATA: PluginMetadataType = PluginMetadataType;

impl PluginInformation for PluginMetadataType {
    fn name(&self) -> String {
        "ελληνικά".to_string()
    }

    fn get_instance(&self) -> Box<SayHello> {
        Box::new(Greek)
    }
}

pub struct Greek;

impl SayHello for Greek {
    fn say_hello(&self) -> String {
        "Γεια σε όλους".to_string()
    }
}