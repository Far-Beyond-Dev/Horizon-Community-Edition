use uuid::Uuid;
use crate::recipe_smith::{PlayerInventory, RecipeBook, RpcFunction, PluginContext, Recipe, Crafter, Item, StorageContainer};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::fmt;
use async_trait::async_trait;
use plugin_test_api::{RpcPlugin, BaseAPI, GameEvent, CustomEvent, Plugin, PluginInformation, SayHello};
use std::any::Any;

/// The main plugin struct for the RecipeSmith crafting system.
///
/// This struct implements the core functionality for managing recipes,
/// player inventories, and crafting operations in the game.
///
/// # Fields
/// * `id`: A unique identifier for the plugin instance.
/// * `name`: The name of the plugin.
/// * `initialized`: A flag indicating whether the plugin has been initialized.
/// * `recipe_book`: The collection of all available recipes.
/// * `player_inventories`: A collection of player inventories.
/// * `rpc_functions`: A map of RPC function names to their implementations.
///
/// # Example
/// ```
/// let recipe_smith = RecipeSmith::new();
/// let mut context = PluginContext::new();
/// recipe_smith.initialize(&mut context);
/// ```
pub struct RecipeSmith {
    id: Uuid,
    name: String,
    initialized: bool,
    recipe_book: Arc<RwLock<RecipeBook>>,
    player_inventories: Arc<RwLock<HashMap<String, PlayerInventory>>>,
    rpc_functions: HashMap<String, RpcFunction>,
}

impl fmt::Debug for RecipeSmith {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RecipeSmith")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("initialized", &self.initialized)
            .field("recipe_book", &"<RecipeBook>")
            .field("player_inventories", &"<PlayerInventories>")
            .field("rpc_functions", &"<RpcFunctions>")
            .finish()
    }
}


impl RecipeSmith {
    /// Creates a new instance of the RecipeSmith plugin.
    ///
    /// This method initializes the plugin with default values and
    /// registers all the necessary RPC functions.
    ///
    /// # Returns
    /// A new RecipeSmith instance.
    ///
    /// # Example
    /// ```
    /// let recipe_smith = RecipeSmith::new();
    /// ```
    pub fn new() -> Self {
        let mut plugin = Self {
            id: Uuid::new_v4(),
            name: "RecipeSmith".to_string(),
            initialized: false,
            recipe_book: Arc::new(RwLock::new(RecipeBook::new())),
            player_inventories: Arc::new(RwLock::new(HashMap::new())),
            rpc_functions: HashMap::new(),
        };

        // Register RPC functions
        plugin.register_rpc("create_player_inventory", Arc::new(RecipeSmith::create_player_inventory_rpc));
        plugin.register_rpc("get_player_inventory", Arc::new(RecipeSmith::get_player_inventory_rpc));
        plugin.register_rpc("update_player_inventory", Arc::new(RecipeSmith::update_player_inventory_rpc));
        plugin.register_rpc("craft_item", Arc::new(RecipeSmith::craft_item_rpc));
        plugin.register_rpc("get_all_recipes", Arc::new(RecipeSmith::get_all_recipes_rpc));
        plugin.register_rpc("add_new_recipe", Arc::new(RecipeSmith::add_new_recipe_rpc));
        plugin.register_rpc("get_recipes_by_crafter", Arc::new(RecipeSmith::get_recipes_by_crafter_rpc));
        plugin.register_rpc("get_player_inventory_contents", Arc::new(RecipeSmith::get_player_inventory_contents_rpc));
        plugin.register_rpc("add_item_to_player_inventory", Arc::new(RecipeSmith::add_item_to_player_inventory_rpc));
        plugin.register_rpc("remove_item_from_player_inventory", Arc::new(RecipeSmith::remove_item_from_player_inventory_rpc));
        plugin.register_rpc("create_storage_container", Arc::new(RecipeSmith::create_storage_container_rpc));
        plugin.register_rpc("access_storage_container", Arc::new(RecipeSmith::access_storage_container_rpc));
        plugin.register_rpc("transfer_item", Arc::new(RecipeSmith::transfer_item_rpc));


        plugin // Return the plugin with RPCs registered
    }

