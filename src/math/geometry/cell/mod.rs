/// The Cell structure.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Cell {
    /// The x-coordinate.
    pub x: usize,
    /// The y-coordinate.
    pub y: usize,
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

    /// To tuple.
    pub fn to_tuple(&self) -> (usize, usize) {
        (self.x, self.y)
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
        assert_eq!((3, 4), Cell::new(3, 4).to_tuple());
    }

}
