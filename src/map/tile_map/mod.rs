use crate::math;
use math::geometry::cell::Cell;
use math::geometry::rectangle::{Rectangle, Rectangular};
use crate::seed;
use seed::SeedType;
use crate::tile;
use tile::Tile;

/// Empty...
pub mod empty;
/// Tutorial...
pub mod simple;
/// Random...
pub mod random;

/// The map type.
pub type TileMapType = Vec<Vec<Tile>>;

/// The tile map.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct TileMap {
    /// The width.
    pub width: usize,
    /// The height.
    pub height: usize,
    /// The actual tile vector.
    pub vector: TileMapType,
    /// The starting position cell.
    pub start: Cell,
}

/// The tile map.
impl TileMap {

    /// Constructor.
    pub fn new(tile_map: TileMapType, start: Cell) -> Self {
        TileMap {
            width: tile_map.len(),
            height: tile_map[0].len(),
            vector: tile_map,
            start: start,
        }
    }

    /// Get tile at index.
    pub fn get_tile(&self, cell: &Cell) -> Option<&Tile> {
        if self.as_rectangle().contains_cell(cell) {
            Some(&self.vector[cell.x][cell.y])
        } else {
            None
        }
    }

    /// Retrieve the entity identifiers at multiple cells.
    pub fn get_tiles(&self, cells: &[Cell]) -> Vec<&Tile> {
        let mut result = Vec::new();
        for cell in cells {
            if let Some(tile) = self.get_tile(cell) {
                result.push(tile);
            }
        }
        result
    }

    /// Retrieve the entity identifiers in the Von Neumann neighborhood.
    pub fn get_tiles_in_von_neumann_neighborhood(&self, cell: &Cell) -> Vec<&Tile> {
        let cells = cell.get_von_neumann_neighborhood(&self.as_rectangle());
        self.get_tiles(&cells)
    }

    /// Retrieve the entity identifiers in the Von Neumann neighborhood.
    pub fn get_tiles_in_moore_neighborhood(&self, cell: &Cell) -> Vec<&Tile> {
        let cells = cell.get_moore_neighborhood(&self.as_rectangle());
        self.get_tiles(&cells)
    }

}

/// Allows us to create a rectangle representation of this map.
impl Rectangular for TileMap {

    /// Create a rectangle from this object.
    fn as_rectangle(&self) -> Rectangle {
        Rectangle {
            x: 0,
            y: 0,
            width: self.width,
            height: self.height,
        }
    }

}

/// The type of function of a tile map generator.
pub type TileMapGeneratorType = fn(SeedType, usize, usize) -> (TileMapType, Cell);

/// The factory.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Factory {
    /// All floor, useful(?) for testing.
    Empty,
    /// The simple algorithm used by this Rust roguelike tutorial.
    Simple,
    /// This should be a lot cooler when I don't have exactly two choices.
    Random,
}


/// The factory.
impl Factory {

    /// Create a tile map.
    pub fn create(&self, seed: SeedType, width: usize, height: usize) -> TileMap {
        use Factory::*;
        let generate_tile_map: TileMapGeneratorType = match self {
            Empty => empty::generate_tile_map,
            Simple => simple::generate_tile_map,
            Random => random::generate_tile_map,
        };
        let (inner_map, start) = generate_tile_map(seed, width, height);
        TileMap::new(inner_map, start)
    }

}
