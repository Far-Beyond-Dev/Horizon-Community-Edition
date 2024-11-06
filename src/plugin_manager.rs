use libloading::{Library, Symbol};
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use notify::{Watcher,RecursiveMode, Config, PollWatcher, Event, Error, EventKind};
use std::sync::mpsc::{channel, Receiver};
// use crate::plugin_api::{Plugin, PluginCreateFn, PluginMetadata};
use std::ffi::OsStr;
use std::sync::{RwLock, RwLockReadGuard};
use std::ops::Deref;

// pub struct PluginManager {
//     plugins: Arc<RwLock<HashMap<String, Box<dyn Plugin>>>>,
//     libraries: Arc<RwLock<HashMap<String, Library>>>,
// }


impl PluginManager {
    pub fn new() -> Self {
        PluginManager {
            plugins: Arc::new(RwLock::new(HashMap::new())),
            libraries: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn contains_plugin(&self, plugin_name: &str) -> bool {
        self.plugins.read().unwrap().contains_key(plugin_name)
    }

    pub fn add_plugin(&self, name: String, plugin: Box<dyn Plugin>) {
        self.plugins.write().unwrap().insert(name, plugin);
    }

    // get a ref to a plugin
    pub fn get_plugin<'a>(&'a self, name: &str) -> Option<impl std::ops::Deref<Target = dyn Plugin> + 'a> {
        struct PluginRef<'a> {
            guard: std::sync::RwLockReadGuard<'a, HashMap<String, Box<dyn Plugin>>>,
            name: String,
        }

        impl<'a> std::ops::Deref for PluginRef<'a> {
            type Target = dyn Plugin;

            fn deref(&self) -> &Self::Target {
                self.guard.get(&self.name).unwrap().as_ref()
            }
        }

        let guard = self.plugins.read().unwrap();
        if guard.contains_key(name) {
            Some(PluginRef {
                guard,
                name: name.to_string(),
            })
        } else {
            None
        }
    }

    pub unsafe fn load_plugin<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        let path: &OsStr = path.as_ref().as_os_str();
        
        let lib = Library::new(path).map_err(|e| e.to_string())?;
   
        let metadata: Symbol<fn() -> PluginMetadata> = lib.get(b"get_plugin_metadata")
            .map_err(|e| e.to_string())?;
        let create: Symbol<PluginCreateFn> = lib.get(b"create_plugin")
            .map_err(|e| e.to_string())?;
   
        let plugin_metadata = metadata();
        let plugin_name = plugin_metadata.name.clone();
       
        if self.plugins.read().unwrap().contains_key(&plugin_name) {
            return Err(format!("Plugin '{}' is already loaded.", plugin_name));
        }
   
        let plugin = create();
   
        println!(
            "Loaded plugin: {} (v{})",
            plugin_metadata.name, plugin_metadata.version
        );
   
        // Removed unnecessary boxing
        self.plugins.write().unwrap().insert(plugin_name.clone(), plugin);
        self.libraries.write().unwrap().insert(plugin_name, lib);
   
        Ok(())
    }

    pub fn unload_plugin(&self, name: &str) -> Result<(), String> {
        if self.plugins.write().unwrap().remove(name).is_some() {
            if let Some(lib) = self.libraries.write().unwrap().remove(name) {
                drop(lib);
                println!("Unloaded Plugin: {}", name);
                Ok(())
            } else {
                Err(format!("Error: Library for plugin '{}' not found, but plugin was removed. This is an inconsistent state.", name))
            }
        } else {
            Err(format!("Error: Plugin '{}' is not loaded.", name))
        }
    }

    pub unsafe fn reload_plugin<P: AsRef<Path>>(&mut self, path: P, name: &str) -> Result<(), String> {
        self.unload_plugin(name)?;
        self.load_plugin(path)?;
        println!("Reloaded plugin: {}", name);
        Ok(())
    }

    pub fn execute_plugin(&self, name: &str) {
        if let Some(plugin) = self.plugins.read().unwrap().get(name) {
            plugin.execute();
        } else {
            println!("Plugin with name '{}' not found.", name);
        }
    }


    /// Loads all plugins from the specified directory.
    pub unsafe fn load_plugins_from_directory<P: AsRef<Path>>(&mut self, directory: P) -> Result<(), String> {
        let dir_path = directory.as_ref();
        if !dir_path.is_dir() {
            return Err(format!("{} is not a valid Directory.", dir_path.display()));
        }

        let entries = std::fs::read_dir(dir_path).map_err(|e| e.to_string())?;

        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if self.is_plugin_file(&path) {
                    if let Err(e) = self.load_plugin(&path) {
                        eprintln!("Failt do load plugin from {}: {}", path.display(), e);
                    }
                }
            }
        }
        Ok(())
    }

    /// Checks if a given file is in the correct format to be a plugin
    fn is_plugin_file(&self, path: &PathBuf) -> bool {
        if let Some(extension) = path.extension() {
            let ext = extension.to_string_lossy().to_lowercase();
            ext == "dll" || ext == "so" || ext == "dylib"
        } else {
            false
        }
    }

    /// Monitors the plugin directory for changes and reloads plugins as needed.
    pub fn monitor_directory_for_changes<P: AsRef<Path>>(&mut self, directory: P) -> Result<Receiver<Result<Event, Error>>, String> {
        let dir_path = directory.as_ref();
        if !dir_path.is_dir() {
            return Err(format!("{} is not a valid directory.", dir_path.display()));
        }

         let (tx, rx) = channel();
         // let mut watcher = watcher(tx, Duration::from_secs(2)).map_err(|e| e.to_string())?;
         let mut watcher = match PollWatcher::new(tx, Config::default()) {
            Ok(w) => w,
            Err(e) => return Err(format!("Error on watcher {}", e))
         };
         watcher.watch(dir_path, RecursiveMode::NonRecursive).map_err(|e| e.to_string())?;

         Ok(rx)
         }

        /// Handles the events received from the directory mon and reloads plugins as needed
    pub unsafe fn handle_directory_events(&mut self, rx: Receiver<Result<Event, Error>>) {
        loop {
            match rx.recv() {
                Ok(Ok(event)) => match event.kind {
                    EventKind::Create(_) | EventKind::Modify(_) => {
                        let path = event.paths.get(0).cloned();
                        if let Some(path) = path {
                            if self.is_plugin_file(&path) {
                            let name = path.file_stem().unwrap().to_string_lossy().to_string();
                            if self.reload_plugin(&path, &name).is_err() {
                                println!("Failed to reload plugin '{}'", name);
                            }
                        }
                    }
                },
                        EventKind::Remove(_) => {
                    let path = event.paths.get(0).cloned();
                    if let Some(path) = path {
                        if self.is_plugin_file(&path) {
                            let name = path.file_stem().unwrap().to_string_lossy().to_string();
                            if self.unload_plugin(&name).is_err() {
                                println!("Failed to unload plugin '{}'", name);
                            }
                        }
                    }
                },
                _ => println!("Other event: {:?}", event),
                },
                Ok(Err(e)) => println!("Error: {:?}", e),
                Err(e) => println!("Channel receive error: {:?}", e),
            }
        }

    }
}