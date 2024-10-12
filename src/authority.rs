// authority.rs
use std::sync::{Arc, Mutex};
use serde_json::json;
use serde::{Deserialize, Serialize};
use tracing::info;

// Authority levels ranging from 1 (Normal Player) to 9 (System Administrator/Boss)
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum AuthorityLevel {
    Level1, // Normal Player
    Level2, // VIP Player
    Level3, // Chat Moderator
    Level4, // Game Master
    Level5, // NPC System
    Level6, // Chief Game Master
    Level7, // Developer
    Level8, // Chief Developer
    Level9, // System Administrator/Boss
}

// Struct representing the player with their authority level
#[derive(Debug, Clone)]
pub struct PlayerAuthority {
    pub player_id: u64,
    pub authority: AuthorityLevel,
}

// Struct to control the authority system toggle
pub struct AuthoritySystem {
    pub enabled: bool,
    pub players: Arc<Mutex<Vec<PlayerAuthority>>>,
}

impl AuthoritySystem {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            players: Arc::new(Mutex::new(Vec::new())),
        }
    }

    // Add player with authority level
    pub fn add_player(&self, player_id: u64, authority: AuthorityLevel) {
        if self.enabled {
            let mut players = self.players.lock().unwrap();
            players.push(PlayerAuthority { player_id, authority });
            info!("Player {} added with authority {:?}", player_id, authority);
        } else {
            info!("Authority system is disabled, not adding player.");
        }
    }

    // Check if a player has the required authority level
    pub fn has_authority(&self, player_id: u64, required_level: AuthorityLevel) -> bool {
        if !self.enabled {
            info!("Authority system is disabled, allowing all actions.");
            return true; // If disabled, allow all actions
        }

        let players = self.players.lock().unwrap();
        if let Some(player) = players.iter().find(|p| p.player_id == player_id) {
            player.authority >= required_level
        } else {
            false
        }
    }
}
