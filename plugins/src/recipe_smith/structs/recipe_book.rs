use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::recipe_smith::{Recipe, Crafter, Ingredient};

/// Represents a collection of recipes and their associated crafters.
///
/// # Fields
/// * `recipes`: A HashMap of recipe names to Recipe objects.
/// * `crafters`: A HashMap of Crafter objects to lists of recipe names they can craft.
///
/// # Example
/// ```
/// let mut recipe_book = RecipeBook::new();
/// let sword_recipe = Recipe {
///     name: "Iron Sword".to_string(),
///     ingredients: vec![
///         Ingredient { name: "Iron Ingot".to_string(), quantity: 2, recipe_craftable: true },
///         Ingredient { name: "Wood".to_string(), quantity: 1, recipe_craftable: false },
///     ],
///     outcome: "Iron Sword".to_string(),
///     crafters: vec![Crafter { name: "Blacksmith".to_string() }],
///     base_cook_time: 30,
///     cook_count: 0,
/// };
/// recipe_book.add_recipe(sword_recipe);
/// ```
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