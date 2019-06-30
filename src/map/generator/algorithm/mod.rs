use specs::*;

/// None...
pub mod none;
/// Tutorial...
pub mod simple;
/// Random...
pub mod random;

/// The distinct algorithms for generating maps.
#[derive(Clone, Copy, Debug)]
pub enum Algorithm {
    /// All floor, useful(?) for testing.
    None,
    /// The simple algorithm used by this Rust roguelike tutorial.
    Simple,
    /// This should be a lot cooler when I don't have exactly two choices.
    Random,
}

/// The distinct algorithms for generating maps.
impl Algorithm {

    /// Generate the map.
    pub fn generate_map(&self, world: &mut World, width: i32, height: i32, seed: i64) -> (i32, i32) {
        use Algorithm::*;
        let generate_map: fn(&mut World, i32, i32, i64) -> (i32, i32) = match self {
            None => none::generate_map,
            Simple => simple::generate_map,
            Random => random::generate_map,
        };
        generate_map(world, width, height, seed)
    }

}
