use crate::game;
use game::Game;
use crate::math;
use math::geometry::cell::Cell;

/// Move an entity from one cell to another.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct MoveEntity {
    /// The id of the entity being moved.
    pub id: usize,
    /// The cell where the entity begins.
    pub cell1: Cell,
    /// The cell where the entity ends.
    pub cell2: Cell,
}

/// Move an entity from one cell to another.
impl MoveEntity {

    /// Constructor.
    pub fn new(id: usize, cell1: Cell, cell2: Cell) -> Self {
        MoveEntity {
            id,
            cell1,
            cell2,
        }
    }

    /// Execute the move.
    pub fn execute(&self, game: &mut Game) {
        trace!("Entering MoveEntity::execute().");
        game.map.entity_map
            .move_entity_id(self.id, &self.cell1, &self.cell2);
        let mut entity = &mut game.get_entity_mut(self.id);
        entity.cell = self.cell2;
        trace!("Exiting MoveEntity::execute().");
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::entity;
    use entity::Entity;
    use crate::map;
    use map::Map;
    use map::tile_map::Factory as TileMapFactory;
    use crate::math;
    use math::geometry::cell::Cell;
    use crate::seed;
    use seed::get_rng_seed;

    /// Test and see if this makes sense.
    #[test]
    fn execute() {
        let seed = get_rng_seed();
        let width: usize = 4;
        let height: usize = 4;
        let inner_map = TileMapFactory::Empty.create(seed, width, height);
        let mut map = Map::new(inner_map);
        let mut entity = Entity::new("Test".to_string());
        entity.cell = Cell::new(0, 0);
        map.entity_map.insert_entity(entity);
        MoveEntity::new(0, Cell::new(0, 0), Cell::new(1, 1))
            .execute(&mut map);
        assert_eq!(Cell::new(1, 1), map.entity_map.vector[0].cell);
    }

}
