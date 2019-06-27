use specs::*;

/// Prevents you from moving into the same position.
#[derive(Clone, Component, Debug)]
pub struct Name {
    /// The name of this entity.
    pub name: String,
}
