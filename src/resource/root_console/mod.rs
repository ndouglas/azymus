use std::fmt;
use tcod::console::Root;
use std::sync::Arc;
use std::sync::Mutex;

/// Resource.
pub struct RootConsoleResource(pub Arc<Mutex<Root>>);

/// Allows us to show this object in tests, etc.
impl fmt::Debug for RootConsoleResource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Root Console")
    }
}

#[cfg(test)]
mod tests {

}
