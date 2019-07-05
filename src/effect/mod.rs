use crate::component;
use component::position::Position;
use crate::game;
use game::Game;

/// A direct modification of the game world.
#[derive(Clone, Copy, Debug)]
pub enum Effect {
    /// Move the specified entity from one position to another.
    EntityMoves(Position, Position),
}

/// A direct modification of the game world.
impl Effect {

    /// Perform the effect.
    pub fn execute(&self, id: usize, game: &mut Game) {
        use Effect::*;
        match self {
            EntityMoves(position1, position2) => {
                let mut entity = &mut game.entities[id];
                debug!("Moving entity {} from ({}, {}) to ({}, {}).", entity.name, position1.x, position1.y, position2.x, position2.y);
                entity.position = Some(*position2);
                if let Some(fov) = &mut entity.field_of_view {
                    fov.update(position2.x, position2.y);
                }
            }
        }
    }

}
