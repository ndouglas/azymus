use crate::command;
use command::Command;
use command::CompassDirection;
use crate::component;
use component::position::Position;
use crate::game;
use game::Game;

/// Something that can act autonomously.
#[derive(Clone, Copy, Debug)]
pub struct Agent {
    /// The algorithm used by this agent.
    pub algorithm: Algorithm,
}

/// Algorithms used to vend commands when given a context.
#[derive(Clone, Copy, Debug)]
pub enum Algorithm {
    /// Just move South.
    JustMoveSouth,
    /// Approach the player.
    ApproachPlayer,
    /// Approach and fight the player.
    ApproachAndFightPlayer,
    /// Be a chicken.
    BeChicken,
    /// Be a mushroom.
    BeMushroom,
    /// Just be moss.
    BeMoss,
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
                if let Some(player_position) = &player.position {
                    return command_to_move_towards(id, player_position, game);
                }
                None
            },
            ApproachAndFightPlayer => {
                let player = &game.entities[game.player_id];
                if let Some(player_position) = &player.position {
                    let entity = &game.entities[id];
                    if let Some(entity_position) = &entity.position {
                        if entity_position.distance_to(player_position) >= 2.0 {
                            return command_to_move_towards(id, player_position, game)
                        } else {
                            return command_to_attack(id, player_position, game);
                        }
                    }
                }
                None
            },
            BeChicken => {
                None
            },
            BeMushroom => {
                None
            },
            BeMoss => {
                None
            },
        }
    }

}

fn get_direction_to(id: usize, position: &Position, game: &Game) -> Option<CompassDirection> {
    let entity = &game.entities[id];
    if let Some(entity_position) = &entity.position {
        return entity_position.direction_to(position);
    }
    None
}

fn command_to_move_towards(id: usize, position: &Position, game: &Game) -> Option<Command> {
    if let Some(compass_direction) = get_direction_to(id, position, game) {
        return Some(Command::Walk(compass_direction));
    }
    None
}

fn command_to_attack(id: usize, position: &Position, game: &Game) -> Option<Command> {
    if let Some(compass_direction) = get_direction_to(id, position, game) {
        return Some(Command::MeleeAttack(compass_direction));
    }
    None
}
