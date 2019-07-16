use crate::entity;
use entity::Entity;

/// The vector of entities.
pub type EntityVectorType = Vec<Entity>;

/// The entity list.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct EntityList {
    /// The entity vector.
    pub vector: EntityVectorType,
}

impl EntityList {

    /// Constructor.
    pub fn new() -> Self {
        EntityList {
            vector: Vec::new(),
        }
    }

    /// Insert an entity into the vector.
    pub fn insert_entity(&mut self, mut entity: Entity) -> usize {
        let id = self.vector.len();
        entity.id = id;
        self.vector.push(entity);
        id
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

}
