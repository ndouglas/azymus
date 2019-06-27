use specs::*;
use crate::action::*;
use crate::component;
use component::position::Position;

/// The commands.
#[derive(Clone, Copy, Debug)]
pub enum Command {
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

/// Handle a command.
pub fn handle_command(command: Command, entity: Entity, world: &mut World) {
    if let Some(action) = get_command_action(command, entity, world) {
        if let Some(action) = get_permitted_action(action, entity, world) {
            action.execute(entity, world);
        }
    }
}
