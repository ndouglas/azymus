use crate::command;
use command::CompassDirection;

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

    /// Position to the northeast.
    pub fn to_northeast(&self) -> Position {
        trace!("Entering Position::to_northeast().");
        self.to_north().to_east()
    }

    /// Position to the northeast.
    pub fn to_northwest(&self) -> Position {
        trace!("Entering Position::to_northwest().");
        self.to_north().to_west()
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

    /// Position to the southeast.
    pub fn to_southeast(&self) -> Position {
        trace!("Entering Position::to_southeast().");
        self.to_south().to_east()
    }

    /// Position to the southwest.
    pub fn to_southwest(&self) -> Position {
        trace!("Entering Position::to_southwest().");
        self.to_south().to_west()
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

    /// Position to a compass direction.
    pub fn to_direction(&self, compass_direction: CompassDirection) -> Position {
        trace!("Entering Position::to_compass_direction().");
        use CompassDirection::*;
        match compass_direction {
            North => self.to_north(),
            Northeast => self.to_northeast(),
            Northwest => self.to_northwest(),
            South => self.to_south(),
            Southeast => self.to_southeast(),
            Southwest => self.to_southwest(),
            East => self.to_east(),
            West => self.to_west(),
        }
    }

    /// Returns the direction to a given position.
    pub fn direction_to(&self, position: &Position) -> Option<CompassDirection> {
        trace!("Entering Position::direction_to().");
        let x_diff = position.x - self.x;
        let y_diff = position.y - self.y;
        match (x_diff, y_diff) {
            (xd, yd) if xd > 0 && yd > 0 => Some(CompassDirection::South),
            (xd, yd) if xd > 0 && yd == 0 => Some(CompassDirection::East),
            (xd, yd) if xd > 0 && yd < 0 => Some(CompassDirection::North),
            (xd, yd) if xd == 0 && yd > 0 => Some(CompassDirection::South),
            (xd, yd) if xd == 0 && yd == 0 => None,
            (xd, yd) if xd == 0 && yd < 0 => Some(CompassDirection::North),
            (xd, yd) if xd < 0 && yd > 0 => Some(CompassDirection::South),
            (xd, yd) if xd < 0 && yd == 0 => Some(CompassDirection::West),
            (xd, yd) if xd < 0 && yd < 0 => Some(CompassDirection::North),
            _ => None,
        }
    }

    /// Returns the distance to another object.
    pub fn distance_to(&self, position: &Position) -> f32 {
        (((position.x - self.x).pow(2) + (position.y - self.y).pow(2)) as f32).sqrt()
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
