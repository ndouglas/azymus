use std::collections::VecDeque;
use specs::*;
use crate::command::Command;

/// Something that can act.
#[derive(Clone, Component, Debug)]
#[storage(VecStorage)]
pub struct Actor {
    /// The current energy of the entity.
    pub energy: i32,
    /// The speed acquired by the entity each turn.
    pub speed: i32,
    /// The enqueued commands for this entity.
    pub command_queue: VecDeque<Command>,
}
