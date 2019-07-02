/// Indicates a position of the entity within the game world.
#[derive(Clone, Copy, Debug)]
pub struct Position {
    /// The w-coordinate of the object.  (World)
    pub w: i64,
    /// The x-coordinate of the object.  (Horizontal)
    pub x: i32,
    /// The y-coordinate of the object.  (Vertical)
    pub y: i32,
    /// The z-coordinate of the object.  (Altitude)
    pub z: i32,
}

impl Position {

    /// Constructor.
    pub fn new(w: i64, x: i32, y: i32, z: i32) -> Position {
        Position {
            w,
            x,
            y,
            z,
        }
    }

}

/// Creates a default instance.
impl Default for Position {

    /// Creates a default instance.
    fn default() -> Self {
        Position {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }

}
