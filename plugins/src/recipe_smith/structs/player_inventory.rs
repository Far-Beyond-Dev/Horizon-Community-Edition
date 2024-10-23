use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::recipe_smith::Item;

/// Represents a player's inventory with a fixed number of slots.
///
/// # Fields
/// * `slots`: A HashMap representing inventory slots and their contents.
///
/// # Example
/// ```
/// let mut inventory = PlayerInventory::new(20);
/// let sword = Item {
///     name: "Iron Sword".to_string(),
///     model: Some("models/iron_sword.obj".to_string()),
///     meta_tags: HashMap::new(),
/// };
/// inventory.add_item(0, sword);
/// ```
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