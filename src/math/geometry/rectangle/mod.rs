use super::cell::Cell;

/// The Rectangle structure.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Rectangle {
    /// The x-coordinate (upper-left).
    pub x: usize,
    /// The y-coordinate (upper-left).
    pub y: usize,
    /// The width.
    pub width: usize,
    /// The height.
    pub height: usize,
}

/// The Rectangle structure.
impl Rectangle {

    /// Constructor.
    pub fn new(x: usize, y: usize, w: usize, h: usize) -> Rectangle {
        Rectangle {
            x: x,
            y: y,
            width: w,
            height: h,
        }
    }

    /// x2.
    pub fn x2(&self) -> usize {
        self.x + self.width
    }

    /// y2.
    pub fn y2(&self) -> usize {
        self.y + self.height
    }

    /// Top-left corner as cell.
    pub fn top_left_cell(&self) -> Cell {
        Cell::new(self.x, self.y)
    }

    /// Top-right corner as cell.
    pub fn top_right_cell(&self) -> Cell {
        Cell::new(self.x2(), self.y)
    }

    /// Bottom left corner as cell.
    pub fn bottom_left_cell(&self) -> Cell {
        Cell::new(self.x, self.y2())
    }

    /// Bottom right corner as cell.
    pub fn bottom_right_cell(&self) -> Cell {
        Cell::new(self.x2(), self.y2())
    }

    /// From cells:
    pub fn from_cells(tl: &Cell, br: &Cell) -> Rectangle {
        assert!(br.x > tl.x);
        assert!(br.y > tl.y);
        Rectangle::new(tl.x, tl.y, br.x - tl.x, br.y - tl.y)
    }

    /// Center.
    pub fn center(&self) -> Cell {
        Cell::new(self.x + self.width / 2, self.y + self.height / 2)
    }

    /// Overlaps:
    pub fn overlaps(&self, other: &Rectangle) -> bool {
        (self.x < other.x2())
        && (self.x2() > other.x)
        && (self.y < other.y2())
        && (self.y2() > other.y)
    }

    /// Contains xy.
    pub fn contains_coordinates(&self, x: usize, y: usize) -> bool {
        (self.x <= x)
        && (self.x2() >= x)
        && (self.y <= y)
        && (self.y2() >= y)
    }

    /// Contains cell.
    pub fn contains_cell(&self, cell: &Cell) -> bool {
        self.contains_coordinates(cell.x, cell.y)
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    /// Ensure our constructor makes sense.
    #[test]
    fn new_rectangle() {
        assert_eq!(3, Rectangle::new(3, 4, 4, 8).x);
        assert_eq!(4, Rectangle::new(3, 4, 4, 8).y);
        assert_eq!(Cell::new(3, 4), Rectangle::new(3, 4, 4, 8).top_left_cell());
        assert_eq!(Cell::new(7, 4), Rectangle::new(3, 4, 4, 8).top_right_cell());
        assert_eq!(Cell::new(3, 12), Rectangle::new(3, 4, 4, 8).bottom_left_cell());
        assert_eq!(Cell::new(7, 12), Rectangle::new(3, 4, 4, 8).bottom_right_cell());
    }

    /// Top-left cell.
    #[test]
    fn top_left_cell() {
        assert_eq!(Cell::new(3, 4), Rectangle::new(3, 4, 4, 8).top_left_cell());
    }

    /// Top-right cell.
    #[test]
    fn top_right_cell() {
        assert_eq!(Cell::new(7, 4), Rectangle::new(3, 4, 4, 8).top_right_cell());
    }

    /// Bottom-left cell.
    #[test]
    fn bottom_left_cell() {
        assert_eq!(Cell::new(3, 12), Rectangle::new(3, 4, 4, 8).bottom_left_cell());
    }

    /// Bottom-right cell.
    #[test]
    fn bottom_right_cell() {
        assert_eq!(Cell::new(7, 12), Rectangle::new(3, 4, 4, 8).bottom_right_cell());
    }

    /// From cells.
    #[test]
    fn from_cells() {
        assert_eq!(Rectangle::new(3, 4, 4, 8), Rectangle::from_cells(&Cell::new(3, 4), &Cell::new(7, 12)));
    }

    /// Center.
    #[test]
    fn center() {
        assert_eq!(Cell::new(10, 10), Rectangle::new(6, 6, 8, 8).center());
        assert_eq!(Cell::new(10, 10), Rectangle::new(6, 6, 8, 9).center());
        assert_eq!(Cell::new(10, 11), Rectangle::new(6, 6, 8, 10).center());
        assert_eq!(Cell::new(10, 10), Rectangle::new(6, 6, 9, 9).center());
        assert_eq!(Cell::new(11, 11), Rectangle::new(6, 6, 10, 10).center());
    }

    /// Overlaps.
    #[test]
    fn overlaps() {
        assert!(Rectangle::new(3, 4, 4, 8).overlaps(&Rectangle::new(3, 4, 4, 8)));
        assert!(Rectangle::new(3, 4, 4, 8).overlaps(&Rectangle::new(3, 4, 5, 9)));
        assert!(Rectangle::new(3, 4, 4, 8).overlaps(&Rectangle::new(2, 3, 6, 10)));
        assert!(Rectangle::new(3, 4, 4, 8).overlaps(&Rectangle::new(5, 6, 3, 3)));
        assert!(!Rectangle::new(3, 4, 4, 4).overlaps(&Rectangle::new(13, 14, 4, 4)));
    }

    /// Contains cell.
    #[test]
    fn contains_cell() {
        assert!(Rectangle::new(3, 4, 4, 8).contains_cell(&Cell::new(3, 4)));
        assert!(Rectangle::new(3, 4, 4, 8).contains_cell(&Cell::new(7, 4)));
        assert!(Rectangle::new(3, 4, 4, 8).contains_cell(&Cell::new(3, 12)));
        assert!(Rectangle::new(3, 4, 4, 8).contains_cell(&Cell::new(7, 12)));
        assert!(!Rectangle::new(3, 4, 4, 8).contains_cell(&Cell::new(2, 14)));
        assert!(!Rectangle::new(3, 4, 4, 8).contains_cell(&Cell::new(8, 4)));
        assert!(!Rectangle::new(3, 4, 4, 8).contains_cell(&Cell::new(2, 4)));
        assert!(!Rectangle::new(3, 4, 4, 8).contains_cell(&Cell::new(7, 13)));
        assert!(!Rectangle::new(3, 4, 4, 4).contains_cell(&Cell::new(30, 40)));
    }

}
