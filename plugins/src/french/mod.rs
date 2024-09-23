use plugin_test_api::{PluginInformation, SayHello};

pub struct PluginMetadataType;

pub const PLUGIN_METADATA: PluginMetadataType = PluginMetadataType;

impl PluginInformation for PluginMetadataType {
    fn name(&self) -> String {
        "français".to_string()
    }

    fn get_instance(&self) -> Box<SayHello> {
        Box::new(French)
    }
}

pub struct French;

impl SayHello for French {
    fn say_hello(&self) -> String {
        "Bonjour, tout le monde".to_string()
    }
}
