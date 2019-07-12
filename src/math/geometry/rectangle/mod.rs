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

    /// Top-left corner as cell.
    pub fn top_left_cell(&self) -> Cell {
        Cell::new(self.x, self.y)
    }

    /// Top-right corner as cell.
    pub fn top_right_cell(&self) -> Cell {
        Cell::new(self.x + self.width, self.y)
    }

    /// Bottom left corner as cell.
    pub fn bottom_left_cell(&self) -> Cell {
        Cell::new(self.x, self.y + self.height)
    }

    /// Bottom right corner as cell.
    pub fn bottom_right_cell(&self) -> Cell {
        Cell::new(self.x + self.width, self.y + self.height)
    }

    /// From poinds:
    pub fn from_points(tl: &Cell, br: &Cell) -> Rectangle {
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
        (self.x < other.x + other.width)
        && (self.x + self.width > other.x)
        && (self.y < other.y + other.height)
        && (self.y + self.height > other.y)
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

    /// From points.
    #[test]
    fn from_points() {
        assert_eq!(Rectangle::new(3, 4, 4, 8), Rectangle::from_points(&Cell::new(3, 4), &Cell::new(7, 12)));
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

}
