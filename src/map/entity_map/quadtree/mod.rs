use ntree::NTree;
use crate::math;
use math::geometry::cell::{Cell, Cellular};
use math::geometry::rectangle::Rectangle;

/// A point that might be found in a quadtree region.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct QuadTreePoint {
    /// Entity ID.
    pub id: usize,
    /// X.
    pub x: usize,
    /// Y.
    pub y: usize,
}

/// Implement cellular for this point.
impl Cellular for QuadTreePoint {

    /// Create a cell from this object.
    fn as_cell(&self) -> Cell {
        Cell {
            x: self.x,
            y: self.y,
        }
    }

}

/// The standard quadtree type.
pub type QuadTreeType = NTree<Rectangle, QuadTreePoint>;
