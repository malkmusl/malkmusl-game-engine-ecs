use entitys::entity::Entity;
use entitys::trainer_entity::TrainerEntity;
use entitys::TRAINERS;
use malkmusl_log::{print_error, print_info, print_info_debug};
use std::future::Future;
use std::pin::Pin;
use std::thread::{self};
use std::time::Duration;


use crate::entitys;

use super::{System, MAX_TRAINER_ENTITIES, TRAINER_COUNT};

pub struct TrainerSpawnSystem;

impl System for TrainerSpawnSystem {
    fn enable(&self, enable: bool) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async move {
            if enable {
                let spawn_handle = tokio::spawn(TrainerSpawnSystem::spawn());
                let despawn_handle = tokio::spawn(TrainerSpawnSystem::despawn());

                // Wait for both tasks to complete
                tokio::try_join!(spawn_handle, despawn_handle)
                    .expect(print_error("Error waiting for tasks to complete"));
            }
        })
    }
}

impl TrainerSpawnSystem {
    async fn spawn() {
        unsafe {
            let spawn_trainer_handle = thread::spawn(|| {
                print_info("Starting Spawn Trainer Handle...");
                loop {
                    // Trainer entity spawning
                    while *TRAINER_COUNT.lock().unwrap() < MAX_TRAINER_ENTITIES {
                        let trainer_entity = TrainerEntity::spawn_entity();
                        TRAINERS.push(trainer_entity.clone());
                        *TRAINER_COUNT.lock().unwrap() += 1;
                        let message = format!("{} Spawned Trainer Entity - ID: {}, Name: {}, Lifetime: ", trainer_entity.id(), trainer_entity.name(), trainer_entity.lifetime());
                        print_info_debug(&message);
                    }
                    // Sleep for 10 seconds before the next iteration
                    thread::sleep(Duration::from_millis(100));
                }
            });
            spawn_trainer_handle.join().expect(print_error("Unable to start Spawn Trainer Handle!"));
        }
    }
    
    async fn despawn() {
        unsafe {
            // Spawn thread for entity despawning
            let despawn_trainer_handle = thread::spawn(|| {
                print_info("Starting Despawn Trainer Handle...");
                loop {
                    // Tick and despawn entities
                    let mut index = 0;
                    while index < TRAINERS.len() {
                        let entity = &mut TRAINERS[index];
                        entity.tick();
                        entity.despawn();
    
                        // Increment the index if the entity is not removed
                        if TRAINERS.len() > index && TRAINERS[index].lifetime > 0 {
                            index += 1;
                        }
                    }
    
                    // Sleep for a short duration before the next iteration
                    thread::sleep(Duration::from_millis(100));
                }
            });
            despawn_trainer_handle.join().expect(print_error("Unable to start Despawn Trainer Handle!"));
        }
    }
}

