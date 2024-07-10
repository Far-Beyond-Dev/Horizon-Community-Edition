/// This Script is simply a Rough Concept and May be changed Multiple Times before we settle on a result so don't get too comfy just yet.

// This is mainly just a simple Subsystem Asterisk & Trident Have Been Working on. We will most likely be implementing the Associated Databases found here: Horizon-Community-Edition\src\subsystems\Recipe Smith Subsystem\Recipe List into (SQL) Soon for performance Reasons.
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind};
use csv::Reader;
use serde_json::Value;

/// Structure representing an Ingredient.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ingredient {
    pub name: String,    // Name of the ingredient
    pub quantity: u32,   // Quantity of the ingredient needed
    // Add more properties here as needed
}

/// Structure representing a Recipe.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Recipe {
    pub name: String,                       // Name of the recipe
    pub ingredients: Vec<Ingredient>,       // List of ingredients and their quantities
    pub outcome: String,                    // Outcome of the recipe
    // Add more properties here as needed
}

/// Struct representing a Recipe Book.
pub struct RecipeBook {
    pub recipes: HashMap<String, Recipe>,    // HashMap to store recipes
    // Add more properties here if you want to, I just did this as a starting example so people understand it better
}

impl RecipeBook {
    /// Creates a new RecipeBook.
    pub fn new() -> Self {
        Self {
            recipes: HashMap::new(),    // Here I've Simply Initialized an empty HashMap for recipes
        }
    }

    /// The Code Below Adds a new recipe to the RecipeBook.
    pub fn add_recipe(&mut self, recipe: Recipe) {
        self.recipes.insert(recipe.name.clone(), recipe);    // Insert a recipe into the HashMap
    }

    /// Retrieves a recipe by its name.
    pub fn get_recipe(&self, name: &str) -> Option<&Recipe> {
        self.recipes.get(name)    // Retrieve a recipe by name from the HashMap
    }

    /// Checks if a recipe can be crafted with the given inventory.
    pub fn can_craft(&self, recipe_name: &str, inventory: &HashMap<String, u32>) -> bool {
        if let Some(recipe) = self.get_recipe(recipe_name) {
            for ingredient in &recipe.ingredients {
                if let Some(inventory_qty) = inventory.get(&ingredient.name) {
                    if *inventory_qty < ingredient.quantity {
                        return false;    // Not enough of this ingredient in inventory
                    }
                } else {
                    return false;    // Missing this ingredient in inventory
                }
            }
            true    // Can craft the recipe
        } else {
            false    // Recipe not found in the RecipeBook
        }
    }

    /// Crafts a recipe if possible, updating the inventory.
    pub fn craft(&self, recipe_name: &str, inventory: &mut HashMap<String, u32>) -> Option<String> {
        if self.can_craft(recipe_name, inventory) {
            let recipe = self.get_recipe(recipe_name).unwrap().clone();    // Clone the recipe
            for ingredient in &recipe.ingredients {
                *inventory.entry(ingredient.name.clone()).and_modify(|e| *e -= ingredient.quantity).or_insert(0);
            }
            Some(recipe.outcome.clone())    // Return the outcome of crafting
        } else {
            None    // Return None if crafting fails
        }
    }

    // This import Feature is a special request from that of Trident so please (Do Not Remove) Unless Authorized by him First.

    /// Imports recipes from a JSON or CSV file.
    pub fn import_recipes_from_file(&mut self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        if filename.ends_with(".json") {
            // Parse JSON
            let recipes: Vec<Recipe> = serde_json::from_reader(reader)?;
            for recipe in recipes {
                self.add_recipe(recipe);
            }
        } else if filename.ends_with(".csv") {
            // Parse CSV
            let mut csv_reader = Reader::from_reader(reader);
            for result in csv_reader.deserialize::<Recipe>() {
                let recipe = result?;
                self.add_recipe(recipe);
            }
        } else {
            return Err(Box::new(Error::new(ErrorKind::Other, "Unsupported file format")));
        }

        Ok(())
    }
}

/// Main.rs code is below this stays here and below I've provided a Sample for understanding it Further by the rest of the SBH Team.

fn main() {
    // Example usage:
    let mut recipe_book = RecipeBook::new();

    // Import recipes from JSON
    if let Err(e) = recipe_book.import_recipes_from_file("Recipe List/recipes.json") {
        eprintln!("Error importing recipes: {}", e);
    }

    // Import recipes from CSV
    if let Err(e) = recipe_book.import_recipes_from_file("Recipe List/recipes.csv") {
        eprintln!("Error importing recipes: {}", e);
    }

    // Improved the hashmap based on the structure of common matrix arrays for ease of use and editing.
    let items = [
        "Herb", "Water", "Flour", "Salt", "Sugar", "Egg", "Milk", "Meat", 
        "Potato", "Carrot", "Lettuce", "Tomato", "Cucumber", "Olive Oil", 
        "Ham", "Cheese"
    ];
    
    let quantities = [
        3, 2, 4, 2, 3, 4, 1, 2, 3, 2, 3, 2, 1, 1, 1, 1
    ];

    let mut inventory: HashMap<String, u32> = items.iter()
        .cloned()
        .zip(quantities.iter().cloned())
        .map(|(item, quantity)| (item.to_string(), quantity))
        .collect();

    // Below is the section I've added that uses the items and attempts to craft them if the user has the required resources / ingredients.
    // Otherwise, it should say not enough Ingredients or Simply Failed to craft if they don't meet Gathered Requirements.

    // Attempt to craft Bread
    if recipe_book.can_craft("Bread", &inventory) {
        if let Some(item) = recipe_book.craft("Bread", &mut inventory) {
            println!("Crafted: {}", item);
        } else {
            println!("Failed to craft Bread.");
        }
    } else {
        println!("Not enough ingredients to craft Bread.");
    }

    // Attempt to craft Cake
    if recipe_book.can_craft("Cake", &inventory) {
        if let Some(item) = recipe_book.craft("Cake", &mut inventory) {
            println!("Crafted: {}", item);
        } else {
            println!("Failed to craft Cake.");
        }
    } else {
        println!("Not enough ingredients to craft Cake.");
    }
}