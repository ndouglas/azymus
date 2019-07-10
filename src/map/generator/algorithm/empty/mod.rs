use crate::component;
use component::position::Position;
use component::renderable::Factory as RenderableFactory;
use crate::entity;
use entity::Entity;
use crate::seed;
use seed::SeedType;
use seed::RngType;
use crate::tile;
use tile::Tile;
use super::super::MapGeneratorReturnType;

/// Generate the map.
pub fn generate_map(seed: SeedType, _rng: &mut RngType, width: i32, height: i32, level: i32, _objects: &mut Vec<Entity>) -> MapGeneratorReturnType {
    let mut map = vec![vec![Tile::new(); height as usize]; width as usize];
    for y in 0..height {
        for x in 0..width {
            let mut tile = &mut map[x as usize][y as usize];
            tile.position = Some(Position::new(seed, x, y, level));
            tile.renderable = Some(RenderableFactory::Floor.create());
        }
    }
    (map, Position::new(seed, width / 2, height / 2, level))
}
