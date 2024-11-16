use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::fs;
use tokio::sync::RwLock;

// Define an inventory item
#[derive(Serialize, Deserialize, Debug, Clone)]
struct InventoryItem {
    id: String,       // Item ID
    quantity: u32,    // Number of items
    is_stackable: bool, // Whether the item can stack
}

// Define an NPC's inventory
#[derive(Serialize, Deserialize, Debug, Clone)]
struct NpcInventory {
    npc_name: String,        // Unique NPC name or ID (e.g., "Merchant", "Goblin Scout")
    npc_type: String,        // Type of NPC (e.g., "Merchant", "Enemy", "Boss")
    items: Vec<InventoryItem>, // Items the NPC holds or sells
}

type NpcInventoryStore = Arc<RwLock<HashMap<String, NpcInventory>>>;

// Methods for NPC inventory
impl NpcInventory {
    // Add an item to the NPC's inventory
    fn add_item(&mut self, item: InventoryItem) {
        if let Some(existing_item) = self.items.iter_mut().find(|i| i.id == item.id && i.is_stackable) {
            existing_item.quantity += item.quantity;
        } else {
            self.items.push(item);
        }
    }

    // Remove an item from the NPC's inventory
    fn remove_item(&mut self, item_id: &str, quantity: u32) -> bool {
        if let Some(existing_item) = self.items.iter_mut().find(|i| i.id == item_id) {
            if existing_item.quantity > quantity {
                existing_item.quantity -= quantity;
                return true;
            } else if existing_item.quantity == quantity {
                self.items.retain(|i| i.id != item_id);
                return true;
            }
        }
        false
    }
}

// Load NPC inventory from file
async fn load_npc_inventory(npc_name: &str) -> Option<NpcInventory> {
    let file_path = format!("npc_data/{}.json", npc_name);
    match fs::read_to_string(&file_path).await {
        Ok(data) => serde_json::from_str(&data).ok(),
        Err(_) => None,
    }
}

// Save NPC inventory to file
async fn save_npc_inventory(inventory: &NpcInventory) {
    let file_path = format!("npc_data/{}.json", inventory.npc_name);
    if let Ok(data) = serde_json::to_string_pretty(inventory) {
        let _ = fs::write(file_path, data).await;
    }
}

// Retrieve or create inventory for an NPC
async fn get_or_create_npc_inventory(
    npc_name: &str,
    npc_type: &str,
    inventory_store: &NpcInventoryStore,
) -> Arc<RwLock<NpcInventory>> {
    let mut store = inventory_store.write().await;

    if let Some(inventory) = store.get(npc_name) {
        return Arc::new(RwLock::new(inventory.clone()));
    }

    let inventory = load_npc_inventory(npc_name).await.unwrap_or_else(|| NpcInventory {
        npc_name: npc_name.to_string(),
        npc_type: npc_type.to_string(),
        items: vec![],
    });

    let inventory_arc = Arc::new(RwLock::new(inventory.clone()));
    store.insert(npc_name.to_string(), inventory);
    inventory_arc
}

// Update NPC inventory
async fn update_npc_inventory(
    npc_name: &str,
    npc_type: &str,
    item: InventoryItem,
    inventory_store: &NpcInventoryStore,
) {
    let inventory = get_or_create_npc_inventory(npc_name, npc_type, inventory_store).await;

    {
        let mut inventory_lock = inventory.write().await;
        inventory_lock.add_item(item.clone());
        println!(
            "Updated inventory for NPC '{}': {:?}",
            npc_name, inventory_lock
        );
    }

    save_npc_inventory(&inventory.read().await).await;
}

// Example NPC initialization
async fn initialize_npc(npc_name: &str, npc_type: &str, inventory_store: &NpcInventoryStore) {
    let inventory = get_or_create_npc_inventory(npc_name, npc_type, inventory_store).await;
    println!("Initialized NPC '{}' of type '{}'", npc_name, npc_type);
    println!("Current Inventory: {:?}", inventory.read().await);
}

#[tokio::main]
async fn main() {
    // Store to manage NPC inventories
    let npc_inventory_store: NpcInventoryStore = Arc::new(RwLock::new(HashMap::new()));

    // Initialize some NPCs
    initialize_npc("Merchant", "Merchant", &npc_inventory_store).await;
    initialize_npc("Goblin Scout", "Enemy", &npc_inventory_store).await;

    // Add items to an NPC's inventory
    let merchant_item = InventoryItem {
        id: "healing_potion".to_string(),
        quantity: 10,
        is_stackable: true,
    };

    update_npc_inventory("Merchant", "Merchant", merchant_item, &npc_inventory_store).await;

    let goblin_item = InventoryItem {
        id: "goblin_knife".to_string(),
        quantity: 1,
        is_stackable: false,
    };

    update_npc_inventory("Goblin Scout", "Enemy", goblin_item, &npc_inventory_store).await;
}
