use crate::command;
use command::Command;
use command::CompassDirection;
use crate::component;
use component::position::Position;
use crate::game;
use game::Game;


/// Algorithms used to vend commands when given a context.
#[derive(Clone, Copy, Debug)]
pub enum Algorithm {
    /// Just move South.
    JustMoveSouth,
    /// Approach the player.
    ApproachPlayer,
}

/// Algorithms used to vend commands when given a context.
impl Algorithm {

    /// Get the command that this agent would like to execute.
    pub fn get_command(self, time: i32, id: usize, game: &Game) -> Option<Command> {
        trace!("Entering Algorithm::get_command().");
        use Algorithm::*;
        if time <= 0 {
            return None;
        }
        match self {
            JustMoveSouth => {
                Some(Command::Walk(CompassDirection::South))
            },
            ApproachPlayer => {
                let player = &game.entities[game.player_id];
                let player_position = player.position.unwrap();
                command_to_move_towards(id, player_position.x, player_position.y, game)
            },
        }
    }

}

fn get_direction_to(id: usize, target_x: i32, target_y: i32, game: &Game) -> Option<CompassDirection> {
    let entity = &game.entities[id];
    if let Some(position1) = entity.position {
        let position2 = Position {
            w: position1.w,
            x: target_x,
            y: target_y,
            z: position1.z,
        };
        return position1.direction_to(position2);
    }
    None
}

fn command_to_move_towards(id: usize, target_x: i32, target_y: i32, game: &Game) -> Option<Command> {
    if let Some(compass_direction) = get_direction_to(id, target_x, target_y, game) {
        return Some(Command::Walk(compass_direction));
    }
    None
}
