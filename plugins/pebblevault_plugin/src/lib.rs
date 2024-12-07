use std::collections::HashMap;
pub use horizon_plugin_api::{Plugin, Pluginstate, LoadedPlugin};
use serde::{Serialize, Deserialize};
use PebbleVault::{VaultManager, SpatialObject, VaultRegion};
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use once_cell::sync::Lazy;

/// VaultManager instance for the PebbleVault plugin
static VAULT_MANAGER: Lazy<Arc<Mutex<VaultManager<PebbleVaultCustomData>>>> = 
    Lazy::new(|| {
        let vault_manager = VaultManager::new("./pv-horizon-plugin-data").expect("Failed to create VaultManager");
        Arc::new(Mutex::new(vault_manager))
    });


/// Custom data structure for PebbleVault objects
///
/// This struct represents the custom data associated with each spatial object
/// in the PebbleVault system. It can be extended or modified to suit specific
/// game or application needs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PebbleVaultCustomData {
    /// Name of the object
    pub name: String,
    /// Custom value associated with the object
    pub value: i32,
}

// Define both required traits
pub trait PluginAPI {
    fn new() -> Plugin;
    fn persist_to_disk(&self) -> Result<(), String>;
    fn get_region(&self, region_id: Uuid) -> Option<Arc<Mutex<VaultRegion<PebbleVaultCustomData>>>>;
    fn transfer_player(&self, player_uuid: Uuid, from_region_id: Uuid, to_region_id: Uuid) -> Result<(), String>;
    fn update_object(&self, object: &SpatialObject<PebbleVaultCustomData>) -> Result<(), String>;
    fn get_object(&self, object_id: Uuid) -> Result<Option<SpatialObject<PebbleVaultCustomData>>, String>;
    fn remove_object(&self, object_id: Uuid) -> Result<(), String>;
    fn add_object(&self, region_id: Uuid, uuid: Uuid, object_type: &str, x: f64, y: f64, z: f64, custom_data: PebbleVaultCustomData) -> Result<(), String>;
    fn query_region(&self, region_id: Uuid, min_x: f64, min_y: f64, min_z: f64, max_x: f64, max_y: f64, max_z: f64) -> Result<Vec<SpatialObject<PebbleVaultCustomData>>, String>;
    fn create_or_load_region(&self, center: [f64; 3], radius: f64) -> Result<Uuid, String>;    
    fn thing(&self) -> String;
}

pub trait PluginConstruct {
    fn new(plugins: HashMap<String, (Pluginstate, Plugin)>) -> Plugin;
}

// Implement constructor
impl PluginConstruct for Plugin {
    fn new(plugins: HashMap<String, (Pluginstate, Plugin)>) -> Plugin {
        println!("Initializing PebbleVault plugin");
        Plugin {}
    }
}

// Implement the trait for Plugin
impl PluginAPI for Plugin {    
    // Add the thing() method implementation
    fn thing(&self) -> String {
        self.create_or_load_region([0.0,0.0,0.0], 1000.0).expect("Failure");

        "No String to return".to_string()
    }

        /// Creates a new region or loads an existing one
    ///
    /// This method creates a new spatial region in the PebbleVault system or
    /// loads an existing one if it already exists.
    ///
    /// # Arguments
    ///
    /// * `center` - Center coordinates of the region [x, y, z]
    /// * `radius` - Radius of the region
    ///
    /// # Returns
    ///
    /// A Result containing the UUID of the created or loaded region, or an error string
    fn create_or_load_region(&self, center: [f64; 3], radius: f64) -> Result<Uuid, String> {
        VAULT_MANAGER.lock().unwrap().create_or_load_region(center, radius)
    }

    /// Queries a region for objects within a bounding box
    ///
    /// This method searches for objects within the specified bounding box in a given region.
    ///
    /// # Arguments
    ///
    /// * `region_id` - UUID of the region to query
    /// * `min_x` - Minimum x-coordinate of the bounding box
    /// * `min_y` - Minimum y-coordinate of the bounding box
    /// * `min_z` - Minimum z-coordinate of the bounding box
    /// * `max_x` - Maximum x-coordinate of the bounding box
    /// * `max_y` - Maximum y-coordinate of the bounding box
    /// * `max_z` - Maximum z-coordinate of the bounding box
    ///
    /// # Returns
    ///
    /// A Result containing a vector of SpatialObjects or an error string
    fn query_region(&self, region_id: Uuid, min_x: f64, min_y: f64, min_z: f64, max_x: f64, max_y: f64, max_z: f64) -> Result<Vec<SpatialObject<PebbleVaultCustomData>>, String> {
        VAULT_MANAGER.lock().unwrap().query_region(region_id, min_x, min_y, min_z, max_x, max_y, max_z)
    }

