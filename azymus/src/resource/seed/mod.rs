use std::fmt;
use rand::Rng;
use rand::thread_rng;

/// Resource.
#[derive(Copy, Clone)]
pub struct SeedResource(pub i64);

impl Default for SeedResource {
    fn default() -> Self {
        SeedResource(thread_rng().gen::<i64>())
    }
}

/// Allows us to show this object in tests, etc.
impl fmt::Debug for SeedResource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SeedResource {:?}", self.0)
    }
}

#[cfg(test)]
mod tests {

}
