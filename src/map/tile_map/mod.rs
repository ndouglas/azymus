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

/// The type of function of a tile map generator.
pub type TileMapGeneratorType = fn(SeedType, usize, usize) -> TileMapType;

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
    pub fn create(&self, seed: SeedType, width: usize, height: usize) -> TileMapType {
        use Factory::*;
        let generate_tile_map: TileMapGeneratorType = match self {
            Empty => empty::generate_tile_map,
            Simple => simple::generate_tile_map,
            Random => random::generate_tile_map,
        };
        generate_tile_map(seed, width, height)
    }

}
