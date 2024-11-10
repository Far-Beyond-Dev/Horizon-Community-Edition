use serde::{Deserialize, Serialize};

/// Represents an ingredient in a recipe.
///
/// # Fields
/// * `name`: The name of the ingredient.
/// * `quantity`: The required quantity of the ingredient.
/// * `recipe_craftable`: Indicates if this ingredient can be crafted from other recipes.
///
/// # Example
/// ```
/// let flour = Ingredient {
///     name: "Flour".to_string(),
///     quantity: 2,
///     recipe_craftable: false,
/// };
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ingredient {
    pub name: String,
    pub quantity: u32,
    pub recipe_craftable: bool,
}
