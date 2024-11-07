use std::collections::HashMap;

use horizon_data_types::Player;
use socketioxide::{extract::SocketRef, socket};
pub use test_plugin;
use std::sync::RwLock;
use std::sync::Arc;

mod plugin_imports;
mod proposal;

macro_rules! get_plugin {
    ($name:ident, $plugins:expr) => {
        $plugins
            .get(stringify!($name))
            .map(|p| &p.instance as &dyn $name::Plugin_API)
            .expect(&format!("Plugin {} not found", stringify!($name)))
    };
}

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

pub fn load_all(socket: SocketRef, players: Arc<RwLock<Vec<horizon_data_types::Player>>>) {
    let plugins = plugin_imports::load_plugins(socket, players);

    let my_test_plugin = get_plugin!(test_plugin, plugins);
    let result = my_test_plugin.thing();
    println!("{}", result);
}