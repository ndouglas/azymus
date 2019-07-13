use super::cell::CellOffsetType;

/// Compass directions.
#[derive(Clone, Copy, Debug)]
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

    /// Compass direction corresponding to offset.
    pub fn from_offset(&self, offset: CellOffsetType) -> Option<Self> {
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
            _ => None,
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
