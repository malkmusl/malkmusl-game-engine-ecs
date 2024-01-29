use std::{future::Future, pin::Pin, sync::Mutex};

pub mod pokemon_spawn_system;
pub mod trainer_spawn_system;
pub mod overworld_item_spawn_system;

const MAX_POKEMON_ENTITIES: usize = 60;
const MAX_TRAINER_ENTITIES: usize = 5;
const MAX_OVERWORLD_ITEM_ENTITIES: usize = 20;
lazy_static::lazy_static! {
    pub static ref POKEMON_COUNT: Mutex<usize> = Mutex::new(0);
    pub static ref TRAINER_COUNT: Mutex<usize> = Mutex::new(0);
    pub static ref OVERWORLD_ITEM_COUNT: Mutex<usize> = Mutex::new(0);
}

#[async_trait::async_trait]
pub trait System {
    fn enable(&self, enable: bool) -> Pin<Box<dyn Future<Output = ()> + Send>>;
}