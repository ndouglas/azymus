use crate::seed;
use seed::SeedType;
use crate::tile;
use tile::Tile;
use super::super::TileMapType;

/// Generate the tile map.
pub fn generate_tile_map(seed: SeedType, width: usize, height: usize) -> TileMapType {
    let map = vec![vec![Tile::floor(); height as usize]; width as usize];
    (map, Position::new(seed, width / 2, height / 2, level))
}
