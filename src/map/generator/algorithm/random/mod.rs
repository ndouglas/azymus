use super::super::MapGeneratorReturnType;
use crate::entity;
use entity::Entity;
use crate::seed;
use seed::SeedType;

/// Generate the map.
pub fn generate_map(seed: SeedType, width: i32, height: i32, level: i32, objects: &mut Vec<Entity>) -> MapGeneratorReturnType {
    super::simple::generate_map(seed, width, height, level, objects) // Chosen at random.
}
