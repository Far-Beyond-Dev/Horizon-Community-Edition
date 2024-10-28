use plugin_test_api::{RpcFunction, PluginContext};

mod structs;

pub use structs::*;


/// Creates and returns a new instance of the RecipeSmith plugin.
///
/// This function is used to create the plugin metadata and instantiate
/// the RecipeSmith plugin for use in the game engine.
///
/// # Returns
/// A new RecipeSmith instance.
///
/// # Example
/// ```
/// let recipe_smith_plugin = create_plugin_metadata();
/// // Use recipe_smith_plugin to register with the game engine
/// ```
pub fn create_plugin_metadata() -> RecipeSmith {
    RecipeSmith::new()
}