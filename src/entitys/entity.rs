use uuid::Uuid;
pub trait Entity {
    fn despawn(&mut self);
    fn id(&self) -> Uuid;
    fn name(&self) -> &str;
    fn lifetime(&self) -> u128;
    fn tick(&mut self);
    fn spawn_entity() -> Self
    where
        Self: Sized;
}