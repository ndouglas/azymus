use ntree::Region;

/// A Quad-Tree region.
#[derive(Clone, Debug, PartialEq)]
pub struct QuadTreeRegion {
    /// X.
    pub x: i32,
    /// Y.
    pub y: i32,
    /// Width.
    pub width: i32,
    /// Height.
    pub height: i32,
}

/// A point that might be found in the region.
#[derive(Debug, PartialEq, Clone)]
pub struct QuadTreePoint {
    /// Entity ID.
    pub id: usize,
    /// X.
    pub x: i32,
    /// Y.
    pub y: i32,
}

impl Region<QuadTreePoint> for QuadTreeRegion {

    /// Does this region contain this point?
    fn contains(&self, point: &QuadTreePoint) -> bool {
        let result = self.x <= point.x
            && self.y <= point.y
            && (self.x + self.width) >= point.x
            && (self.y + self.height) >= point.y;
        result
    }

    /// Tear my tree into pieces, this is my subdivide()
    fn split(&self) -> Vec<QuadTreeRegion> {
        let halfwidth = self.width / 2;
        let halfheight = self.height / 2;
        vec![
            QuadTreeRegion {
                x: self.x,
                y: self.y,
                width: halfwidth,
                height: halfheight
            },
            QuadTreeRegion {
                x: self.x,
                y: self.y + halfheight,
                width: halfwidth,
                height: self.height - halfheight
            },
            QuadTreeRegion {
                x: self.x + halfwidth,
                y: self.y,
                width: self.width - halfwidth,
                height: halfheight
            },
            QuadTreeRegion {
                x: self.x + halfwidth,
                y: self.y + halfheight,
                width: self.width - halfwidth,
                height: self.height - halfheight
            }
        ]
    }

    fn overlaps(&self, other: &QuadTreeRegion) -> bool {
        self.x < other.x + other.width
        && self.x + self.width > other.x
        && self.y < other.y + other.height
        && self.y + self.height > other.y
    }

}
