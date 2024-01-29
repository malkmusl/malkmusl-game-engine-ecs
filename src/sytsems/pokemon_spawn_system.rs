use console::{print_error, print_info, print_info_debug};
use entitys::entity::Entity;
use entitys::pokemon_entity::PokemonEntity;
use entitys::BASIC_ENTITIES;
use std::future::Future;
use std::pin::Pin;
use std::thread::{self};
use std::time::Duration;

use crate::{console, entitys};

use super::{System, MAX_POKEMON_ENTITIES, POKEMON_COUNT};

pub struct PokemonSpawnSystem;

impl System for PokemonSpawnSystem {
    fn enable(&self, enable: bool) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async move {
            if enable {
                let spawn_handle = tokio::spawn(PokemonSpawnSystem::spawn());
                let despawn_handle = tokio::spawn(PokemonSpawnSystem::despawn());

                // Wait for both tasks to complete
                tokio::try_join!(spawn_handle, despawn_handle)
                    .expect(print_error("Error waiting for tasks to complete"));
            }
        })
    }
}

impl PokemonSpawnSystem {
    async fn spawn() {
        unsafe {
            let spawn_pokemon_handle = thread::spawn(|| {
                print_info("Starting Spawn Pokemon Handle...");
                loop {
                    // Pokemon entity spawning
                    while *POKEMON_COUNT.lock().unwrap() < MAX_POKEMON_ENTITIES {
                        let pokemon_entity = PokemonEntity::spawn_entity();
                        BASIC_ENTITIES.push(pokemon_entity.clone());
                        *POKEMON_COUNT.lock().unwrap() += 1;
                        let message = format!("Spawned Pokemon Entity - ID: {}, Name: {}, Lifetime: {}, Level: {}", pokemon_entity.id(), pokemon_entity.name(), pokemon_entity.lifetime(), pokemon_entity.level);
                        print_info_debug(&message);
                    }
        
                    // Sleep for 10 seconds before the next iteration
                    thread::sleep(Duration::from_millis(100));
                }
            });
            spawn_pokemon_handle.join().expect(print_error("Unable to start Spawn Pokemon Handle!"));
        }
    }

    async fn despawn() {
        unsafe {
            // Spawn thread for entity despawning
            let despawn_pokemon_handle = thread::spawn(|| {
                print_info("Starting Despawn Pokemon Handle...");
                loop {
                    // Tick and despawn entities
                    let mut index = 0;
                    while index < BASIC_ENTITIES.len() {
                        let entity = &mut BASIC_ENTITIES[index];
                        entity.tick();
                        entity.despawn();
    
                        // Increment the index if the entity is not removed
                        if BASIC_ENTITIES.len() > index && BASIC_ENTITIES[index].lifetime > 0 {
                            index += 1;
                        }
                    }
                    // Sleep for a short duration before the next iteration
                    thread::sleep(Duration::from_millis(100));
                }
            });
            despawn_pokemon_handle.join().expect(print_error("Unable to start Despawn Pokemon Handle!"));
        }
    }
}










