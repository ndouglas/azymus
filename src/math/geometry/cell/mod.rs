use std::fmt;
use super::compass::Direction;
use super::rectangle::Rectangle;

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

    /// Convert to a tuple.
    pub fn as_tuple(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    /// Gets the offset to another cell.
    pub fn offset_to(&self, cell: &Cell) -> CellOffsetType {
        ((cell.x as i64 - self.x as i64), (cell.y as i64 - self.y as i64))
    }

    /// To a specified offset.
    pub fn to_offset(&self, offset: CellOffsetType, rectangle: &Rectangle) -> Option<Cell> {
        let (dx, dy) = offset;
        let final_x = (self.x as i64 + dx) as usize;
        let final_y = (self.y as i64 + dy) as usize;
        if rectangle.contains_coordinates(final_x, final_y) {
            Some(Cell::new(final_x, final_y))
        } else {
            None
        }
    }

    /// To a specified direction.
    pub fn to_compass_direction(&self, compass_direction: &Direction, rectangle: &Rectangle) -> Option<Cell> {
        self.to_offset(compass_direction.as_offset(), rectangle)
    }

    /// In the direction of a specific cell.
    pub fn toward_cell(&self, cell: &Cell, rectangle: &Rectangle) -> Option<Cell> {
        if let Some(compass_direction) = Direction::from_offset(self.offset_to(cell)) {
            self.to_compass_direction(&compass_direction, rectangle)
        } else {
            None
        }
    }

    /// To northwest.
    pub fn to_northwest(&self, rectangle: &Rectangle) -> Option<Cell> {
        self.to_compass_direction(&Direction::Northwest, rectangle)
    }

    /// To north.
    pub fn to_north(&self, rectangle: &Rectangle) -> Option<Cell> {
        self.to_compass_direction(&Direction::North, rectangle)
    }

    /// To northeast.
    pub fn to_northeast(&self, rectangle: &Rectangle) -> Option<Cell> {
        self.to_compass_direction(&Direction::Northeast, rectangle)
    }

    /// To southwest.
    pub fn to_southwest(&self, rectangle: &Rectangle) -> Option<Cell> {
        self.to_compass_direction(&Direction::Southwest, rectangle)
    }

    /// To south.
    pub fn to_south(&self, rectangle: &Rectangle) -> Option<Cell> {
        self.to_compass_direction(&Direction::South, rectangle)
    }

    /// To southeast.
    pub fn to_southeast(&self, rectangle: &Rectangle) -> Option<Cell> {
        self.to_compass_direction(&Direction::Southeast, rectangle)
    }

    /// To west.
    pub fn to_west(&self, rectangle: &Rectangle) -> Option<Cell> {
        self.to_compass_direction(&Direction::West, rectangle)
    }

    /// To east.
    pub fn to_east(&self, rectangle: &Rectangle) -> Option<Cell> {
        self.to_compass_direction(&Direction::East, rectangle)
    }

    /// Is in bounds.
    pub fn is_contained_by_rectangle(&self, rectangle: &Rectangle) -> bool {
        rectangle.contains_cell(self)
    }

    /// Get Von Neumann Neighborhood.
    pub fn get_von_neumann_neighborhood(&self, rectangle: &Rectangle) -> Vec<Cell> {
        use Direction::*;
        let mut result: Vec<Cell> = vec![];
        for compass_direction in &[
                North,
                South,
                East,
                West,
            ] {
            if let Some(neighbor) = self.to_compass_direction(compass_direction, rectangle) {
                result.push(neighbor);
            }
        }
        result
    }

    /// Get Moore Neighborhood.
    pub fn get_moore_neighborhood(&self, rectangle: &Rectangle) -> Vec<Cell> {
        use Direction::*;
        let mut result = self.get_von_neumann_neighborhood(rectangle);
        for compass_direction in &[
                Northeast,
                Southeast,
                Southwest,
                Northwest,
            ] {
            if let Some(neighbor) = self.to_compass_direction(compass_direction, rectangle) {
                result.push(neighbor);
            }
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
        let rectangle = Rectangle::new(0, 0, 10, 10);
        assert_eq!(Cell::new(3, 4), Cell::new(3, 4).to_northeast(&rectangle).unwrap().to_southwest(&rectangle).unwrap());
        assert_eq!(Cell::new(3, 4), Cell::new(3, 4).to_southeast(&rectangle).unwrap().to_west(&rectangle).unwrap().to_north(&rectangle).unwrap());
    }

}
