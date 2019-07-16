/// The entity list.
pub mod entity_list;
use entity_list::EntityList;

/// The game world.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct World {
    /// The entity list.
    pub entity_list: EntityList,

}

impl World {

    /// Constructor.
    pub fn new() -> Self {
        World {
            entity_list: EntityList::new(),
        }
    }

}
