use std::sync::Arc;
use tokio::sync::RwLock;
use async_trait::async_trait;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use horizon_data_types::Player;
use std::any::Any;
use plugin_test_api::{RpcPlugin, RpcFunction, BaseAPI, GameEvent, CustomEvent, PluginContext, Plugin, PluginInformation, SayHello};
use csv;
use std::fmt;

// Struct definitions

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ingredient {
    pub name: String,
    pub quantity: u32,
    pub recipe_craftable: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Crafter {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Recipe {
    pub name: String,
    pub ingredients: Vec<Ingredient>,
    pub outcome: String,
    pub crafters: Vec<Crafter>,
    pub base_cook_time: u32,
    pub cook_count: u32,
}

impl Recipe {
    fn increment_cook_count(&mut self) {
        self.cook_count += 1;
    }

    fn is_mastered(&self) -> bool {
        self.cook_count >= 10
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Item {
    pub name: String,
    pub model: Option<String>,
    pub meta_tags: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlayerInventory {
    pub slots: HashMap<u32, Option<Item>>,
}

impl PlayerInventory {
    pub fn new(num_slots: u32) -> Self {
        let mut slots = HashMap::new();
        for i in 0..num_slots {
            slots.insert(i, None);
        }
        Self { slots }
    }

    pub fn get_item(&self, slot: u32) -> Option<&Item> {
        self.slots.get(&slot).and_then(|item| item.as_ref())
    }

    pub fn add_item(&mut self, slot: u32, item: Item) {
        self.slots.insert(slot, Some(item));
    }

    pub fn remove_item(&mut self, slot: u32) -> Option<Item> {
        self.slots.insert(slot, None).flatten()
    }

    pub fn empty_slot(&mut self, slot: u32) {
        self.slots.insert(slot, None);
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StorageContainer {
    pub uuid: Uuid,
    pub inventory: PlayerInventory,
}

impl StorageContainer {
    pub fn new(num_slots: u32) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            inventory: PlayerInventory::new(num_slots),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecipeBook {
    pub recipes: HashMap<String, Recipe>,
    pub crafters: HashMap<Crafter, Vec<String>>,
}

impl RecipeBook {
    pub fn new() -> Self {
        Self {
            recipes: HashMap::new(),
            crafters: HashMap::new(),
        }
    }

    pub fn add_recipe(&mut self, recipe: Recipe) {
        for crafter in &recipe.crafters {
            self.crafters.entry(crafter.clone()).or_insert_with(Vec::new).push(recipe.name.clone());
        }
        self.recipes.insert(recipe.name.clone(), recipe);
    }

    pub fn get_recipe(&self, name: &str) -> Option<Recipe> {
        self.recipes.get(name).cloned()
    }

    pub fn get_recipes_for_crafter(&self, crafter: &Crafter) -> Vec<Recipe> {
        self.crafters.get(crafter)
            .map(|recipe_names| recipe_names.iter().filter_map(|name| self.get_recipe(name)).collect())
            .unwrap_or_else(Vec::new)
    }

    pub fn can_craft(&self, recipe_name: &str, inventory: &HashMap<String, Ingredient>) -> bool {
        if let Some(recipe) = self.get_recipe(recipe_name) {
            recipe.ingredients.iter().all(|ingredient| {
                inventory.get(&ingredient.name)
                    .map(|inv_ingredient| inv_ingredient.recipe_craftable && inv_ingredient.quantity >= ingredient.quantity)
                    .unwrap_or(false)
            })
        } else {
            false
        }
    }

    pub async fn craft(&mut self, recipe_name: &str, inventory: &mut HashMap<String, Ingredient>) -> Option<String> {
        if self.can_craft(recipe_name, inventory) {
            let recipe = self.get_recipe(recipe_name)?;
            
            // Consume ingredients
            for ingredient in &recipe.ingredients {
                if let Some(inv_ingredient) = inventory.get_mut(&ingredient.name) {
                    inv_ingredient.quantity -= ingredient.quantity;
                }
            }

            // Simulate crafting time
            tokio::time::sleep(tokio::time::Duration::from_secs(recipe.base_cook_time.into())).await;

            // Update recipe
            if let Some(recipe) = self.recipes.get_mut(recipe_name) {
                recipe.increment_cook_count();
            }

            Some(recipe.outcome.clone())
        } else {
            None
        }
    }

    pub fn import_recipes_from_file(&mut self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::File::open(filename)?;
        let reader = std::io::BufReader::new(file);

        if filename.ends_with(".json") {
            let recipes: Vec<Recipe> = serde_json::from_reader(reader)?;
            for recipe in recipes {
                self.add_recipe(recipe);
            }
        } else if filename.ends_with(".csv") {
            let mut csv_reader = csv::Reader::from_reader(reader);
            for result in csv_reader.deserialize() {
                let recipe: Recipe = result?;
                self.add_recipe(recipe);
            }
        } else {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Unsupported file format")));
        }

        Ok(())
    }
}

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

        plugin
    }

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
    fn create_player_inventory_rpc(params: &(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync> {
        if let Some((player_id, num_slots)) = params.downcast_ref::<(String, u32)>() {
            let inventory = PlayerInventory::new(*num_slots);
            Box::new(inventory)
        } else {
            Box::new(())
        }
    }

    fn get_player_inventory_rpc(params: &(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync> {
        if let Some(player_id) = params.downcast_ref::<String>() {
            // This would typically involve accessing the player_inventories, but for simplicity, we'll return a new inventory
            Box::new(PlayerInventory::new(20))
        } else {
            Box::new(None::<PlayerInventory>)
        }
    }

    fn update_player_inventory_rpc(params: &(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync> {
        if let Some((player_id, inventory)) = params.downcast_ref::<(String, PlayerInventory)>() {
            // This would typically involve updating the player_inventories
            Box::new(true)
        } else {
            Box::new(false)
        }
    }

    fn craft_item_rpc(params: &(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync> {
        if let Some((player_id, recipe_name)) = params.downcast_ref::<(String, String)>() {
            // This would typically involve the crafting logic
            Box::new(Some("Crafted Item".to_string()))
        } else {
            Box::new(None::<String>)
        }
    }

    fn get_all_recipes_rpc(_params: &(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync> {
        // This would typically involve accessing the recipe_book
        Box::new(Vec::<Recipe>::new())
    }

    fn add_new_recipe_rpc(params: &(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync> {
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
}

#[async_trait]
impl RpcPlugin for RecipeSmith {
    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn register_rpc(&mut self, name: &str, func: RpcFunction) {
        self.rpc_functions.insert(name.to_string(), func);
    }

    async fn call_rpc(&self, rpc_name: &str, params: &(dyn Any + Send + Sync)) -> Option<Box<dyn Any + Send + Sync>> {
        self.rpc_functions.get(rpc_name).map(|func| func(params))
    }
}

#[async_trait]
impl BaseAPI for RecipeSmith {
async fn on_game_event(&self, event: &GameEvent) {
match event {
    GameEvent::PlayerJoined(player) => {
        println!("RecipeSmith: Player {} joined. Initializing crafting data...", player.id);
        let params: Box<dyn Any + Send + Sync> = Box::new((player.id.clone(), 20u32));
        self.call_rpc("create_player_inventory", &*params).await;
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
            "craft_item" => {
                if let Some((player_id, recipe_name)) = custom_event.data.downcast_ref::<(String, String)>() {
                    if let Some(result) = self.call_rpc("craft_item", &(player_id.clone(), recipe_name.clone())).await {
                        if let Some(crafted_item) = result.downcast_ref::<Option<String>>() {
                            if let Some(item_name) = crafted_item {
                                println!("RecipeSmith: Player {} crafted {}", player_id, item_name);
                            } else {
                                println!("RecipeSmith: Player {} failed to craft {}", player_id, recipe_name);
                            }
                        }
                    }
                }
            },
            "add_recipe" => {
                if let Some(recipe) = custom_event.data.downcast_ref::<Recipe>() {
                    if let Some(result) = self.call_rpc("add_new_recipe", recipe).await {
                        if let Some(&success) = result.downcast_ref::<bool>() {
                            if success {
                                println!("RecipeSmith: New recipe added: {}", recipe.name);
                            } else {
                                println!("RecipeSmith: Failed to add new recipe: {}", recipe.name);
                            }
                        }
                    }
                }
            },
            "get_player_inventory" => {
                if let Some(player_id) = custom_event.data.downcast_ref::<String>() {
                    if let Some(result) = self.call_rpc("get_player_inventory", player_id).await {
                        if let Some(inventory) = result.downcast_ref::<PlayerInventory>() {
                            println!("RecipeSmith: Retrieved inventory for player {}", player_id);
                            // You might want to emit a custom event here with the inventory data
                        } else {
                            println!("RecipeSmith: Failed to retrieve inventory for player {}", player_id);
                        }
                    }
                }
            },
            "get_all_recipes" => {
                if let Some(result) = self.call_rpc("get_all_recipes", &()).await {
                    if let Some(recipes) = result.downcast_ref::<Vec<Recipe>>() {
                        println!("RecipeSmith: Retrieved all recipes, count: {}", recipes.len());
                        // You might want to emit a custom event here with the recipes data
                    } else {
                        println!("RecipeSmith: Failed to retrieve recipes");
                    }
                }
            },
            _ => {}
        }
    }
    _ => {}
}
}

async fn on_game_tick(&self, _delta_time: f64) {
// Implement tick logic if needed
}

async fn register_custom_event(&self, event_type: &str, context: &mut PluginContext) {
context.register_for_custom_event(event_type, Arc::new(self.clone())).await;
}

async fn emit_custom_event(&self, event: CustomEvent, context: &mut PluginContext) {
context.dispatch_custom_event(event).await;
}

fn as_any(&self) -> &dyn std::any::Any {
self
}
}

impl Plugin for RecipeSmith {
    fn on_load(&self) {
        println!("RecipeSmith plugin loaded");
    }

    fn on_unload(&self) {
        println!("RecipeSmith plugin unloaded");
    }

    fn execute(&self) {
    println!("RecipeSmith plugin executed");
    }

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

    fn shutdown(&self, _context: &mut PluginContext) {
        println!("RecipeSmith plugin shut down");
    }

    fn on_enable(&self, _context: &mut PluginContext) {
        println!("RecipeSmith plugin enabled");
    }

    fn on_disable(&self, _context: &mut PluginContext) {
        println!("RecipeSmith plugin disabled");
    }
}

impl PluginInformation for RecipeSmith {
    fn name(&self) -> String {
    "RecipeSmith".to_string()
    }

    fn get_instance(&self) -> Box<dyn SayHello> {
    Box::new(self.clone())
    }
}

impl SayHello for RecipeSmith {
    fn say_hello(&self) -> String {
        "Hello from RecipeSmith! Ready to craft some amazing items?".to_string()
    }
}

impl Clone for RecipeSmith {
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

pub fn create_plugin_metadata() -> RecipeSmith {
    RecipeSmith::new()
}