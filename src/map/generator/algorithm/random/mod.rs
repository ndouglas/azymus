use super::super::MapGeneratorReturnType;

/// Generate the map.
pub fn generate_map(seed: i64, width: i32, height: i32, level: i32) -> MapGeneratorReturnType {
    super::simple::generate_map(seed, width, height, level) // Chosen at random.
}
