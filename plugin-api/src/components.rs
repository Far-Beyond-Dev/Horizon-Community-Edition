use std::any::Any;
use std::fmt::Debug;
use crate::ApiVersion;

/// Trait that all plugins must implement.
pub trait Plugin: Any + Debug + Send + Sync {
    /// Called when the plugin is loaded. Perform initialization here.
    fn on_load(&self);

    /// Called when the plugin is unloaded. Perform cleanup here.
    fn on_unload(&self);

    /// Executes the plugin's main functionality.
    fn execute(&self);
}

/// Struct representing metadata about the plugin.
#[derive(Debug, Clone)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub api_version: ApiVersion,
}

impl PluginMetadata {
    pub fn new(name: &str, version: &str, description: &str, api_version: ApiVersion) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            description: description.to_string(),
            api_version,
        }
    }
}

/// Type alias for the function used to create a plugin instance.
/// Each plugin DLL must expose this function.
pub type PluginCreateFn = fn() -> Box<dyn Plugin>;