use specs::*;
use crate::action::Action;

/// The position of an object in the world.
#[derive(Clone, Component, Debug)]
#[storage(VecStorage)]
pub struct Actor {
    /// The current energy of the entity.
    pub energy: i32,
    /// The speed acquired by the entity each turn.
    pub speed: i32,
    /// The enqueued actions for this entity.
    pub queue: Vec<Action>,
}
