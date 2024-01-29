pub mod entity;
pub mod pokemon_entity;
pub mod trainer_entity;
pub mod overworld_item_entity;

use crate::entitys::pokemon_entity::PokemonEntity;
use crate::entitys::trainer_entity::TrainerEntity;
use crate::entitys::overworld_item_entity::OverworldItemEntity;

pub static mut BASIC_ENTITIES: Vec<PokemonEntity> = Vec::new();
pub static mut OVERWORLD_ITEMS: Vec<OverworldItemEntity> = Vec::new();
pub static mut TRAINERS: Vec<TrainerEntity> = Vec::new();
#[allow(private_interfaces)]
pub static mut ALL_ENTITIES: Vec<AnyEntity> = Vec::new();


// Enum to represent different entity types
#[allow(dead_code)]
enum AnyEntity {
    Pokemon(PokemonEntity),
    OverworldItem(OverworldItemEntity),
    Trainer(TrainerEntity),
}