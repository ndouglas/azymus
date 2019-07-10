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

/// The distinct algorithms for generating maps.
impl Algorithm {

    /// Generate the map.
    pub fn generate_map(&self, seed: SeedType, rng: &mut RngType, width: i32, height: i32, level: i32, objects: &mut Vec<Entity>) -> MapGeneratorReturnType {
        use Algorithm::*;
        let generate_map: MapGeneratorType = match self {
            Empty => empty::generate_map,
            Simple => simple::generate_map,
            Random => random::generate_map,
        };
        generate_map(seed, rng, width, height, level, objects)
    }

}
