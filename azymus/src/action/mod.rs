use specs::*;

/// The actions.
#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
pub enum Action {
    /// Walk in some direction.
    Walk((i32, i32), (i32,i32)),
    /// Attack something in a specified square.
    Attack((i32, i32), (i32, i32)),
    /// Wait.
    Wait,
}

