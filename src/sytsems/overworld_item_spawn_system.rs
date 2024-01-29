use std::{future::Future, pin::Pin};

use console::{print_error, print_info, print_info_debug};
use entitys::entity::Entity;
use entitys::overworld_item_entity::OverworldItemEntity;

use entitys::OVERWORLD_ITEMS;
use tokio::time::{self, Duration};

use crate::{console, entitys};

use super::{System, MAX_OVERWORLD_ITEM_ENTITIES, OVERWORLD_ITEM_COUNT};

pub struct OverworldItemSpawnSystem;

// Implementation for OverworldItemSpawnSystem
impl System for OverworldItemSpawnSystem {
    fn enable(&self, enable: bool) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async move {
            if enable {
                let spawn_handle = tokio::spawn(OverworldItemSpawnSystem::spawn());
                let despawn_handle = tokio::spawn(OverworldItemSpawnSystem::despawn());

                // Wait for both tasks to complete
                tokio::try_join!(spawn_handle, despawn_handle)
                    .expect(print_error("Error waiting for tasks to complete"));
            }
        })
    }
}

impl OverworldItemSpawnSystem {
    async fn spawn() {
        unsafe {
            print_info("Starting Spawn Overworld Item Handle...");
            loop {
                // Overworld Item entity spawning
                while *OVERWORLD_ITEM_COUNT.lock().unwrap() < MAX_OVERWORLD_ITEM_ENTITIES {
                    let item_entity = OverworldItemEntity::spawn_entity();
                    OVERWORLD_ITEMS.push(item_entity.clone());
                    *OVERWORLD_ITEM_COUNT.lock().unwrap() += 1;
                    let message = format!(
                        "Spawned Overworld Item Entity - ID: {}, Name: {}",
                        item_entity.id(),
                        item_entity.name()
                    );
                    print_info_debug(&message);
    
                    // Sleep for 10 seconds before the next iteration
                    time::sleep(Duration::from_millis(100)).await;
                }
            }
        }
    }

    async fn despawn() {
        unsafe {
            print_info("Starting Despawn Overworld Item Handle...");
            loop {
                // Tick and despawn entities
                let mut index = 0;
                while index < OVERWORLD_ITEMS.len() {
                    let entity = &mut OVERWORLD_ITEMS[index];
                    entity.tick();
                    entity.despawn();
    
                    // Increment the index if the entity is not removed
                    if OVERWORLD_ITEMS.len() > index && OVERWORLD_ITEMS[index].lifetime > 0 {
                        index += 1;
                    }
                }
    
                // Sleep for a short duration before the next iteration
                time::sleep(Duration::from_millis(100)).await;
            }
        }
    }
}
