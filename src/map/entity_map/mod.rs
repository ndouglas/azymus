use std::fmt;
use ntree::NTree;
use std::collections::{HashMap, HashSet};
use crate::entity;
use entity::Entity;
use crate::math;
use math::geometry::cell::{Cell, Cellular};
use math::geometry::rectangle::Rectangle;

/// Number of entities kept in a given quadtree node.
const QUADTREE_CAPACITY: u8 = 16;

/// Entity quad tree.
pub mod quadtree;
use quadtree::QuadTreePoint;
use quadtree::QuadTreeType;

/// The vector of entities.
pub type EntityVectorType = Vec<Entity>;

/// The entity set type.
pub type EntitySetType = HashSet<usize>;

/// The spatial hash map type.
pub type EntityVectorSpatialHashType = HashMap<Cell, EntitySetType>;

/// The entity map type.
pub struct EntityMap {
    /// The map's width.
    pub width: usize,
    /// The map's height.
    pub height: usize,
    /// The entity vector.
    pub vector: EntityVectorType,
    /// The entity spatial hash map.
    pub spatial_hash: EntityVectorSpatialHashType,
    /// The entity quadtree.
    pub quadtree: QuadTreeType,
}

/// The entity map type.
impl EntityMap {

    /// Constructor.
    pub fn new(width: usize, height: usize) -> Self {
        let mut spatial_hash = HashMap::new();
        for y in 0..height {
            for x in 0..width {
                spatial_hash.insert(Cell::new(x, y), HashSet::new());
            }
        }
        let quadtree = NTree::new(Rectangle {
            x: 0,
            y: 0,
            width: width,
            height: height,
        }, QUADTREE_CAPACITY);
        EntityMap {
            width: width,
            height: height,
            vector: Vec::new(),
            spatial_hash: spatial_hash,
            quadtree: quadtree,
        }
    }

    /// Update the quadtree.
    pub fn update_quadtree(&mut self) {
        let mut quadtree = NTree::new(Rectangle {
            x: 0,
            y: 0,
            width: self.width,
            height: self.height,
        }, QUADTREE_CAPACITY);
        for entity in &self.vector {
            let cell = &entity.as_cell();
            quadtree.insert(QuadTreePoint {
                id: entity.id,
                x: cell.x,
                y: cell.y,
            });
        }
        self.quadtree = quadtree;
    }

}

/// Allows us to show this object in tests, etc.
impl fmt::Debug for EntityMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Entity Map")
    }
}

