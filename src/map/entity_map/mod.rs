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

    /// Insert an entity into the vector.
    pub fn insert_entity(&mut self, mut entity: Entity) {
        entity.id = self.vector.len();
        self.insert_entity_id(entity.id, &entity.cell);
        self.vector.push(entity);
    }

    /// Remove an entity from the vector.
    pub fn remove_entity(&mut self, entity: &Entity) {
        let entity_id = entity.id;
        self.vector.swap_remove(entity_id);
        let mut moved_entity = &mut self.vector[entity_id];
        moved_entity.id = entity_id;
    }

    /// Get entities for identifiers.
    pub fn get_entities(&self, ids: &[usize]) -> Vec<&Entity> {
        let mut result = Vec::new();
        for id in ids {
            result.push(&self.vector[*id]);
        }
        result
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