    /// Initializes the RecipeSmith plugin.
    ///
    /// This method sets up custom events, loads recipes from files,
    /// and marks the plugin as initialized.
    ///
    /// # Arguments
    /// * `context`: A mutable reference to the PluginContext.
    ///
    /// # Example
    /// ```
    /// let mut recipe_smith = RecipeSmith::new();
    /// let mut context = PluginContext::new();
    /// recipe_smith.initialize_recipe_smith(&mut context).await;
    /// ```
    async fn initialize_recipe_smith(&mut self, context: &mut PluginContext) {
        if !self.initialized {
            println!("RecipeSmith initializing...");
            self.register_custom_event("recipe_learned", context).await;
            self.register_custom_event("item_crafted", context).await;
            self.register_custom_event("inventory_changed", context).await;
            self.register_custom_event("recipe_mastered", context).await;
            self.register_custom_event("crafting_failed", context).await;
            self.register_custom_event("storage_container_created", context).await;
            self.register_custom_event("storage_container_accessed", context).await;

            // Load recipes from files
            let mut recipe_book = self.recipe_book.write().await;
            if let Err(e) = recipe_book.import_recipes_from_file("recipes.json") {
                println!("Error importing recipes from JSON: {}", e);
            }
            if let Err(e) = recipe_book.import_recipes_from_file("recipes.csv") {
                println!("Error importing recipes from CSV: {}", e);
            }

            self.initialized = true;
            println!("RecipeSmith initialized!");
        }
    }

