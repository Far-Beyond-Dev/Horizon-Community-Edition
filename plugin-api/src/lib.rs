use std::collections::HashMap;
use std::sync::RwLock;

use test_plugin::Plugin_API;

mod plugin_imports;
mod proposal;

// Define the current plugin version
const PLUGIN_API_VERSION: Version = Version {
    major: 0,
    minor: 1,
    hotfix: 0
};

struct Version {
    major: u16,
    minor: u16,
    hotfix: u16,
}

// static PLUGIN_MANAGER: LazyLock<RwLock<PluginManager>>>;
#[derive(PartialEq, Eq,Hash)]
enum Pluginstate {
    ACTIVE,
    INACTIVE,
    CRASH,
}
struct PluginManager {
    plugins: HashMap<String,(Pluginstate,Plugin)>
}

impl PluginManager {
    /// Allow instantiation of the ``PluginManager`` struct
    fn new() -> PluginManager {
        let new_manager = PluginManager {
            plugins: HashMap::new(), 
        };

        new_manager 
    }

    fn load_plugin(mut self,name: String, plugin: Plugin) {
        self.plugins.insert(name, (Pluginstate::ACTIVE, plugin));
    }

    fn unload_plugin(mut self,name: String) {
        self.plugins.remove(&name);
    }

    fn get_plugins(self) -> HashMap<String,(Pluginstate,Plugin)> {
        self.plugins
    }
}

struct Plugin {
    name: String,
    version: Version,
    api_versin: Version
}

impl Plugin {
    pub fn t() {}
}

pub fn load_all() {
    let plugins = plugin_imports::load_plugins();

    let first_plugin = &plugins.get(0).unwrap().instance;

    //    println!("{}", first_plugin)
    //let _ = 
}