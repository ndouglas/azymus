use std::f64::consts::PI;
use super::cell::CellOffsetType;

/// Compass directions.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Direction {
    /// North.
    North,
    /// Northeast.
    Northeast,
    /// Northwest.
    Northwest,
    /// South.
    South,
    /// Southeast.
    Southeast,
    /// Southwest.
    Southwest,
    /// East.
    East,
    /// West.
    West,
}

impl Direction {

    /// From angle.
    pub fn from_angle(angle: f64) -> Option<Self> {
        use Direction::*;
        let index = ((angle + 450.0) % 360.0 / 45.0 ) as usize;
        let directions = [
            North,
            Northeast,
            East,
            Southeast,
            South,
            Southwest,
            West,
            Northwest,
        ];
        return Some(directions[index]);
    }

    /// Compass direction corresponding to offset.
    pub fn from_offset(offset: CellOffsetType) -> Option<Self> {
        use Direction::*;
        match offset {
            (-1, -1) => Some(Northwest),
            (0, -1) => Some(North),
            (1, -1) => Some(Northeast),
            (-1, 1) => Some(Southwest),
            (0, 1) => Some(South),
            (1, 1) => Some(Southeast),
            (-1, 0) => Some(West),
            (1, 0) => Some(East),
            (dx, dy) => {
                let angle = (dy as f64).atan2(dx as f64) / PI * 180 as f64;
                Direction::from_angle(angle)
            },
        }
    }

    /// Offsets corresponding to compass directions.
    pub fn as_offset(&self) -> CellOffsetType {
        use Direction::*;
        match self {
            Northwest => (-1, -1),
            North => (0, -1),
            Northeast => (1, -1),
            Southwest => (-1, 1),
            South => (0, 1),
            Southeast => (1, 1),
            West => (-1, 0),
            East => (0, -1),
        }
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    /// Ensure our constructor makes sense.
    #[test]
    fn from_offset() {
        assert_eq!(Direction::North, Direction::from_offset((0,-2)).unwrap());
        assert_eq!(Direction::Northeast, Direction::from_offset((2,-2)).unwrap());
        assert_eq!(Direction::East, Direction::from_offset((2,0)).unwrap());
        assert_eq!(Direction::Southeast, Direction::from_offset((2,2)).unwrap());
        assert_eq!(Direction::South, Direction::from_offset((0,2)).unwrap());
        assert_eq!(Direction::Southwest, Direction::from_offset((-2,2)).unwrap());
        assert_eq!(Direction::West, Direction::from_offset((-2,0)).unwrap());
        assert_eq!(Direction::Northwest, Direction::from_offset((-2,-2)).unwrap());
    }

}
