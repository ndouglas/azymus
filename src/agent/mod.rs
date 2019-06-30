use crate::command::Command;
use crate::command::CompassDirection;

/// Orc behavior.
pub mod orc;

/// The commands.
#[derive(Clone, Copy, Debug)]
pub enum Agent {
    /// Classic orc.
    Orc,
    /// Stupid troll.
    Troll,
}

impl Agent {

    /// Get action.
    ///
    /// Returns the action that the entity would like to perform next.
    pub fn get_next_command(self, _seed: i64, energy: i32) -> Option<Command> {
        use Agent::*;
        match self {
            Orc                  => {
                if energy < 120 {
                    return None;
                }
                return Some(Command::Walk(CompassDirection::South));
            },
            Troll                => {
                if energy < 120 {
                    return None;
                }
                return Some(Command::Walk(CompassDirection::South));
            },
        }
    }

}
