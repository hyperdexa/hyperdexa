use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, Write},
    path::Path,
};
use tokio::sync::RwLock;

// Player and NPC data models
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InventoryItem {
    pub id: String,
    pub quantity: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub name: String,
    pub inventory: Vec<InventoryItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NPC {
    pub name: String,
    pub inventory: Vec<InventoryItem>,
}

// Game Manager: Handles all game entities
pub struct GameManager {
    pub players: HashMap<String, Player>,
    pub npcs: HashMap<String, NPC>,
}

impl GameManager {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            npcs: HashMap::new(),
        }
    }

    // Load a player's inventory
    pub async fn load_player(&mut self, name: &str) {
        let path = format!("data/players/{}.json", name);
        if Path::new(&path).exists() {
            let file = File::open(&path).expect("Failed to open player file");
            let player: Player =
                serde_json::from_reader(file).expect("Failed to deserialize player data");
            self.players.insert(name.to_string(), player);
        } else {
            // Initialize a new player if not found
            self.players.insert(
                name.to_string(),
                Player {
                    name: name.to_string(),
                    inventory: Vec::new(),
                },
            );
        }
    }

    // Save all player and NPC data
    pub async fn save_all_data(&self) {
        // Save players
        for (name, player) in &self.players {
            let path = format!("data/players/{}.json", name);
            let mut file = File::create(&path).expect("Failed to create player file");
            serde_json::to_writer_pretty(&mut file, player)
                .expect("Failed to serialize player data");
        }

        // Save NPCs
        for (name, npc) in &self.npcs {
            let path = format!("data/npcs/{}.json", name);
            let mut file = File::create(&path).expect("Failed to create NPC file");
            serde_json::to_writer_pretty(&mut file, npc).expect("Failed to serialize NPC data");
        }
    }

    // Load an NPC's inventory
    pub async fn load_npc(&mut self, name: &str) {
        let path = format!("data/npcs/{}.json", name);
        if Path::new(&path).exists() {
            let file = File::open(&path).expect("Failed to open NPC file");
            let npc: NPC = serde_json::from_reader(file).expect("Failed to deserialize NPC data");
            self.npcs.insert(name.to_string(), npc);
        } else {
            // Initialize a new NPC if not found
            self.npcs.insert(
                name.to_string(),
                NPC {
                    name: name.to_string(),
                    inventory: Vec::new(),
                },
            );
        }
    }
}

// Add inventory modification methods
impl Player {
    pub fn add_item(&mut self, item_id: &str, quantity: u32) {
        if let Some(item) = self.inventory.iter_mut().find(|i| i.id == item_id) {
            item.quantity += quantity;
        } else {
            self.inventory.push(InventoryItem {
                id: item_id.to_string(),
                quantity,
            });
        }
    }
}

impl NPC {
    pub fn add_item(&mut self, item_id: &str, quantity: u32) {
        if let Some(item) = self.inventory.iter_mut().find(|i| i.id == item_id) {
            item.quantity += quantity;
        } else {
            self.inventory.push(InventoryItem {
                id: item_id.to_string(),
                quantity,
            });
        }
    }
}
