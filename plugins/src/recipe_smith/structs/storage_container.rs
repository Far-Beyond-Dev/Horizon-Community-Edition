use serde::{Deserialize, Serialize};
use crate::recipe_smith::{PlayerInventory};
use uuid::Uuid;

/// Represents a storage container in the game world.
///
/// # Fields
/// * `uuid`: A unique identifier for the container.
/// * `inventory`: The inventory associated with this container.
///
/// # Example
/// ```
/// let chest = StorageContainer::new(30);
/// let item = Item {
///     name: "Gold Coin".to_string(),
///     model: None,
///     meta_tags: HashMap::new(),
/// };
/// chest.inventory.add_item(0, item);
/// ```
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