    /// Adds a new object to a region
    ///
    /// This method adds a new spatial object to the specified region in the PebbleVault system.
    ///
    /// # Arguments
    ///
    /// * `region_id` - UUID of the region to add the object to
    /// * `uuid` - UUID of the new object
    /// * `object_type` - Type of the object (e.g., "player", "item")
    /// * `x` - X-coordinate of the object
    /// * `y` - Y-coordinate of the object
    /// * `z` - Z-coordinate of the object
    /// * `custom_data` - Custom data associated with the object
    ///
    /// # Returns
    ///
    /// A Result indicating success or an error string
    fn add_object(&self, region_id: Uuid, uuid: Uuid, object_type: &str, x: f64, y: f64, z: f64, custom_data: PebbleVaultCustomData) -> Result<(), String> {
        VAULT_MANAGER.lock().unwrap().add_object(region_id, uuid, object_type, x, y, z, Arc::new(custom_data))
    }

    /// Removes an object from its region and the persistent database
    ///
    /// This method removes a spatial object from the PebbleVault system.
    ///
    /// # Arguments
    ///
    /// * `object_id` - UUID of the object to remove
    ///
    /// # Returns
    ///
    /// A Result indicating success or an error string
    ///
    fn remove_object(&self, object_id: Uuid) -> Result<(), String> {
        VAULT_MANAGER.lock().unwrap().remove_object(object_id)
    }

    /// Gets a reference to an object by its ID
    ///
    /// This method retrieves a spatial object from the PebbleVault system by its UUID.
    ///
    /// # Arguments
    ///
    /// * `object_id` - UUID of the object to retrieve
    ///
    /// # Returns
    ///
    /// A Result containing an Option with the SpatialObject if found, or an error string
    fn get_object(&self, object_id: Uuid) -> Result<Option<SpatialObject<PebbleVaultCustomData>>, String> {
        VAULT_MANAGER.lock().unwrap().get_object(object_id)
    }

    /// Updates an existing object in the VaultManager's in-memory storage
    ///
    /// This method updates the data of an existing spatial object in the PebbleVault system.
    ///
    /// # Arguments
    ///
    /// * `object` - A reference to the updated SpatialObject
    ///
    /// # Returns
    ///
    /// A Result indicating success or an error string
    fn update_object(&self, object: &SpatialObject<PebbleVaultCustomData>) -> Result<(), String> {
        VAULT_MANAGER.lock().unwrap().update_object(object)
    }

    /// Transfers a player (object) from one region to another
    ///
    /// This method moves a spatial object (typically a player) from one region to another
    /// in the PebbleVault system.
    ///
    /// # Arguments
    ///
    /// * `player_uuid` - UUID of the player to transfer
    /// * `from_region_id` - UUID of the source region
    /// * `to_region_id` - UUID of the destination region
    ///
    /// # Returns
    ///
    /// A Result indicating success or an error string
    fn transfer_player(&self, player_uuid: Uuid, from_region_id: Uuid, to_region_id: Uuid) -> Result<(), String> {
        VAULT_MANAGER.lock().unwrap().transfer_player(player_uuid, from_region_id, to_region_id)
    }

    /// Persists all in-memory databases to disk
    ///
    /// This method saves all spatial data from memory to the persistent storage.
    ///
    /// # Returns
    ///
    /// A Result indicating success or an error string
    fn persist_to_disk(&self) -> Result<(), String> {
        VAULT_MANAGER.lock().unwrap().persist_to_disk()
    }

    /// Gets a reference to a region by its ID
    ///
    /// This method retrieves a reference to a spatial region in the PebbleVault system.
    ///
    /// # Arguments
    ///
    /// * `region_id` - UUID of the region to retrieve
    ///
    /// # Returns
    ///
    /// An Option containing a reference to the region if found, or None if not found
    fn get_region(&self, region_id: Uuid) -> Option<Arc<Mutex<VaultRegion<PebbleVaultCustomData>>>> {
        VAULT_MANAGER.lock().unwrap().get_region(region_id)
    }
    
    fn new() -> Plugin {
        Plugin{}
    }
}