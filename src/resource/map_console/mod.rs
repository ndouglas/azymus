use std::fmt;
use tcod::console::Offscreen;
use std::sync::Arc;
use std::sync::Mutex;

/// Resource.
pub struct MapConsoleResource(pub Arc<Mutex<Offscreen>>);

/// Allows us to show this object in tests, etc.
impl fmt::Debug for MapConsoleResource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Map Console")
    }
}

#[cfg(test)]
mod tests {

}
