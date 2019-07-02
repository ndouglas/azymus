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
