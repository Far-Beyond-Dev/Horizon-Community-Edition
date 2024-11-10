use serde::{Deserialize, Serialize};
use crate::recipe_smith::{Ingredient, Crafter};

/// Represents a recipe for crafting items.
///
/// # Fields
/// * `name`: The name of the recipe.
/// * `ingredients`: A list of required ingredients.
/// * `outcome`: The name of the item produced by this recipe.
/// * `crafters`: A list of crafters who can use this recipe.
/// * `base_cook_time`: The base time required to craft this recipe.
/// * `cook_count`: The number of times this recipe has been crafted.
///
/// # Example
/// ```
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
/// ```
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
    pub fn increment_cook_count(&mut self) {
        self.cook_count += 1;
    }

    pub fn is_mastered(&self) -> bool {
        self.cook_count >= 10
    }
}