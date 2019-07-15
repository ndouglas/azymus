use rand::*;
use rand::prelude::StdRng;

/// The seed type.
pub type SeedType = [u8; 32];

/// The RNG type.
pub type RngType = StdRng;

/// Get the random number generator.
pub fn get_rng(seed: SeedType) -> RngType {
    let rng: StdRng = SeedableRng::from_seed(seed);
    rng
}

/// Get a random seed.
pub fn get_rng_seed() -> SeedType {
    let mut rng = thread_rng();
    rng.gen::<SeedType>()
}
