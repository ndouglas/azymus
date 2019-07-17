use rand::Rng;
use rand::distributions::{Distribution, Standard};
use std::f64::consts::PI;
use crate::geometry;
use geometry::cell::CellOffsetType;

/// Compass directions.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Direction {
    /// North.
    North,
    /// Northeast.
    Northeast,
    /// East.
    East,
    /// Southeast.
    Southeast,
    /// South.
    South,
    /// Southwest.
    Southwest,
    /// West.
    West,
    /// Northwest.
    Northwest,
}

impl Direction {

    /// From angle.
    pub fn from_angle(angle: f64) -> Self {
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
        directions[index]
    }

    /// To angle.
    pub fn as_angle(&self) -> i32 {
        use Direction::*;
        match self {
            North => 0,
            Northeast => 45,
            East => 90,
            Southeast => 135,
            South => 180,
            Southwest => 225,
            West => 270,
            Northwest => 315,
        }
    }

    /// Compass direction corresponding to offset.
    pub fn from_offset(offset: CellOffsetType) -> Self {
        use Direction::*;
        match offset {
            (-1, -1) => Northwest,
            (0, -1) => North,
            (1, -1) => Northeast,
            (-1, 1) => Southwest,
            (0, 1) => South,
            (1, 1) => Southeast,
            (-1, 0) => West,
            (1, 0) => East,
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
            North => (0, -1),
            Northeast => (1, -1),
            East => (1, 0),
            Southeast => (1, 1),
            South => (0, 1),
            Southwest => (-1, 1),
            West => (-1, 0),
            Northwest => (-1, -1),
        }
    }

}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0, 8) {
            0 => Direction::North,
            1 => Direction::Northeast,
            2 => Direction::East,
            3 => Direction::Southeast,
            4 => Direction::South,
            5 => Direction::Southwest,
            6 => Direction::West,
            _ => Direction::Northwest,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    /// Ensure our constructor makes sense.
    #[test]
    fn as_offset() {
        assert_eq!(Direction::North, Direction::from_offset(Direction::North.as_offset()));
        assert_eq!(Direction::Northeast, Direction::from_offset(Direction::Northeast.as_offset()));
        assert_eq!(Direction::East, Direction::from_offset(Direction::East.as_offset()));
        assert_eq!(Direction::Southeast, Direction::from_offset(Direction::Southeast.as_offset()));
        assert_eq!(Direction::South, Direction::from_offset(Direction::South.as_offset()));
        assert_eq!(Direction::Southwest, Direction::from_offset(Direction::Southwest.as_offset()));
        assert_eq!(Direction::West, Direction::from_offset(Direction::West.as_offset()));
        assert_eq!(Direction::Northwest, Direction::from_offset(Direction::Northwest.as_offset()));
        assert_eq!(Direction::North, Direction::from_offset((0,-2)));
        assert_eq!(Direction::Northeast, Direction::from_offset((2,-2)));
        assert_eq!(Direction::East, Direction::from_offset((2,0)));
        assert_eq!(Direction::Southeast, Direction::from_offset((2,2)));
        assert_eq!(Direction::South, Direction::from_offset((0,2)));
        assert_eq!(Direction::Southwest, Direction::from_offset((-2,2)));
        assert_eq!(Direction::West, Direction::from_offset((-2,0)));
        assert_eq!(Direction::Northwest, Direction::from_offset((-2,-2)));
    }

    /// Ensure our constructor makes sense.
    #[test]
    fn from_offset() {
        assert_eq!(Direction::North, Direction::from_offset((0,-2)));
        assert_eq!(Direction::Northeast, Direction::from_offset((2,-2)));
        assert_eq!(Direction::East, Direction::from_offset((2,0)));
        assert_eq!(Direction::Southeast, Direction::from_offset((2,2)));
        assert_eq!(Direction::South, Direction::from_offset((0,2)));
        assert_eq!(Direction::Southwest, Direction::from_offset((-2,2)));
        assert_eq!(Direction::West, Direction::from_offset((-2,0)));
        assert_eq!(Direction::Northwest, Direction::from_offset((-2,-2)));
    }

}
