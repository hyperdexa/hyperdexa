use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::fs;
use tokio::sync::RwLock;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct InventoryItem {
    id: String,
    quantity: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PlayerInventory {
    username: String,
    items: Vec<InventoryItem>,
}

type InventoryStore = Arc<RwLock<HashMap<String, PlayerInventory>>>;

impl PlayerInventory {
    fn add_item(&mut self, item: InventoryItem) {
        if let Some(existing_item) = self.items.iter_mut().find(|i| i.id == item.id) {
            existing_item.quantity += item.quantity;
        } else {
            self.items.push(item);
        }
    }

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

async fn load_inventory(username: &str) -> Option<PlayerInventory> {
    let file_path = format!("data/{}.json", username);
    match fs::read_to_string(&file_path).await {
        Ok(data) => serde_json::from_str(&data).ok(),
        Err(_) => None,
    }
}

async fn save_inventory(inventory: &PlayerInventory) {
    let file_path = format!("data/{}.json", inventory.username);
    if let Ok(data) = serde_json::to_string(inventory) {
        let _ = fs::write(file_path, data).await;
    }
}

async fn get_or_create_inventory(
    username: &str,
    inventory_store: &InventoryStore,
) -> Arc<RwLock<PlayerInventory>> {
    let mut store = inventory_store.write().await;

    if let Some(inventory) = store.get(username) {
        return Arc::new(RwLock::new(inventory.clone()));
    }

    let inventory = load_inventory(username).await.unwrap_or_else(|| PlayerInventory {
        username: username.to_string(),
        items: vec![],
    });

    let inventory_arc = Arc::new(RwLock::new(inventory.clone()));
    store.insert(username.to_string(), inventory);
    inventory_arc
}

async fn handle_player_login(username: &str, inventory_store: &InventoryStore) {
    let inventory = get_or_create_inventory(username, inventory_store).await;
    println!("Player '{}' logged in with inventory: {:?}", username, inventory.read().await);
}

async fn update_inventory(
    username: &str,
    item: InventoryItem,
    inventory_store: &InventoryStore,
) {
    let inventory = get_or_create_inventory(username, inventory_store).await;

    {
        let mut inventory_lock = inventory.write().await;
        inventory_lock.add_item(item.clone());
        println!("Updated inventory for '{}': {:?}", username, inventory_lock);
    }

    save_inventory(&inventory.read().await).await;
}

#[tokio::main]
async fn main() {
    let inventory_store: InventoryStore = Arc::new(RwLock::new(HashMap::new()));

    // Simulating player logins and inventory updates
    handle_player_login("PlayerOne", &inventory_store).await;

    let new_item = InventoryItem {
        id: "gold_pickaxe".to_string(),
        quantity: 1,
    };
    update_inventory("PlayerOne", new_item, &inventory_store).await;

    handle_player_login("AnotherPlayer", &inventory_store).await;

    let another_item = InventoryItem {
        id: "iron_sword".to_string(),
        quantity: 2,
    };
    update_inventory("AnotherPlayer", another_item, &inventory_store).await;
}
