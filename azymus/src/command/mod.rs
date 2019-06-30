use specs::*;

/// Compass directions.
#[derive(Clone, Copy, Debug)]
pub enum CompassDirection {
    /// North.
    North,
    /// South.
    South,
    /// East.
    East,
    /// West.
    West,
}

/// The commands.
#[derive(Clone, Copy, Component, Debug)]
pub enum Command {
    /// Walk.
    Walk(CompassDirection),
    /// Wait.
    Wait,
}
