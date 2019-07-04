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

/// Indicates a position of the entity within the game world.
impl Position {

    /// Constructor.
    pub fn new(w: i64, x: i32, y: i32, z: i32) -> Position {
        trace!("Entering Position::new().");
        Position {
            w,
            x,
            y,
            z,
        }
    }

    /// Position to the north.
    pub fn to_north(&self) -> Position {
        trace!("Entering Position::to_north().");
        Position {
            w: self.w,
            x: self.x,
            y: self.y - 1,
            z: self.z,
        }
    }

    /// Position to the south.
    pub fn to_south(&self) -> Position {
        trace!("Entering Position::to_south().");
        Position {
            w: self.w,
            x: self.x,
            y: self.y + 1,
            z: self.z,
        }
    }

    /// Position to the west.
    pub fn to_west(&self) -> Position {
        trace!("Entering Position::to_west().");
        Position {
            w: self.w,
            x: self.x - 1,
            y: self.y,
            z: self.z,
        }
    }

    /// Position to the east.
    pub fn to_east(&self) -> Position {
        trace!("Entering Position::to_east().");
        Position {
            w: self.w,
            x: self.x + 1,
            y: self.y,
            z: self.z,
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
