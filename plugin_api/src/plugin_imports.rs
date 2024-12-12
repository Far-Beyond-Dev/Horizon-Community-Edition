// This file is automatically generated by build.rs
// Do not edit this file manually!

use horizon_plugin_api::{Pluginstate, LoadedPlugin, Plugin};
use std::collections::HashMap;

pub use chronos_plugin;
pub use chronos_plugin::*;
pub use chronos_plugin::Plugin as chronos_plugin_plugin;
pub use player_lib;
pub use player_lib::*;
pub use player_lib::Plugin as player_lib_plugin;
pub use stars_beyond_plugin;
pub use stars_beyond_plugin::*;
pub use stars_beyond_plugin::Plugin as stars_beyond_plugin_plugin;
pub use unreal_adapter;
pub use unreal_adapter::*;
pub use unreal_adapter::Plugin as unreal_adapter_plugin;
pub use unreal_adapter_horizon;
pub use unreal_adapter_horizon::*;
pub use unreal_adapter_horizon::Plugin as unreal_adapter_horizon_plugin;


// Invoke the macro with all discovered plugins
pub fn load_plugins() -> HashMap<String, (Pluginstate, Plugin)> {
    let plugins = crate::load_plugins!(
        chronos_plugin,
        player_lib,
        stars_beyond_plugin,
        unreal_adapter,
        unreal_adapter_horizon
    );
    plugins
}
