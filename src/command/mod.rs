use crate::component;
use component::position::Position;
use crate::entity;
use entity::Entity;

/// Actions are processes that modify the game world.
#[derive(Clone, Copy, Debug)]
pub enum Command {
    /// Move the entity argument to the specified position.
    Move(Position),
}

/// Actions are processes that modify the game world.
impl Command {

    /// Perform the action.
    pub fn execute(&self, entity: &mut Entity) {
        use Command::*;
        match self {
            Move(position) => {
                entity.move_to(position.x, position.y, position.z);
            }
        }
    }

}
