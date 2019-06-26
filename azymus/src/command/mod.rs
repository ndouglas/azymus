use specs::*;
use crate::action::Action;
use crate::component;
use component::position::Position;

/// The commands.
#[derive(Clone, Copy, Debug)]
pub enum Command {
    /// Quits the game.
    QuitGame,
    /// Toggles fullscreen.
    ToggleFullscreen,
    /// Move North.
    MoveNorth,
    /// Move South.
    MoveSouth,
    /// Move West.
    MoveWest,
    /// Move East.
    MoveEast,
}

/// Get command action.
///
/// Returns the attempted action corresponding to the command.
pub fn get_command_action(command: Command, entity: Entity, world: &World) -> Option<Action> {
    match command {
        Command::ToggleFullscreen           => { Some(Action::ToggleFullscreen) }
        Command::QuitGame                   => { Some(Action::QuitGame) },
        Command::MoveNorth                  => {
            let position_storage = world.read_storage::<Position>();
            if let Some(position) = position_storage.get(entity) {
                return Some(Action::Walk((position.x, position.y),(position.x, position.y - 1)));
            }
            None
        },
        Command::MoveSouth                  => {
            let position_storage = world.read_storage::<Position>();
            if let Some(position) = position_storage.get(entity) {
                return Some(Action::Walk((position.x, position.y),(position.x, position.y + 1)));
            }
            None
        },
        Command::MoveWest                   => {
            let position_storage = world.read_storage::<Position>();
            if let Some(position) = position_storage.get(entity) {
                return Some(Action::Walk((position.x, position.y),(position.x - 1, position.y)));
            }
            None
        },
        Command::MoveEast                   => {
            let position_storage = world.read_storage::<Position>();
            if let Some(position) = position_storage.get(entity) {
                return Some(Action::Walk((position.x, position.y),(position.x + 1, position.y)));
            }
            None
        },
    }
}
