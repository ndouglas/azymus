use crate::command;
use command::Command;
use command::CompassDirection;
use crate::game;
use game::Game;


/// Algorithms used to vend commands when given a context.
#[derive(Clone, Copy, Debug)]
pub enum Algorithm {
    /// Just move South.
    JustMoveSouth,

}

/// Algorithms used to vend commands when given a context.
impl Algorithm {

    /// Get the command that this agent would like to execute.
    pub fn get_command(self, time: i32, _id: usize, _game: &Game) -> Option<Command> {
        trace!("Entering Algorithm::get_command().");
        use Algorithm::*;
        match self {
            JustMoveSouth => {
                if time >= 120 {
                    Some(Command::Walk(CompassDirection::South))
                } else {
                    Some(Command::Wait)
                }
            }
        }
    }

}
