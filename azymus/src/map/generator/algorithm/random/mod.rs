use specs::*;

/// Generate the map.
pub fn generate_map(world: &mut World, width: i32, height: i32, seed: i64) -> (i32, i32) {
    super::simple::generate_map(world, width, height, seed) // chosen at random.
}
