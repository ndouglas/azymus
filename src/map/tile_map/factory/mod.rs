use super::TileMapType;
use crate::seed::SeedType;

/// Empty...
pub mod empty;
/// Tutorial...
pub mod simple;
/// Random...
pub mod random;

/// The type of function of a tile map generator.
pub type TileMapGeneratorType = fn() -> TileMapType;

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
        }
        generate_tile_map(seed, width, height)
    }

}






/*
use super::MapGeneratorReturnType;
use super::MapGeneratorType;
use crate::entity;
use entity::Entity;
use crate::seed;
use seed::SeedType;
use seed::RngType;

/// Empty...
pub mod empty;
/// Tutorial...
pub mod simple;
/// Random...
pub mod random;

/// The distinct algorithms for generating maps.
#[derive(Clone, Copy, Debug)]
pub enum Algorithm {
    /// All floor, useful(?) for testing.
    Empty,
    /// The simple algorithm used by this Rust roguelike tutorial.
    Simple,
    /// This should be a lot cooler when I don't have exactly two choices.
    Random,
}

/// The factory for generating maps.
impl Factory {

    /// Generate the map.
    pub fn create(&self, seed: SeedType, rng: &mut RngType, width: i32, height: i32, level: i32, objects: &mut Vec<Entity>) -> MapGeneratorReturnType {
        use Algorithm::*;
        let generate_map: MapGeneratorType = match self {
            Empty => empty::generate_map,
            Simple => simple::generate_map,
            Random => random::generate_map,
        };
        generate_map(seed, rng, width, height, level, objects)
    }

}
*/
