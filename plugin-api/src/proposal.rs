use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::OnceLock;
use std::sync::Arc;
use std::panic::AssertUnwindSafe;
use horizon_data_types::Player;
static PLUGIN_MANAGER: OnceLock<RwLock<PluginManager>> = OnceLock::new();

// Define the current plugin version
const PLUGIN_API_VERSION: Version = Version {
    major: 0,
    minor: 1,
    hotfix: 0,
};

struct Version {
    major: u16,
    minor: u16,
    hotfix: u16,
}

// static PLUGIN_MANAGER: LazyLock<RwLock<PluginManager>>>;
#[derive(PartialEq, Eq, Hash, Debug)]
enum Pluginstate {
    ACTIVE,
    INACTIVE,
    CRASH,
}

pub struct PluginManager {
    plugins: HashMap<String, (Pluginstate, PluginInstance)>,
    api_version: Version,
}

// Event types
pub enum GameEvent {
    None,
    PlayerJoined(Player),
    PlayerLeft(Player),
    ChatMessage { sender: Player, content: String },
    PlayerMoved { player: Player }
}

impl PluginManager {
    /// Allow instantiation of the ``PluginManager`` struct
    fn new() -> PluginManager {
        let new_manager = PluginManager {
            plugins: HashMap::new(),
            api_version: PLUGIN_API_VERSION,
        };
        new_manager
    }

    fn load_plugin(&mut self, name: &str, plugin: PluginInstance) {
        self.plugins.insert(name.to_string(), (Pluginstate::ACTIVE, plugin));
        println!("Loaded plugin: {}", name);
    }

    fn unload_plugin(&mut self, name: &str) {
        //self.plugins.remove(&name);
        if let Some((state,_plugin_instance)) = self.plugins.get_mut(name) {
            let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
                panic!("im panicking")
            }));
            match result {
                Ok(_) => {}
                Err(e) => {println!("Plugin '{name}' crashed: {e:#?}")}
            }
        }
    }

    fn disable_plugin(&mut self, name: String) {
        if let Some((state, _plugin_instance)) = self.plugins.get_mut(&name) {
            *state = Pluginstate::INACTIVE;
            println!("Unloaded plugin: {}", name);
        } else {
            println!("Plugin {} not found", name);
        }
    }
}

struct PluginInstance {
    name: String,
    version: Version,
    api_versin: Version,
    plugin: Arc<dyn Plugin + Send + Sync>,
}

pub fn plugin_manager() -> &'static RwLock<PluginManager> {
    PLUGIN_MANAGER.get_or_init(|| { RwLock::new(PluginManager::new()) })
}

trait Plugin {
    fn hello(&self) {}
    fn on_event(&mut self) {}
    fn on_exit_request(&mut self) {}
}

///////////////////////////////////////////////////////////////////////////////
//                                   Tests                                   //
//  Lets test the API and make sure it is working properly.                  //
///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn load_plugin() {
        struct TestPlugin {}
        impl Plugin for TestPlugin {
            fn on_event(&mut self) {}
            fn hello(&self) {
                println!("Hello from testplugin!")
            }
        }
        let plugin_instance = PluginInstance {
            name: "TestPlugin".to_string(),
            version: Version { major: 0, minor: 1, hotfix: 0 },
            api_versin: PLUGIN_API_VERSION,
            plugin: Arc::new(TestPlugin {}),
        };

        plugin_manager().write().unwrap().load_plugin("TestPlugin", plugin_instance);
        assert!(plugin_manager().read().unwrap().plugins.contains_key("TestPlugin"))
    }

    #[test]
    fn disable_plugin() {
        struct TestPlugin {}
        impl Plugin for TestPlugin {
            fn on_event(&mut self) {}
            fn hello(&self) {
                println!("Hello from testplugin!")
            }
        }
        let plugin_instance = PluginInstance {
            name: "TestPlugin".to_string(),
            version: Version { major: 0, minor: 1, hotfix: 0 },
            api_versin: PLUGIN_API_VERSION,
            plugin: Arc::new(TestPlugin {}),
        };

        // insert manually to not depend on load_plugin test
        plugin_manager()
            .write()
            .unwrap()
            .plugins.insert("TestPlugin".to_string(), (Pluginstate::ACTIVE, plugin_instance));
        plugin_manager().write().unwrap().disable_plugin("TestPlugin".to_string());
        assert_eq!(
            plugin_manager().read().unwrap().plugins.get("TestPlugin").unwrap().0,
            Pluginstate::INACTIVE
        );
    }

    #[test]
    fn unload_plugin() {
        struct TestPlugin {}
        impl Plugin for TestPlugin {
            fn on_event(&mut self) {}
            fn hello(&self) {
                println!("Hello from testplugin!")
            }
        }
        let plugin_instance = PluginInstance {
            name: "TestPlugin".to_string(),
            version: Version { major: 0, minor: 1, hotfix: 0 },
            api_versin: PLUGIN_API_VERSION,
            plugin: Arc::new(TestPlugin {}),
        };

        // insert manually to not depend on load_plugin test
        let mut plugin_write = plugin_manager().write().unwrap();
        plugin_write.load_plugin("TestPlugin", plugin_instance);
        plugin_write.unload_plugin("TestPlugin");
        assert_eq!(true,true) // are we still alive?
    }
}


//  FOR CAZ \/  Should the below items be defined in a helper crate that we
//              install in each plugin to prevent circular dep issues?


// register listener


// unregister listener

//register emitter

//unregister emitter