use specs::*;

/// The position of an object in the world.
#[derive(Clone, Copy, Component, Debug)]
#[storage(VecStorage)]
pub struct Actor {
    /// The current energy of the entity.
    pub current: i32,
    /// The speed acquired by the entity each turn.
    pub speed: i32,
}
