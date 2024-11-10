use serde::{Deserialize, Serialize};

/// Represents a crafter (e.g., a crafting station or profession).
///
/// # Fields
/// * `name`: The name of the crafter.
///
/// # Example
/// ```
/// let blacksmith = Crafter {
///     name: "Blacksmith".to_string(),
/// };
/// ```
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Crafter {
    pub name: String,
}