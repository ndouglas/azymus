use specs::*;

/// The position of an object in the world.
#[derive(Clone, Copy, Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    /// The x-coordinate of the object.
    pub x: i32,
    /// The y-coordinate of the object.
    pub y: i32,
}