    // RPC function implementations
    pub fn create_player_inventory_rpc(params: &(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync> {
        if let Some((player_id, num_slots)) = params.downcast_ref::<(String, u32)>() {
            let inventory = PlayerInventory::new(*num_slots);
            Box::new(inventory)
        } else {
            Box::new(())
        }
    }

    pub fn get_player_inventory_rpc(params: &(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync> {
        if let Some(player_id) = params.downcast_ref::<String>() {
            // This would typically involve accessing the player_inventories, but for simplicity, we'll return a new inventory
            Box::new(PlayerInventory::new(20))
        } else {
            Box::new(None::<PlayerInventory>)
        }
    }

    pub fn update_player_inventory_rpc(params: &(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync> {
        if let Some((player_id, inventory)) = params.downcast_ref::<(String, PlayerInventory)>() {
            // This would typically involve updating the player_inventories
            Box::new(true)
        } else {
            Box::new(false)
        }
    }

    pub fn craft_item_rpc(params: &(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync> {
        if let Some((player_id, recipe_name)) = params.downcast_ref::<(String, String)>() {
            // This would typically involve the crafting logic
            Box::new(Some("Crafted Item".to_string()))
        } else {
            Box::new(None::<String>)
        }
    }

    pub fn get_all_recipes_rpc(_params: &(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync> {
        // This would typically involve accessing the recipe_book
        Box::new(Vec::<Recipe>::new())
    }

    pub fn add_new_recipe_rpc(params: &(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync> {
        if let Some(recipe) = params.downcast_ref::<Recipe>() {
            // This would typically involve adding the recipe to the recipe_book
            Box::new(true)
        } else {
            Box::new(false)
        }
    }

    // Existing methods
    async fn get_all_recipes(&self) -> Vec<Recipe> {
        let recipe_book = self.recipe_book.read().await;
        recipe_book.recipes.values().cloned().collect()
    }

    async fn get_recipes_by_crafter(&self, crafter_name: &str) -> Vec<Recipe> {
        let recipe_book = self.recipe_book.read().await;
        let crafter = Crafter { name: crafter_name.to_string() };
        recipe_book.get_recipes_for_crafter(&crafter)
    }

    async fn add_new_recipe(&self, recipe: Recipe) {
        let mut recipe_book = self.recipe_book.write().await;
        recipe_book.add_recipe(recipe);
    }

    async fn get_player_inventory_contents(&self, player_id: &str) -> Option<Vec<Item>> {
        let inventories = self.player_inventories.read().await;
        inventories.get(player_id).map(|inventory| 
            inventory.slots.values().filter_map(|item| item.clone()).collect()
        )
    }

    async fn add_item_to_player_inventory(&self, player_id: &str, item: Item) -> Result<(), String> {
        let mut inventories = self.player_inventories.write().await;
        let inventory = inventories.get_mut(player_id).ok_or("Player inventory not found")?;
        
        for (_slot, item_opt) in inventory.slots.iter_mut() {
            if item_opt.is_none() {
                *item_opt = Some(item);
                return Ok(());
            }
        }
        
        Err("Inventory is full".to_string())
    }

    async fn remove_item_from_player_inventory(&self, player_id: &str, item_name: &str) -> Result<(), String> {
        let mut inventories = self.player_inventories.write().await;
        let inventory = inventories.get_mut(player_id).ok_or("Player inventory not found")?;
        
        for (_slot, item_opt) in inventory.slots.iter_mut() {
            if let Some(item) = item_opt {
                if item.name == item_name {
                    *item_opt = None;
                    return Ok(());
                }
            }
        }
        
        Err("Item not found in inventory".to_string())
    }

    async fn create_storage_container(&self, num_slots: u32) -> StorageContainer {
        StorageContainer::new(num_slots)
    }

    async fn access_storage_container(&self, container: &mut StorageContainer, player_id: &str, context: &mut PluginContext) {
        self.emit_custom_event(CustomEvent {
            event_type: "storage_container_accessed".to_string(),
            data: Arc::new((player_id.to_string(), container.uuid.to_string())),
        }, context).await;
    }

    async fn transfer_item(&self, from_inventory: &mut PlayerInventory, to_inventory: &mut PlayerInventory, item_name: &str) -> Result<(), String> {
        let mut item_to_transfer: Option<Item> = None;

        // Find and remove the item from the source inventory
        for (_slot, item_opt) in from_inventory.slots.iter_mut() {
            if let Some(item) = item_opt {
                if item.name == item_name {
                    item_to_transfer = item_opt.take();
                    break;
                }
            }
        }

        // If we found the item, add it to the destination inventory
        if let Some(item) = item_to_transfer {
            for (_slot, item_opt) in to_inventory.slots.iter_mut() {
                if item_opt.is_none() {
                    *item_opt = Some(item);
                    return Ok(());
                }
            }
            // If we couldn't add the item to the destination inventory, put it back in the source
            for (_slot, item_opt) in from_inventory.slots.iter_mut() {
                if item_opt.is_none() {
                    *item_opt = Some(item);
                    break;
                }
            }
            Err("Destination inventory is full".to_string())
        } else {
            Err("Item not found in source inventory".to_string())
        }
    }

    pub fn get_recipes_by_crafter_rpc(params: &(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync> {
        if let Some(crafter_name) = params.downcast_ref::<String>() {
            // This would typically involve accessing the recipe_book
            Box::new(Vec::<Recipe>::new())
        } else {
            Box::new(Vec::<Recipe>::new())
        }
    }

    pub fn get_player_inventory_contents_rpc(params: &(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync> {
        if let Some(player_id) = params.downcast_ref::<String>() {
            // RecipeSmith::get_player_inventory_contents(&Self, player_id);
            Box::new(Some(Vec::<Item>::new()))
        } else {
            Box::new(None::<Vec<Item>>)
        }
    }

    pub fn add_item_to_player_inventory_rpc(params: &(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync> {
        if let Some((player_id, item)) = params.downcast_ref::<(String, Item)>() {
            // This would typically involve updating the player_inventories
            Box::new(Ok(()) as Result<(), String>)
        } else {
            Box::new(Err("Invalid parameters".to_string()) as Result<(), String>)
        }
    }

    pub fn remove_item_from_player_inventory_rpc(params: &(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync> {
        if let Some((player_id, item_name)) = params.downcast_ref::<(String, String)>() {
            // This would typically involve updating the player_inventories
            Box::new(Ok(()) as Result<(), String>)
        } else {
            Box::new(Err("Invalid parameters".to_string()) as Result<(), String>)
        }
    }

    pub fn create_storage_container_rpc(params: &(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync> {
        if let Some(num_slots) = params.downcast_ref::<u32>() {
            Box::new(StorageContainer::new(*num_slots))
        } else {
            Box::new(StorageContainer::new(0))
        }
    }

    pub fn access_storage_container_rpc(params: &(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync> {
        if let Some((container, player_id)) = params.downcast_ref::<(StorageContainer, String)>() {
            // This would typically involve emitting a custom event
            Box::new(())
        } else {
            Box::new(())
        }
    }

    pub fn transfer_item_rpc(params: &(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync> {
        if let Some((from_inventory, to_inventory, item_name)) = params.downcast_ref::<(PlayerInventory, PlayerInventory, String)>() {
            // This would typically involve the actual transfer logic
            Box::new(Ok(()) as Result<(), String>)
        } else {
            Box::new(Err("Invalid parameters".to_string()) as Result<(), String>)
        }
    }
}

#[async_trait]
impl RpcPlugin for RecipeSmith {
    /// Returns the unique identifier of the plugin.
    ///
    /// # Returns
    /// The UUID of the RecipeSmith plugin instance.
    ///
    /// # Example
    /// ```
    /// let recipe_smith = RecipeSmith::new();
    /// let plugin_id = recipe_smith.get_id();
    /// ```
    fn get_id(&self) -> Uuid {
        self.id
    }

    /// Returns the name of the plugin.
    ///
    /// # Returns
    /// The name of the RecipeSmith plugin.
    ///
    /// # Example
    /// ```
    /// let recipe_smith = RecipeSmith::new();
    /// let plugin_name = recipe_smith.get_name();
    /// assert_eq!(plugin_name, "RecipeSmith");
    /// ```
    fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Registers an RPC function with the plugin.
    ///
    /// # Arguments
    /// * `name`: The name of the RPC function.
    /// * `func`: The RPC function to register.
    ///
    /// # Example
    /// ```
    /// let mut recipe_smith = RecipeSmith::new();
    /// recipe_smith.register_rpc("craft_item", Arc::new(RecipeSmith::craft_item_rpc));
    /// ```
    fn register_rpc(&mut self, name: &str, func: RpcFunction) {
        self.rpc_functions.insert(name.to_string(), func);
    }

    /// Registers an RPC function with the plugin.
    ///
    /// # Arguments
    /// * `name`: The name of the RPC function.
    /// * `func`: The RPC function to register.
    ///
    /// # Example
    /// ```
    /// let mut recipe_smith = RecipeSmith::new();
    /// recipe_smith.register_rpc("craft_item", Arc::new(RecipeSmith::craft_item_rpc));
    /// ```
    async fn call_rpc(&self, rpc_name: &str, params: &(dyn Any + Send + Sync)) -> Option<Box<dyn Any + Send + Sync>> {
        self.rpc_functions.get(rpc_name).map(|func| func(params))
    }
}

impl Clone for RecipeSmith {
    /// Creates a clone of the RecipeSmith instance.
    ///
    /// # Returns
    /// A new RecipeSmith instance with the same data as the original.
    ///
    /// # Example
    /// ```
    /// let recipe_smith = RecipeSmith::new();
    /// let cloned_recipe_smith = recipe_smith.clone();
    /// ```
    fn clone(&self) -> Self {
        RecipeSmith {
            id: self.id,
            name: self.name.clone(),
            initialized: self.initialized,
            recipe_book: Arc::clone(&self.recipe_book),
            player_inventories: Arc::clone(&self.player_inventories),
            rpc_functions: self.rpc_functions.clone(),
        }
    }
}

#[async_trait]
impl BaseAPI for RecipeSmith {
    /// Handles game events for the RecipeSmith plugin.
    ///
    /// This method processes various game events, such as players joining
    /// and custom events related to crafting and inventory management.
    ///
    /// # Arguments
    /// * `event`: The GameEvent to handle.
    ///
    /// # Example
    /// ```
    /// let recipe_smith = RecipeSmith::new();
    /// let player = Player { id: "player1".to_string(), /* other fields */ };
    /// let event = GameEvent::PlayerJoined(player);
    /// recipe_smith.on_game_event(&event).await;
    /// ```
    fn on_game_event(&self, event: &GameEvent) {
        match event {
            GameEvent::PlayerJoined(player) => {
                println!("RecipeSmith: Player {} joined. Initializing crafting data...", player.id);
                let params: Box<dyn Any + Send + Sync> = Box::new((player.id.clone(), 20u32));
            //  TODO: Fix this async to block perhaps
            //     self.call_rpc("create_player_inventory", &*params).await;
            }
            GameEvent::Custom(custom_event) => {
                match custom_event.event_type.as_str() {
                    "recipe_learned" => println!("RecipeSmith: New recipe learned!"),
                    "item_crafted" => println!("RecipeSmith: Item crafted!"),
                    "inventory_changed" => println!("RecipeSmith: Inventory updated!"),
                    "recipe_mastered" => println!("RecipeSmith: Recipe mastered!"),
                    "crafting_failed" => println!("RecipeSmith: Crafting failed!"),
                    "storage_container_created" => println!("RecipeSmith: New storage container created!"),
                    "storage_container_accessed" => println!("RecipeSmith: Storage container accessed!"),
                    //  TODO: Fix this async
                    //  "craft_item" => {
                    //      if let Some((player_id, recipe_name)) = custom_event.data.downcast_ref::<(String, String)>() {
                    //          if let Some(result) = self.call_rpc("craft_item", &(player_id.clone(), recipe_name.clone())).await {
                    //              if let Some(crafted_item) = result.downcast_ref::<Option<String>>() {
                    //                  if let Some(item_name) = crafted_item {
                    //                      println!("RecipeSmith: Player {} crafted {}", player_id, item_name);
                    //                  } else {
                    //                      println!("RecipeSmith: Player {} failed to craft {}", player_id, recipe_name);
                    //                  }
                    //              }
                    //          }
                    //      }
                    //  },

                    // TODO: Fix this async
                    //  "add_recipe" => {
                    //      if let Some(recipe) = custom_event.data.downcast_ref::<Recipe>() {
                    //          if let Some(result) = self.call_rpc("add_new_recipe", recipe).await {
                    //              if let Some(&success) = result.downcast_ref::<bool>() {
                    //                  if success {
                    //                      println!("RecipeSmith: New recipe added: {}", recipe.name);
                    //                  } else {
                    //                      println!("RecipeSmith: Failed to add new recipe: {}", recipe.name);
                    //                  }
                    //              }
                    //          }
                    //      }
                    //  },
                    
                    // TODO: Fix these asyncs
                    //  "get_player_inventory" => {
                    //      if let Some(player_id) = custom_event.data.downcast_ref::<String>() {
                    //          if let Some(result) = self.call_rpc("get_player_inventory", player_id).await {
                    //              if let Some(inventory) = result.downcast_ref::<PlayerInventory>() {
                    //                  println!("RecipeSmith: Retrieved inventory for player {}", player_id);
                    //                  // You might want to emit a custom event here with the inventory data
                    //              } else {
                    //                  println!("RecipeSmith: Failed to retrieve inventory for player {}", player_id);
                    //              }
                    //          }
                    //      }
                    //  },
                    //  "get_all_recipes" => {
                    //      if let Some(result) = self.call_rpc("get_all_recipes", &()).await {
                    //          if let Some(recipes) = result.downcast_ref::<Vec<Recipe>>() {
                    //              println!("RecipeSmith: Retrieved all recipes, count: {}", recipes.len());
                    //              // You might want to emit a custom event here with the recipes data
                    //          } else {
                    //              println!("RecipeSmith: Failed to retrieve recipes");
                    //          }
                    //      }
                    //  },
                    _ => {}
                }
            }
        // Ignore all other events
        _ => {}
        }
    }

    /// Handles game tick events for the RecipeSmith plugin.
    ///
    /// This method is called on each game tick and can be used to implement
    /// time-based logic for the crafting system.
    ///
    /// # Arguments
    /// * `delta_time`: The time elapsed since the last tick.
    ///
    /// # Example
    /// ```
    /// let recipe_smith = RecipeSmith::new();
    /// recipe_smith.on_game_tick(0.016).await; // 60 FPS
    /// ```
    async fn on_game_tick(&self, _delta_time: f64) {
    // Implement tick logic if needed
    }

    /// Registers a custom event for the RecipeSmith plugin.
    ///
    /// # Arguments
    /// * `event_type`: The type of custom event to register.
    /// * `context`: A mutable reference to the PluginContext.
    ///
    /// # Example
    /// ```
    /// let recipe_smith = RecipeSmith::new();
    /// let mut context = PluginContext::new();
    /// recipe_smith.register_custom_event("recipe_learned", &mut context).await;
    /// ```
    async fn register_custom_event(&self, event_type: &str, context: &mut PluginContext) {
        context.register_for_custom_event(event_type, Arc::new(self.clone())).await;
    }

    /// Emits a custom event for the RecipeSmith plugin.
    ///
    /// # Arguments
    /// * `event`: The CustomEvent to emit.
    /// * `context`: A mutable reference to the PluginContext.
    ///
    /// # Example
    /// ```
    /// let recipe_smith = RecipeSmith::new();
    /// let mut context = PluginContext::new();
    /// let event = CustomEvent {
    ///     event_type: "recipe_learned".to_string(),
    ///     data: Arc::new("Iron Sword".to_string()),
    /// };
    /// recipe_smith.emit_custom_event(event, &mut context).await;
    /// ```
    async fn emit_custom_event(&self, event: CustomEvent, context: &mut PluginContext) {
        context.dispatch_custom_event(event).await;
    }

    /// Returns a reference to the RecipeSmith instance as a trait object.
    ///
    /// # Returns
    /// A reference to the RecipeSmith instance as a trait object.
    ///
    /// # Example
    /// ```
    /// let recipe_smith = RecipeSmith::new();
    /// let as_any = recipe_smith.as_any();
    /// ```
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Plugin for RecipeSmith {
    /// Called when the RecipeSmith plugin is loaded.
    ///
    /// # Example
    /// ```
    /// let recipe_smith = RecipeSmith::new();
    /// recipe_s
    fn on_load(&self) {
        println!("RecipeSmith plugin loaded");
    }

    /// Called when the RecipeSmith plugin is unloaded.
    ///
    /// # Example
    /// ```
    /// let recipe_smith = RecipeSmith::new();
    /// recipe_smith.on_unload();
    /// ```
    fn on_unload(&self) {
        println!("RecipeSmith plugin unloaded");
    }

    /// Executes the RecipeSmith plugin.
    ///
    /// # Example
    /// ```
    /// let recipe_smith = RecipeSmith::new();
    /// recipe_smith.execute();
    /// ```
    fn execute(&self) {
    println!("RecipeSmith plugin executed");
    }

    /// Initializes the RecipeSmith plugin.
    ///
    /// # Arguments
    /// * `context`: A mutable reference to the PluginContext.
    ///
    /// # Example
    /// ```
    /// let recipe_smith = RecipeSmith::new();
    /// let mut context = PluginContext::new();
    /// recipe_smith.initialize(&mut context);
    /// ```
    fn initialize(&self, context: &mut PluginContext) {
        println!("RecipeSmith plugin initializing");
        let mut recipe_smith = self.clone();
        
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async {
                recipe_smith.initialize_recipe_smith(context).await;
            });
        
        println!("RecipeSmith plugin initialized");
    }

    /// Shuts down the RecipeSmith plugin.
    ///
    /// # Arguments
    /// * `context`: A mutable reference to the PluginContext.
    ///
    /// # Example
    /// ```
    /// let recipe_smith = RecipeSmith::new();
    /// let mut context = PluginContext::new();
    /// recipe_smith.shutdown(&mut context);
    /// ```
    fn shutdown(&self, _context: &mut PluginContext) {
        println!("RecipeSmith plugin shut down");
    }

    /// Called when the RecipeSmith plugin is enabled.
    ///
    /// # Arguments
    /// * `context`: A mutable reference to the PluginContext.
    ///
    /// # Example
    /// ```
    /// let recipe_smith = RecipeSmith::new();
    /// let mut context = PluginContext::new();
    /// recipe_smith.on_enable(&mut context);
    /// ```
    fn on_enable(&self, _context: &mut PluginContext) {
        println!("RecipeSmith plugin enabled");
    }

    /// Called when the RecipeSmith plugin is disabled.
    ///
    /// # Arguments
    /// * `context`: A mutable reference to the PluginContext.
    ///
    /// # Example
    /// ```
    /// let recipe_smith = RecipeSmith::new();
    /// let mut context = PluginContext::new();
    /// recipe_smith.on_disable(&mut context);
    /// ```
    fn on_disable(&self, _context: &mut PluginContext) {
        println!("RecipeSmith plugin disabled");
    }
}

impl PluginInformation for RecipeSmith {
    /// Returns the name of the RecipeSmith plugin.
    ///
    /// # Returns
    /// The name of the plugin as a String.
    ///
    /// # Example
    /// ```
    /// let recipe_smith = RecipeSmith::new();
    /// assert_eq!(recipe_smith.name(), "RecipeSmith");
    /// ```
    fn name(&self) -> String {
        "RecipeSmith".to_string()
    }


    /// Returns a boxed instance of the RecipeSmith plugin that implements the SayHello trait.
    ///
    /// # Returns
    /// A Box containing a trait object that implements SayHello.
    ///
    /// # Example
    /// ```
    /// let recipe_smith = RecipeSmith::new();
    /// let hello_instance = recipe_smith.get_instance();
    /// println!("{}", hello_instance.say_hello());
    /// ```
    fn get_instance(&self) -> Box<dyn SayHello> {
        Box::new(self.clone())
    }

    fn broadcast_game_event(&self, plugin: & &Box<dyn BaseAPI> ,event:GameEvent) {
        plugin.on_game_event(&event);
    }
    
    fn get_plugin(&self) -> Box<dyn BaseAPI>  {
        Box::new(RecipeSmith::new())
    }
}   

impl SayHello for RecipeSmith {
    /// Returns a greeting message from the RecipeSmith plugin.
    /// This is displayed on server start.
    ///
    /// # Returns
    /// A String containing the greeting message.
    ///
    /// # Example
    /// ```
    /// let recipe_smith = RecipeSmith::new();
    /// println!("{}", recipe_smith.say_hello());
    /// ```
    fn say_hello(&self) -> String {
        "Hello from RecipeSmith! Ready to craft some amazing items?".to_string()
    }
}