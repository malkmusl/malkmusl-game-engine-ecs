use uuid::Uuid;
use rand::prelude::*;

use crate::console::print_info_debug;
use crate::sytsems::OVERWORLD_ITEM_COUNT;

use super::entity::Entity;
use super::{AnyEntity, ALL_ENTITIES, OVERWORLD_ITEMS};

#[derive(Clone)] // Add this line to derive the Clone trait
pub struct OverworldItemEntity {
    entity_id: Uuid,
    name: String,
    pub lifetime: u128,
}

impl Entity for OverworldItemEntity {
    fn id(&self) -> Uuid {
        self.entity_id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn spawn_entity() -> Self {
        let mut rng = rand::thread_rng();
        OverworldItemEntity {
            entity_id: Uuid::new_v4(),
            name: String::from("Unnamed Overworld Item"),
            lifetime: rng.gen_range(10..=60),
        }
    }

    fn despawn(&mut self) {
        if self.lifetime <= 0 {
            // Perform despawning logic here
            let message = format!("Overworld Item Entity with ID {} despawned.", self.entity_id);
            print_info_debug(&message);
    
            // Find the index of the entity in BASIC_ENTITIES vector
            if let Some(index) = unsafe { OVERWORLD_ITEMS.iter().position(|entity| entity.id() == self.id()) } {
                // Remove the entity from BASIC_ENTITIES
                unsafe { OVERWORLD_ITEMS.remove(index) };
                *OVERWORLD_ITEM_COUNT.lock().unwrap() -= 1;
            }
    
            // Find the index of the entity in ALL_ENTITIES vector
            if let Some(index) = unsafe {
                ALL_ENTITIES
                    .iter()
                    .position(|any_entity| {
                        if let AnyEntity::OverworldItem(entity) = any_entity {
                            entity.id() == self.id()
                        } else {
                            false
                        }
                    })
            } {
                // Remove the entity from ALL_ENTITIES
                unsafe { ALL_ENTITIES.remove(index) };
            }
        }
    }

    fn lifetime(&self) -> u128 {
        self.lifetime
    }

    fn tick(&mut self) {
        self.lifetime -= 1;
    }
}