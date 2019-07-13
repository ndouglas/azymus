use crate::component;
use component::position::Position;
use crate::entity;
use entity::Entity;
use crate::seed;
use seed::SeedType;
use seed::RngType;
use crate::tile;
use tile::factory::Factory as TileFactory;
use super::super::MapGeneratorReturnType;

/// Generate the map.
pub fn generate_map(seed: SeedType, _rng: &mut RngType, width: i32, height: i32, level: i32, _objects: &mut Vec<Entity>) -> MapGeneratorReturnType {
    let map = vec![vec![TileFactory::Floor.create(); height as usize]; width as usize];
    (map, Position::new(seed, width / 2, height / 2, level))
}
