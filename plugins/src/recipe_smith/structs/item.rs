use serde::{Deserialize, Serialize};
use std::collections::HashMap;


/// Represents an item in the game.
///
/// # Fields
/// * `name`: The name of the item.
/// * `model`: An optional 3D model identifier for the item.
/// * `meta_tags`: Additional metadata associated with the item.
///
/// # Example
/// ```
/// let sword = Item {
///     name: "Iron Sword".to_string(),
///     model: Some("models/iron_sword.obj".to_string()),
///     meta_tags: {
///         let mut tags = HashMap::new();
///         tags.insert("damage".to_string(), json!(10));
///         tags.insert("durability".to_string(), json!(100));
///         tags
///     },
/// };
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Item {
    pub name: String,
    pub model: Option<String>,
    pub meta_tags: HashMap<String, serde_json::Value>,
}