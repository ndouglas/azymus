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
                println!("rawr");
                if energy < 120 {
                    return None;
                }
                return Some(Command::Move(CompassDirection::South));
            },
            Troll                => {
                println!("Mrf.");
                if energy < 120 {
                }
                return Some(Command::Move(CompassDirection::South));
            },
        }
    }

}
