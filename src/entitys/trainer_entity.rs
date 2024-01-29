use malkmusl_log::print_info_debug;
use uuid::Uuid;
use rand::prelude::*;

use crate::sytsems::TRAINER_COUNT;

use super::entity::Entity;
use super::{AnyEntity, ALL_ENTITIES, TRAINERS};

#[derive(Clone)] // Add this line to derive the Clone trait
pub struct TrainerEntity {
    entity_id: Uuid,
    name: String,
    pub lifetime: u128,
}

impl Entity for TrainerEntity {
    fn id(&self) -> Uuid {
        self.entity_id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn spawn_entity() -> Self {
        let mut rng = rand::thread_rng();
        TrainerEntity {
            entity_id: Uuid::new_v4(),
            name: String::from("Unnamed Trainer"),
            lifetime: rng.gen_range(100..=600),
        }
    }

    fn despawn(&mut self) {
        if self.lifetime <= 0 {
            // Perform despawning logic here
            let message = format!("Trainer Entity with ID {} despawned.", self.entity_id);
            print_info_debug(&message);
    
            // Find the index of the entity in BASIC_ENTITIES vector
            if let Some(index) = unsafe { TRAINERS.iter().position(|entity| entity.id() == self.id()) } {
                // Remove the entity from BASIC_ENTITIES
                unsafe { TRAINERS.remove(index) };
                *TRAINER_COUNT.lock().unwrap() -= 1;
            }
    
            // Find the index of the entity in ALL_ENTITIES vector
            if let Some(index) = unsafe {
                ALL_ENTITIES
                    .iter()
                    .position(|any_entity| {
                        if let AnyEntity::Trainer(entity) = any_entity {
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