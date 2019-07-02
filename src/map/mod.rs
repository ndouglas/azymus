use tcod::console::*;
use crate::component;
use component::position::Position;
use crate::tile;
use tile::Tile;

/// The map generators.
pub mod generator;

/// The map type.
pub type MapType = Vec<Vec<Tile>>;

/// The map object.
#[derive(Clone, Debug)]
pub struct Map {
    /// The actual inner map.
    map: MapType,
}

/// The map object.
impl Map {

    /// Constructor.
    pub fn new(map: MapType) -> Self {
        Map {
            map: map,
        }
    }

    /// Render this entity's renderable at the current position.
    pub fn draw(&self, console: &mut Console) {
        trace!("Entering Map::draw().");
        for y in 0..console.height() {
            for x in 0..console.width() {
                self.map[x as usize][y as usize].draw(console);
            }
        }
        trace!("Exiting Map::draw().");
    }

}

/// Get a new map.
pub fn get_map(seed: i64, width: i32, height: i32, level: i32) -> (Map, Position) {
    let (inner_map, position) = generator::algorithm::Algorithm::Simple.generate_map(seed, width, height, level);
    let map = Map::new(inner_map);
    (map, position)
}
