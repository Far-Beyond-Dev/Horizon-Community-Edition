extern crate plugin_test_api;

mod english;
mod french;
mod greek;
mod core;
mod stars_beyond;
//mod pebble_vault;
mod recipe_smith;
// mod unit_test;

use plugin_test_api::PluginInformation;
use std::collections::hash_map::HashMap;

pub struct Plugins {
    pub list: HashMap<String, Box<dyn PluginInformation>>,
}

pub fn plugins() -> Plugins {
    let mut h: HashMap<String, Box<dyn PluginInformation>> = HashMap::new();

    h.insert("english".to_string(), Box::new(english::PLUGIN_METADATA));
    h.insert("french".to_string(), Box::new(french::PLUGIN_METADATA));
    h.insert("greek".to_string(), Box::new(greek::PLUGIN_METADATA));
    h.insert("core".to_string(), Box::new(core::PLUGIN_METADATA));
    h.insert("stars_beyond".to_string(), Box::new(stars_beyond::get_plugin()));
    h.insert("recipe_smith".to_string(), Box::new(recipe_smith::create_plugin_metadata()));
    //h.insert("pebble_vault".to_string(), Box::new(pebble_vault::create_plugin_metadata()));
    //h.insert("unit_test".to_string(), Box::new(unit_test::get_plugin(100)));

    Plugins { list: h }
}

// If you need to export specific items from each module, do so explicitly:
pub use english::PluginMetadataType as EnglishPluginMetadataType;
pub use french::PluginMetadataType as FrenchPluginMetadataType;
pub use greek::PluginMetadataType as GreekPluginMetadataType;
pub use core::PluginMetadataType as CorePluginMetadataType;

// Export other necessary items
pub use stars_beyond::{get_plugin as get_stars_beyond_plugin, get_plugin_metadata as get_stars_beyond_metadata};