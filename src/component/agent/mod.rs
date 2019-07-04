use crate::agent;
use agent::Algorithm;

/// Something that can act autonomously.
#[derive(Clone, Copy, Debug)]
pub struct Agent {
    /// The algorithm used by this agent.
    pub algorithm: Algorithm,
}
