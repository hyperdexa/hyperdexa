mod game_data;

use game_data::{GameManager, Player, NPC};
use tokio::sync::RwLock;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Shared state for game data
    let game_manager = Arc::new(RwLock::new(GameManager::new()));

    // Example: Loading player and NPC data
    let player_name = "Terraria_Player";
    let npc_name = "Merchant";

    {
        // Load player data
        let mut manager = game_manager.write().await;
        manager.load_player(player_name).await;

        // Modify player inventory
        if let Some(player) = manager.players.get_mut(player_name) {
            player.add_item("Iron Sword", 1);
        }

        // Load NPC data
        manager.load_npc(npc_name).await;

        // Modify NPC inventory
        if let Some(npc) = manager.npcs.get_mut(npc_name) {
            npc.add_item("Health Potion", 5);
        }
    }

    // Save all data
    {
        let manager = game_manager.read().await;
        manager.save_all_data().await;
    }

    println!("Game data updated and saved!");
}
