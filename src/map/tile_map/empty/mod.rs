use crate::seed;
use seed::SeedType;
use crate::tile;
use tile::Factory as TileFactory;
use super::TileMapType;

/// Generate the tile map.
pub fn generate_tile_map(_seed: SeedType, width: usize, height: usize) -> TileMapType {
    let map = vec![vec![TileFactory::Floor.create(); height]; width];
    map
}
