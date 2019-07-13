use crate::seed;
use seed::SeedType;
use super::TileMapType;
use super::simple;

/// Generate the tile map.
pub fn generate_tile_map(seed: SeedType, width: usize, height: usize) -> TileMapType {
    simple::generate_tile_map(seed, width, height) // Chosen at random.
}
