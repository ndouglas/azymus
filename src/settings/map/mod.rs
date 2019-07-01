/// Default screen width in characters.
const DEFAULT_WIDTH: i32 = 160;
/// Default creen height in characters.
const DEFAULT_HEIGHT: i32 = 100;

/// Map-specific settings.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Map {
    /// The map width.
    pub width: i32,
    /// The map height.
    pub height: i32,
}

/// Map-specific settings.
impl Map {

    /// Constructor.
    pub fn new() -> Self {
        Map {
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
        }
    }

}

impl Default for Map {
    fn default() -> Self {
        Map::new()
    }
}

#[cfg(test)]
mod tests {

}
