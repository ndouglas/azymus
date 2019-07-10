use super::super::MapGeneratorReturnType;
use crate::entity;
use entity::Entity;
use crate::seed;
use seed::SeedType;
use seed::RngType;

/// Generate the map.
pub fn generate_map(seed: SeedType, rng: &mut RngType, width: i32, height: i32, level: i32, objects: &mut Vec<Entity>) -> MapGeneratorReturnType {
    super::simple::generate_map(seed, rng, width, height, level, objects) // Chosen at random.
}
