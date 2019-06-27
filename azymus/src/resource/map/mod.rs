use std::fmt;
use crate::map::MapType;

/// Resource.
pub struct MapResource(pub MapType);

/// Allows us to show this object in tests, etc.
impl fmt::Debug for MapResource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Map")
    }
}

#[cfg(test)]
mod tests {

}
