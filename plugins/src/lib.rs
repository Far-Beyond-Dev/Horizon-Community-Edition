//  extern crate plugin_test_api;
//  
//  mod core;
//  mod stars_beyond;
//  mod horizon_link;
//  mod unreal_engine;
//  
//  //mod pebble_vault;
//  mod recipe_smith;
//  
//  use plugin_test_api::PluginInformation;
//  use std::collections::hash_map::HashMap;
//  
//  pub struct Plugins {
//      pub list: HashMap<String, Box<dyn PluginInformation>>,
//  }
//  
//  pub fn plugins() -> Plugins {
//      let mut h: HashMap<String, Box<dyn PluginInformation>> = HashMap::new();
//  
//      h.insert("core".to_string(), Box::new(core::PLUGIN_METADATA));
//      h.insert("stars_beyond".to_string(), Box::new(stars_beyond::get_plugin()));
//      h.insert("horizon_link".to_string(), Box::new(horizon_link::get_plugin()));
//      h.insert("recipe_smith".to_string(), Box::new(recipe_smith::create_plugin_metadata()));
//      h.insert("unreal_engine".to_string(), Box::new(unreal_engine::get_plugin()));
//      //h.insert("pebble_vault".to_string(), Box::new(pebble_vault::create_plugin_metadata()));
//      //h.insert("unit_test".to_string(), Box::new(unit_test::get_plugin(100)));
//  
//      Plugins { list: h }
//  }
//  
//  pub use core::PluginMetadataType as CorePluginMetadataType;