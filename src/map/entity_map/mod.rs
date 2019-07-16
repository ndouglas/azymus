use std::fmt;
use ntree::NTree;
use std::collections::{HashMap, HashSet};
use crate::entity;
use entity::Entity;
use crate::math;
use math::geometry::cell::{Cell, Cellular};
use math::geometry::rectangle::{Rectangle, Rectangular};

/// Number of entities kept in a given quadtree node.
const QUADTREE_CAPACITY: u8 = 16;

/// Entity quad tree.
pub mod quadtree;
pub use quadtree::QuadTreePoint;
pub use quadtree::QuadTreeType;

/// The entity set type.
pub type EntitySetType = HashSet<usize>;

/// The spatial hash map type.
pub type EntitySpatialHashType = HashMap<Cell, EntitySetType>;

/// The entity map type.
pub struct EntityMap {
    /// The map's width.
    pub width: usize,
    /// The map's height.
    pub height: usize,
    /// The entity spatial hash map.
    pub spatial_hash: EntitySpatialHashType,
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
            spatial_hash: spatial_hash,
            quadtree: quadtree,
        }
    }

    /// Build a quadtree.
    pub fn build_quadtree(&self) -> QuadTreeType {
        let quadtree = NTree::new(Rectangle {
            x: 0,
            y: 0,
            width: self.width,
            height: self.height,
        }, QUADTREE_CAPACITY);
        quadtree
    }

    /// Update the quadtree.
    pub fn update_quadtree(&mut self, vector: Vec<Entity>) {
        let mut quadtree = self.build_quadtree();
        for entity in vector {
            let cell = &entity.as_cell();
            quadtree.insert(QuadTreePoint {
                id: entity.id,
                x: cell.x,
                y: cell.y,
            });
        }
        self.quadtree = quadtree;
    }

    /// Insert an entity identifier into the spatial hash map.
    pub fn insert_entity_id(&mut self, id: usize, cell: &Cell) {
        if let Some(set) = self.spatial_hash.get_mut(cell) {
            set.insert(id);
        }
    }

    /// Remove an entity identifier from the spatial hash map.
    pub fn remove_entity_id(&mut self, id: usize, cell: &Cell) {
        if let Some(set) = self.spatial_hash.get_mut(cell) {
            set.remove(&id);
        }
    }

    /// Moves an entity from one cell to another.
    pub fn move_entity_id(&mut self, id: usize, cell1: &Cell, cell2: &Cell) {
        self.remove_entity_id(id, cell1);
        self.insert_entity_id(id, cell2);
    }

    /// Retrieve the entity identifiers at a specific cell.
    pub fn get_entity_ids_cell(&self, cell: &Cell) -> Option<EntitySetType> {
        if self.as_rectangle().contains_cell(cell) {
            if let Some(set) = self.spatial_hash.get(cell) {
                return Some(set.clone());
            }
        }
        None
    }

    /// Retrieve the entity identifiers at multiple cells.
    pub fn get_entity_ids_cells(&self, cells: &[Cell]) -> EntitySetType {
        let mut result = HashSet::new();
        for cell in cells {
            if let Some(ids) = self.get_entity_ids_cell(cell) {
                result.extend(ids);
            }
        }
        result
    }

    /// Retrieve the entity identifiers in the Von Neumann neighborhood.
    pub fn get_entity_ids_in_von_neumann_neighborhood(&self, cell: &Cell) -> EntitySetType {
        let cells = cell.get_von_neumann_neighborhood(&self.as_rectangle());
        self.get_entity_ids_cells(&cells)
    }

    /// Retrieve the entity identifiers in the Von Neumann neighborhood.
    pub fn get_entity_ids_in_moore_neighborhood(&self, cell: &Cell) -> EntitySetType {
        let cells = cell.get_moore_neighborhood(&self.as_rectangle());
        self.get_entity_ids_cells(&cells)
    }

}

/// Allows us to create a rectangle representation of this map.
impl Rectangular for EntityMap {

    /// Create a rectangle from this object.
    fn as_rectangle(&self) -> Rectangle {
        Rectangle {
            x: 0,
            y: 0,
            width: self.width,
            height: self.height,
        }
    }

}

/// Allows us to show this object in tests, etc.
impl fmt::Debug for EntityMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Entity Map")
    }
}

