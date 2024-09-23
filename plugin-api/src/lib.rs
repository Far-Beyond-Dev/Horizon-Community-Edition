use std::fmt::Debug;
use std::any::Any;

pub mod components;
pub use components::{Plugin, PluginCreateFn, PluginMetadata};

/// Represents the version of the plugin API.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApiVersion {
    pub major: u32,
    pub minor: u32,
    pub hotfix: u32,
}

impl ApiVersion {
    pub const fn new(major: u32, minor: u32, hotfix: u32) -> Self {
        Self { major, minor, hotfix }
    }
}

/// The current version of the plugin API.
/// Plugins must specify this version in their metadata to ensure compatibility.
pub const PLUGIN_API_VERSION: ApiVersion = ApiVersion::new(0, 0, 0);

/// Trait that allows conversion to `Any` for downcasting purposes.
pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Plugin + 'static> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Function signature that each plugin must expose for retrieving metadata.
/// The plugin must expose this function as `get_plugin_metadata` symbol.
pub type PluginMetadataFn = fn() -> PluginMetadata;

/// Macro to declare a plugin.
/// It requires a `PluginMetadata` instance and a creation function for the plugin.
#[macro_export]
macro_rules! declare_plugin {
    ($metadata:expr, $create_fn:expr) => {
        #[no_mangle]
        pub extern "C" fn get_plugin_metadata() -> plugin_api::PluginMetadata {
            $metadata
        }

        #[no_mangle]
        pub extern "C" fn create_plugin() -> Box<dyn plugin_api::Plugin> {
            $create_fn()
        }
    };
}


pub trait PluginInformation {
    fn name(&self) -> String;
    fn get_instance(&self) -> Box<SayHello>;
}

pub trait SayHello {
    fn say_hello(&self) -> String;
}