use crate::component;
use component::position::Position;
use crate::game;
use game::Game;

/// Actions are processes that modify the game world.
#[derive(Clone, Copy, Debug)]
pub enum Action {
    /// Move the entity argument to the specified position.
    Move(Position),
}

/// Actions are processes that modify the game world.
impl Action {

    /// Get the cost of performing this action.
    pub fn get_cost(&self, _id: usize, _game: &Game) -> i32 {
        120
    }

    /// Perform the action.
    pub fn execute(&self, id: usize, game: &mut Game) {
        use Action::*;
        match self {
            Move(position) => {
                let entity = &mut game.entities[id];
                entity.move_to(position.x, position.y, position.z);
            }
        }
    }

}
