//! # PebbleVault Plugin
//!
//! PebbleVault is a spatial database and object management system for game worlds.
//! It provides efficient storage, retrieval, and querying of spatial data, making it
//! ideal for managing large-scale game environments.
//!
//! ## Features
//!
//! - Efficient spatial indexing using R-trees
//! - Support for custom object data
//! - Persistent storage with SQLite backend
//! - Region-based partitioning for improved performance
//! - RPC interface for integration with other systems
//!
//! ## Usage
//!
//! To use PebbleVault in your game or application, create an instance of the `PebbleVault`
//! struct and use its methods to manage spatial data. The plugin can be integrated into
//! the game engine using the provided trait implementations for `Plugin`, `BaseAPI`,
//! `RpcPlugin`, and `PluginInformation`.

/// # Examples
///
/// Here are some comprehensive examples of how to use the PebbleVault plugin:
///
/// ```rust
/// use uuid::Uuid;
/// use pebble_vault::{PebbleVault, PebbleVaultCustomData};
///
/// // Create a new PebbleVault instance
/// let pebble_vault = PebbleVault::new().expect("Failed to create PebbleVault");
///
/// // Create two regions
/// let region1_id = pebble_vault.create_or_load_region([0.0, 0.0, 0.0], 1000.0)
///     .expect("Failed to create region 1");
/// let region2_id = pebble_vault.create_or_load_region([2000.0, 0.0, 0.0], 1000.0)
///     .expect("Failed to create region 2");
///
/// // Add objects to the regions
/// let player_id = Uuid::new_v4();
/// let item_id = Uuid::new_v4();
///
/// let player_data = PebbleVaultCustomData {
///     name: "Player One".to_string(),
///     value: 100,
/// };
/// pebble_vault.add_object(region1_id, player_id, "player", 10.0, 20.0, 30.0, player_data)
///     .expect("Failed to add player");
///
/// let item_data = PebbleVaultCustomData {
///     name: "Magic Sword".to_string(),
///     value: 50,
/// };

use plugin_test_api::{BaseAPI, GameEvent, Plugin, PluginContext, PluginInformation, PluginMetadata, RpcPlugin, SayHello, PLUGIN_API_VERSION};
use std::{any::Any, sync::Arc};
use async_trait::async_trait;
use ez_logging::println;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use PebbleVault::{VaultManager, SpatialObject, VaultRegion};
use std::sync::Mutex;
use horizon_data_types::Player;

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

/// Main struct for the PebbleVault plugin
///
/// This struct encapsulates the core functionality of the PebbleVault system,
/// providing methods for managing spatial data and interfacing with the game engine.
#[derive(Debug, Clone)]
pub struct PebbleVaultPlugin {
    /// The underlying VaultManager instance
    vault_manager: Arc<VaultManager<PebbleVaultCustomData>>,
}

