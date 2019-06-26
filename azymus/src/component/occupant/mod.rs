use specs::*;

/// Prevents you from moving into the same position.
#[derive(Clone, Copy, Component, Debug)]
pub struct Occupant;
