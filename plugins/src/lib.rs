// this file is generated, do not edit manually
// (or edit it if you want, it's not like I can prevent you from doing it)

extern crate plugin_test_api;

#[macro_use] extern crate nom;
mod english;
mod french;
mod greek;
mod core;

pub use english::*;
pub use french::*;
pub use greek::*;
pub use core::*;

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

    Plugins { list: h }
}