impl PebbleVaultPlugin {
    /// Creates a new instance of PebbleVault
    ///
    /// This method initializes a new PebbleVault instance with a SQLite database
    /// for persistent storage.
    ///
    /// # Returns
    ///
    /// A Result containing the new PebbleVault instance or an error string
    ///
    /// # Examples
    ///
    /// ```
    /// use pebble_vault::PebbleVault;
    ///
    /// let pebble_vault = PebbleVault::new().expect("Failed to create PebbleVault");
    /// ```
    pub fn new() -> Result<Self, String> {
        let vault_manager = VaultManager::new("pebble_vault.db")?;
        Ok(PebbleVaultPlugin {
            vault_manager: Arc::new(vault_manager),
        })
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
    ///
    /// # Examples
    ///
    /// ```
    /// # use pebble_vault::PebbleVault;
    /// # let pebble_vault = PebbleVault::new().unwrap();
    /// let center = [0.0, 0.0, 0.0];
    /// let radius = 1000.0;
    /// let region_id = pebble_vault.create_or_load_region(center, radius).expect("Failed to create region");
    /// println!("Created region with ID: {}", region_id);
    /// ```
    pub fn create_or_load_region(&self, center: [f64; 3], radius: f64) -> Result<Uuid, String> {
        self.vault_manager.create_or_load_region(center, radius)
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
    ///
    /// # Examples
    ///
    /// ```
    /// # use pebble_vault::PebbleVault;
    /// # use uuid::Uuid;
    /// # let pebble_vault = PebbleVault::new().unwrap();
    /// # let region_id = pebble_vault.create_or_load_region([0.0, 0.0, 0.0], 1000.0).unwrap();
    /// let objects = pebble_vault.query_region(region_id, -100.0, -100.0, -100.0, 100.0, 100.0, 100.0)
    ///     .expect("Failed to query region");
    /// println!("Found {} objects in the region", objects.len());
    /// ```
    pub fn query_region(&self, region_id: Uuid, min_x: f64, min_y: f64, min_z: f64, max_x: f64, max_y: f64, max_z: f64) -> Result<Vec<SpatialObject<PebbleVaultCustomData>>, String> {
        self.vault_manager.query_region(region_id, min_x, min_y, min_z, max_x, max_y, max_z)
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
    ///
    /// # Examples
    ///
    /// ```
    /// # use pebble_vault::{PebbleVault, PebbleVaultCustomData};
    /// # use uuid::Uuid;
    /// # let pebble_vault = PebbleVault::new().unwrap();
    /// # let region_id = pebble_vault.create_or_load_region([0.0, 0.0, 0.0], 1000.0).unwrap();
    /// let object_id = Uuid::new_v4();
    /// let custom_data = PebbleVaultCustomData {
    ///     name: "Example Object".to_string(),
    ///     value: 42,
    /// };
    /// pebble_vault.add_object(region_id, object_id, "item", 10.0, 20.0, 30.0, custom_data)
    ///     .expect("Failed to add object");
    /// println!("Added object with ID: {}", object_id);
    /// ```
    pub fn add_object(&self, region_id: Uuid, uuid: Uuid, object_type: &str, x: f64, y: f64, z: f64, custom_data: PebbleVaultCustomData) -> Result<(), String> {
        self.vault_manager.add_object(region_id, uuid, object_type, x, y, z, Arc::new(custom_data))
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
    /// # Examples
    ///
    /// ```
    /// # use pebble_vault::{PebbleVault, PebbleVaultCustomData};
    /// # use uuid::Uuid;
    /// # let pebble_vault = PebbleVault::new().unwrap();
    /// # let region_id = pebble_vault.create_or_load_region([0.0, 0.0, 0.0], 1000.0).unwrap();
    /// # let object_id = Uuid::new_v4();
    /// # let custom_data = PebbleVaultCustomData { name: "Example Object".to_string(), value: 42 };
    /// # pebble_vault.add_object(region_id, object_id, "item", 10.0, 20.0, 30.0, custom_data).unwrap();
    /// pebble_vault.remove_object(object_id).expect("Failed to remove object");
    /// println!("Removed object with ID: {}", object_id);
    /// ```
    pub fn remove_object(&self, object_id: Uuid) -> Result<(), String> {
        self.vault_manager.remove_object(object_id)
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
    ///
    /// # Examples
    ///
    /// ```
    /// # use pebble_vault::{PebbleVault, PebbleVaultCustomData};
    /// # use uuid::Uuid;
    /// # let pebble_vault = PebbleVault::new().unwrap();
    /// # let region_id = pebble_vault.create_or_load_region([0.0, 0.0, 0.0], 1000.0).unwrap();
    /// # let object_id = Uuid::new_v4();
    /// # let custom_data = PebbleVaultCustomData { name: "Example Object".to_string(), value: 42 };
    /// # pebble_vault.add_object(region_id, object_id, "item", 10.0, 20.0, 30.0, custom_data).unwrap();
    /// if let Ok(Some(object)) = pebble_vault.get_object(object_id) {
    ///     println!("Found object: {:?}", object);
    /// } else {
    ///     println!("Object not found");
    /// }
    /// ```
    pub fn get_object(&self, object_id: Uuid) -> Result<Option<SpatialObject<PebbleVaultCustomData>>, String> {
        self.vault_manager.get_object(object_id)
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
    ///
    /// # Examples
    ///
    /// ```
    /// # use pebble_vault::{PebbleVault, PebbleVaultCustomData};
    /// # use uuid::Uuid;
    /// # let pebble_vault = PebbleVault::new().unwrap();
    /// # let region_id = pebble_vault.create_or_load_region([0.0, 0.0, 0.0], 1000.0).unwrap();
    /// # let object_id = Uuid::new_v4();
    /// # let custom_data = PebbleVaultCustomData { name: "Example Object".to_string(), value: 42 };
    /// # pebble_vault.add_object(region_id, object_id, "item", 10.0, 20.0, 30.0, custom_data).unwrap();
    /// if let Ok(Some(mut object)) = pebble_vault.get_object(object_id) {
    ///     object.point = [15.0, 25.0, 35.0];
    ///     pebble_vault.update_object(&object).expect("Failed to update object");
    ///     println!("Updated object position");
    /// }
    /// ```
    pub fn update_object(&self, object: &SpatialObject<PebbleVaultCustomData>) -> Result<(), String> {
        self.vault_manager.update_object(object)
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
    ///
    /// # Examples
    ///
    /// ```
    /// # use pebble_vault::{PebbleVault, PebbleVaultCustomData};
    /// # use uuid::Uuid;
    /// # let pebble_vault = PebbleVault::new().unwrap();
    /// # let region1_id = pebble_vault.create_or_load_region([0.0, 0.0, 0.0], 1000.0).unwrap();
    /// # let region2_id = pebble_vault.create_or_load_region([2000.0, 0.0, 0.0], 1000.0).unwrap();
    /// # let player_id = Uuid::new_v4();
    /// # let custom_data = PebbleVaultCustomData { name: "Player".to_string(), value: 100 };
    /// # pebble_vault.add_object(region1_id, player_id, "player", 10.0, 20.0, 30.0, custom_data).unwrap();
    /// pebble_vault.transfer_player(player_id, region1_id, region2_id)
    ///     .expect("Failed to transfer player");
    /// println!("Transferred player to new region");
    /// ```
    pub fn transfer_player(&self, player_uuid: Uuid, from_region_id: Uuid, to_region_id: Uuid) -> Result<(), String> {
        self.vault_manager.transfer_player(player_uuid, from_region_id, to_region_id)
    }

    /// Persists all in-memory databases to disk
    ///
    /// This method saves all spatial data from memory to the persistent storage.
    ///
    /// # Returns
    ///
    /// A Result indicating success or an error string
    ///
    /// # Examples
    ///
    /// ```
    /// # use pebble_vault::PebbleVault;
    /// # let pebble_vault = PebbleVault::new().unwrap();
    /// pebble_vault.persist_to_disk().expect("Failed to persist data");
    /// println!("Data persisted to disk");
    /// ```
    pub fn persist_to_disk(&self) -> Result<(), String> {
        self.vault_manager.persist_to_disk()
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
    ///
    /// # Examples
    ///
    /// ```
    /// # use pebble_vault::PebbleVault;
    /// # use uuid::Uuid;
    /// # let pebble_vault = PebbleVault::new().unwrap();
    /// # let region_id = pebble_vault.create_or_load_region([0.0, 0.0, 0.0], 1000.0).unwrap();
    /// if let Some(region) = pebble_vault.get_region(region_id) {
    ///     println!("Found region: {:?}", region);
    /// } else {
    ///     println!("Region not found");
    /// }
    /// ```
    pub fn get_region(&self, region_id: Uuid) -> Option<Arc<Mutex<VaultRegion<PebbleVaultCustomData>>>> {
        self.vault_manager.get_region(region_id)
    }
}

/// Implementation of the BaseAPI trait for PebbleVault
///
/// This trait implementation allows PebbleVault to handle game events and ticks.
#[async_trait]
impl BaseAPI for PebbleVaultPlugin {
    /// Handles game events
    ///
    /// This method processes various game events and updates the PebbleVault system accordingly.
    ///
    /// # Arguments
    ///
    /// * `event` - A reference to the GameEvent to handle
    async fn on_game_event(&self, event: &GameEvent) {
        match event {
            GameEvent::PlayerJoined(player) => {
                println!("PebbleVault: Player {} joined. Adding to database.", player.id);
                let custom_data = Arc::new(PebbleVaultCustomData {
                    name: format!("Player_{}", player.id),
                    value: 0,
                });
                if let Err(e) = self.vault_manager.add_object(
                    Uuid::nil(), // Assuming a default region, you might want to determine the correct region
                    player.id,
                    "player",
                    player.transform.unwrap().location.unwrap().x,      // TODO: Fix these because of possible runtime panics caused by unwrap()
                    player.transform.unwrap().location.unwrap().y,      // TODO: Fix these because of possible runtime panics caused by unwrap()
                    player.transform.unwrap().location.unwrap().z,      // TODO: Fix these because of possible runtime panics caused by unwrap()
                    custom_data,
                ) {
                    println!("Error adding player to PebbleVault: {}", e);
                }
            }
            GameEvent::PlayerMoved { player, new_position } => {
                println!("PebbleVault: Player {} moved to {:?}", player.id, new_position);
                if let Ok(Some(mut object)) = self.vault_manager.get_object(player.id) {
                    object.point = [new_position.x, new_position.y, new_position.z];
                    if let Err(e) = self.vault_manager.update_object(&object) {
                        println!("Error updating player position in PebbleVault: {}", e);
                    }
                }
            }
            _ => {}
        }
    }

    /// Handles game ticks
    ///
    /// This method is called periodically by the game engine and can be used for
    /// regular maintenance tasks or updates.
    ///
    /// # Arguments
    ///
    /// * `delta_time` - The time elapsed since the last tick
    async fn on_game_tick(&self, delta_time: f64) {
        println!("PebbleVault: Game tick, delta time: {:.2}", delta_time);
        // Perform any periodic operations, such as persisting data to disk
        if let Err(e) = self.vault_manager.persist_to_disk() {
            println!("Error persisting PebbleVault data: {}", e);
        }
    }

    /// Returns this object as a trait object
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Implementation of the RpcPlugin trait for PebbleVault
///
/// This trait implementation allows PebbleVault to be called via RPC (Remote Procedure Call).
#[async_trait]
impl RpcPlugin for PebbleVaultPlugin {
    /// Handles RPC calls to PebbleVault methods
    ///
    /// This method dispatches RPC calls to the appropriate PebbleVault functions.
    ///
    /// # Arguments
    ///
    /// * `method` - The name of the method to call
    /// * `params` - The parameters for the method call
    ///
    /// # Returns
    ///
    /// A Result containing the return value of the called method, or an error string
    async fn call_rpc(&self, method: &str, params: &(dyn Any + Send + Sync)) -> Result<Box<dyn Any + Send + Sync>, String> {
        match method {
            "create_or_load_region" => {
                if let Some((center, radius)) = params.downcast_ref::<([f64; 3], f64)>() {
                    match self.create_or_load_region(*center, *radius) {
                        Ok(region_id) => Ok(Box::new(region_id)),
                        Err(e) => Err(format!("Failed to create or load region: {}", e)),
                    }
                } else {
                    Err("Invalid parameters for create_or_load_region".to_string())
                }
            }
            "query_region" => {
                if let Some((region_id, min_x, min_y, min_z, max_x, max_y, max_z)) = params.downcast_ref::<(Uuid, f64, f64, f64, f64, f64, f64)>() {
                    match self.query_region(*region_id, *min_x, *min_y, *min_z, *max_x, *max_y, *max_z) {
                        Ok(objects) => Ok(Box::new(objects)),
                        Err(e) => Err(format!("Failed to query region: {}", e)),
                    }
                } else {
                    Err("Invalid parameters for query_region".to_string())
                }
            }
            "add_object" => {
                if let Some((region_id, uuid, object_type, x, y, z, custom_data)) = params.downcast_ref::<(Uuid, Uuid, String, f64, f64, f64, PebbleVaultCustomData)>() {
                    match self.add_object(*region_id, *uuid, object_type, *x, *y, *z, custom_data.clone()) {
                        Ok(()) => Ok(Box::new(())),
                        Err(e) => Err(format!("Failed to add object: {}", e)),
                    }
                } else {
                    Err("Invalid parameters for add_object".to_string())
                }
            }
            "remove_object" => {
                if let Some(object_id) = params.downcast_ref::<Uuid>() {
                    match self.remove_object(*object_id) {
                        Ok(()) => Ok(Box::new(())),
                        Err(e) => Err(format!("Failed to remove object: {}", e)),
                    }
                } else {
                    Err("Invalid parameters for remove_object".to_string())
                }
            }
            "get_object" => {
                if let Some(object_id) = params.downcast_ref::<Uuid>() {
                    match self.get_object(*object_id) {
                        Ok(object) => Ok(Box::new(object)),
                        Err(e) => Err(format!("Failed to get object: {}", e)),
                    }
                } else {
                    Err("Invalid parameters for get_object".to_string())
                }
            }
            "update_object" => {
                if let Some(object) = params.downcast_ref::<SpatialObject<PebbleVaultCustomData>>() {
                    match self.update_object(object) {
                        Ok(()) => Ok(Box::new(())),
                        Err(e) => Err(format!("Failed to update object: {}", e)),
                    }
                } else {
                    Err("Invalid parameters for update_object".to_string())
                }
            }
            "transfer_player" => {
                if let Some((player_uuid, from_region_id, to_region_id)) = params.downcast_ref::<(Uuid, Uuid, Uuid)>() {
                    match self.transfer_player(*player_uuid, *from_region_id, *to_region_id) {
                        Ok(()) => Ok(Box::new(())),
                        Err(e) => Err(format!("Failed to transfer player: {}", e)),
                    }
                } else {
                    Err("Invalid parameters for transfer_player".to_string())
                }
            }
            "persist_to_disk" => {
                match self.persist_to_disk() {
                    Ok(()) => Ok(Box::new(())),
                    Err(e) => Err(format!("Failed to persist to disk: {}", e)),
                }
            }
            _ => Err(format!("Unknown RPC method: {}", method)),
        }
    }
    async fn get_name() {
        // TODO
    }
    async fn get_id() {
        // TODO
    }
    async fn register_rpc() {
        // TODO
    }
}

/// Implementation of the Plugin trait for PebbleVault
///
/// This trait implementation allows PebbleVault to be loaded and managed as a plugin
/// in the game engine.
impl Plugin for PebbleVaultPlugin {
    /// Called when the plugin is loaded
    fn on_load(&self) {
        println!("PebbleVault plugin loaded");
    }

    /// Called when the plugin is unloaded
    fn on_unload(&self) {
        println!("PebbleVault plugin unloaded");
    }

    /// Called to execute the plugin
    fn execute(&self) {
        println!("PebbleVault plugin executed");
    }

    /// Called to initialize the plugin
    fn initialize(&self, context: &mut PluginContext) {
        println!("PebbleVault plugin initializing");
    }

    /// Called when the plugin is shutting down
    fn shutdown(&self, context: &mut PluginContext) {
        println!("PebbleVault plugin shutting down");
    }

    /// Called when the plugin is enabled
    fn on_enable(&self, context: &mut PluginContext) {
        println!("PebbleVault plugin enabled");
    }

    /// Called when the plugin is disabled
    fn on_disable(&self, context: &mut PluginContext) {
        println!("PebbleVault plugin disabled");
    }
}

/// Implementation of the PluginInformation trait for PebbleVault
///
/// This trait implementation provides information about the PebbleVault plugin.
impl PluginInformation for PebbleVaultPlugin {
    /// Returns the name of the plugin
    fn name(&self) -> String {
        "PebbleVault".to_string()
    }

    /// Returns an instance of the plugin that implements SayHello
    fn get_instance(&self) -> Box<dyn SayHello> {
        Box::new(self.clone())
    }
}

/// Implementation of the SayHello trait for PebbleVault
///
/// This trait implementation allows PebbleVault to provide a greeting message.
impl SayHello for PebbleVaultPlugin {
    /// Returns a greeting message from PebbleVault
    fn say_hello(&self) -> String {
        "Greetings from PebbleVault, your spatial database and object management system!".to_string()
    }
}

/// Creates and returns a new instance of the PebbleVault plugin
///
/// This function is used by the plugin system to create a new instance of PebbleVault.
///
/// # Returns
///
/// A Result containing a new PebbleVault instance or an error string
pub fn get_plugin() -> Result<PebbleVaultPlugin, String> {
    PebbleVaultPlugin::new()
}

/// Returns the metadata for the PebbleVault plugin
///
/// This function provides metadata about the PebbleVault plugin to the plugin system.
///
/// # Returns
///
/// A PluginMetadata struct containing information about the plugin
pub fn get_plugin_metadata() -> PluginMetadata {
    PluginMetadata {
        name: "PebbleVault".to_string(),
        version: "1.0.0".to_string(),
        description: "A spatial database and object management system for game worlds".to_string(),
        api_version: PLUGIN_API_VERSION,
    }
}