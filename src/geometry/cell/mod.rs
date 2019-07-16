use std::fmt;
use super::compass::Direction;

/// A cell offset.
pub type CellOffsetType = (i64, i64);

/// The Cell structure.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Cell {
    /// The x-coordinate.
    pub x: usize,
    /// The y-coordinate.
    pub y: usize,
}

/// A trait that permits deriving a cell from anything with x and y fields.
pub trait Cellular {

    /// Create a cell from this object.
    fn as_cell(&self) -> Cell;

}

/// The Cell structure.
impl Cell {

    /// Constructor.
    pub fn new(x: usize, y: usize) -> Cell {
        Cell {
            x,
            y,
        }
    }

    /// To a specified offset.
    pub fn get_offset_as_cell(&self, offset: CellOffsetType) -> Cell {
        let (dx, dy) = offset;
        let final_x = (self.x as i64 + dx) as usize;
        let final_y = (self.y as i64 + dy) as usize;
        Cell::new(final_x, final_y)
    }

    /// Convert to a tuple.
    pub fn as_tuple(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    /// Gets the offset to another cell.
    pub fn offset_to(&self, cell: &Cell) -> CellOffsetType {
        ((cell.x as i64 - self.x as i64), (cell.y as i64 - self.y as i64))
    }

    /// Returns the distance to another object.
    pub fn distance_to_cell(&self, cell: &Cell) -> f32 {
        (((cell.x - self.x).pow(2) + (cell.y - self.y).pow(2)) as f32).sqrt()
    }

    /// Returns the distance to another object.
    pub fn distance_to_offset(&self, offset: CellOffsetType) -> f32 {
        self.distance_to_cell(&self.get_offset_as_cell(offset))
    }

    /// To a specified direction.
    pub fn to_direction(&self, direction: &Direction) -> Cell {
        self.get_offset_as_cell(direction.as_offset())
    }

    /// Direction to a specific cell.
    pub fn direction_to(&self, cell: &Cell) -> Direction {
        Direction::from_offset(self.offset_to(cell))
    }

    /// In the direction of a specific cell.
    pub fn toward_cell(&self, cell: &Cell) -> Cell {
        self.to_direction(&self.direction_to(cell))
    }

    /// To northwest.
    pub fn to_northwest(&self) -> Cell {
        self.to_direction(&Direction::Northwest)
    }

    /// To north.
    pub fn to_north(&self) -> Cell {
        self.to_direction(&Direction::North)
    }

    /// To northeast.
    pub fn to_northeast(&self) -> Cell {
        self.to_direction(&Direction::Northeast)
    }

    /// To southwest.
    pub fn to_southwest(&self) -> Cell {
        self.to_direction(&Direction::Southwest)
    }

    /// To south.
    pub fn to_south(&self) -> Cell {
        self.to_direction(&Direction::South)
    }

    /// To southeast.
    pub fn to_southeast(&self) -> Cell {
        self.to_direction(&Direction::Southeast)
    }

    /// To west.
    pub fn to_west(&self) -> Cell {
        self.to_direction(&Direction::West)
    }

    /// To east.
    pub fn to_east(&self) -> Cell {
        self.to_direction(&Direction::East)
    }

    /// Get Von Neumann Neighborhood.
    pub fn von_neumann_neighborhood(&self) -> Vec<Cell> {
        use Direction::*;
        let mut result: Vec<Cell> = vec![];
        for compass_direction in &[
                North,
                South,
                East,
                West,
            ] {
            result.push(self.to_direction(compass_direction));
        }
        result
    }

    /// Get Moore Neighborhood.
    pub fn moore_neighborhood(&self) -> Vec<Cell> {
        use Direction::*;
        let mut result = self.von_neumann_neighborhood();
        for compass_direction in &[
                Northeast,
                Southeast,
                Southwest,
                Northwest,
            ] {
            result.push(self.to_direction(compass_direction));
        }
        result
    }


}

/// Creates a default instance.
impl Default for Cell {

    /// Creates a default instance.
    fn default() -> Self {
        Cell {
            x: std::usize::MAX,
            y: std::usize::MAX,
        }
    }

}

/// Format this object for user display.
impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    /// Ensure our constructor makes sense.
    #[test]
    fn new_cell() {
        assert_eq!(3, Cell::new(3, 4).x);
        assert_eq!(4, Cell::new(3, 4).y);
        assert_eq!((3, 4), Cell::new(3, 4).as_tuple());
    }

    /// Ensure the directions work.
    #[test]
    fn to_offset() {
        assert_eq!(Cell::new(3, 4), Cell::new(3, 4).to_northeast().to_southwest());
        assert_eq!(Cell::new(3, 4), Cell::new(3, 4).to_southeast().to_west().to_north());
    }

}
