use specs::*;

/// Something that can act.
#[derive(Clone, Copy, Component, Debug)]
#[storage(VecStorage)]
pub struct Actor {
    /// The current energy of the entity.
    pub energy: i32,
    /// The speed acquired by the entity each turn.
    pub speed: i32,
}
