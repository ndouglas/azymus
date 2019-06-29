use std::collections::VecDeque;
use specs::*;
use crate::command::Command;

/// Something that can act.
#[derive(Clone, Component, Debug)]
#[storage(VecStorage)]
pub struct CommandQueue {
    /// The enqueued actions for this entity.
    pub queue: VecDeque<Command>,
}

impl Default for CommandQueue {
    fn default() -> Self {
        CommandQueue {
            queue: VecDeque::new(),
        }
    }
}
