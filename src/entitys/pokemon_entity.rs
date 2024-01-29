use malkmusl_log::print_info_debug;
use uuid::Uuid;
use rand::prelude::*;

use crate::sytsems::POKEMON_COUNT;
use super::entity::Entity;
use super::{AnyEntity, ALL_ENTITIES, BASIC_ENTITIES};

#[derive(Clone)]
#[allow(dead_code)]
pub struct PokemonEntity {
    entity_id: Uuid,
    dex_id: u128,
    name: String,
    pub lifetime: u128,
    pub level: i32,
    position: (f64, f64),
}

impl Entity for PokemonEntity {
    fn id(&self) -> Uuid {
        self.entity_id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn lifetime(&self) -> u128 {
        self.lifetime
    }

    fn spawn_entity() -> Self {
        let mut rng = rand::thread_rng();
        PokemonEntity {
            entity_id: Uuid::new_v4(),
            dex_id: rng.gen_range(1..=493),
            name: String::from("Pokemon Entity"),
            lifetime: rng.gen_range(1000..=6000),
            level: rng.gen_range(1..=100),
            position: (0.0, 0.0),
        }
    }

    fn despawn(&mut self) {
        if self.lifetime <= 0 {
            // Perform despawning logic here
            let message = format!("Pokemon Entity with ID {} despawned.", self.entity_id);
            print_info_debug(&message);

    
            // Find the index of the entity in BASIC_ENTITIES vector
            if let Some(index) = unsafe { BASIC_ENTITIES.iter().position(|entity| entity.id() == self.id()) } {
                // Remove the entity from BASIC_ENTITIES
                unsafe { BASIC_ENTITIES.remove(index) };
                *POKEMON_COUNT.lock().unwrap() -= 1;
            }
    
            // Find the index of the entity in ALL_ENTITIES vector
            if let Some(index) = unsafe {
                ALL_ENTITIES
                    .iter()
                    .position(|any_entity| {
                        if let AnyEntity::Pokemon(entity) = any_entity {
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
    
    fn tick(&mut self) {
        if self.lifetime > 0 {
            self.lifetime -= 1;
        }
        
    }
}