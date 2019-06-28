use specs::*;
use crate::agent::Agent as AgentType;

/// Something that can act independently of player control.
#[derive(Clone, Copy, Component, Debug)]
#[storage(VecStorage)]
pub struct Agent {
    /// The behavior system for this agent.
    pub agent: AgentType,
}
