use specs::*;
use crate::action;
use action::Action;
use crate::component;
use component::name::Name;
use component::position::Position;
use crate::resource;
use resource::seed::SeedResource;

/// Processes walk actions.
#[derive(Clone, Copy, Debug)]
pub struct WalkSystem;

/// Processes walk actions.
impl<'a> System<'a> for WalkSystem {

    type SystemData = (
        Entities<'a>,
        Read<'a, SeedResource>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Action>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        trace!("Entering WalkSystem::run().");
        let (
            entities,
            seed_resource,
            name_storage,
            mut action_storage,
            mut position_storage,
        ) = data;
        let _seed = seed_resource.0;
        let mut acted_entities = vec![];
        for (entity, name, action, position) in (&entities, &name_storage, &mut action_storage, &mut position_storage).join() {
            if let Action::Walk((_x1, _y1), (x2, y2)) = action {
                debug!("Walking entity {} to position ({}, {}).", name.name, x2, y2);
                position.x = *x2;
                position.y = *y2;
                acted_entities.push(entity);
            }
        }
        for entity in acted_entities {
            if let Some(_) = action_storage.remove(entity) {
                if let Some(name) = name_storage.get(entity) {
                    debug!("Removed walk action component from entity {}!", name.name);
                }
            }
        }
        trace!("Exiting WalkSystem::run().");
    }
}


/*
/// Checks the map boundaries to make sure we aren't moving something off the map.
pub fn check_map_boundaries(action: Action, x: i32, y: i32, map_console: &Offscreen) -> Option<Action> {
    trace!("Checking if coordinates ({}, {}) are outside map boundaries...", x, y);
    if x <= 0 || y <= 0 {
        trace!("Coordinates ({}, {}) were out of bounds.", x, y);
        return None;
    }
    if x >= map_console.width() - 1 || y >= map_console.height() - 1 {
        trace!("Coordinates ({}, {}) were out of bounds.", x, y);
        return None;
    }
    trace!("Coordinates ({}, {}) were not out of bounds.", x, y);
    Some(action)
}

/// Checks a specific position in the map to see if there's an Occupant there.
pub fn check_map_occupied(action: Action, x: i32, y: i32, world: &World) -> Option<Action> {
    trace!("Checking if coordinates ({}, {}) are occupied...", x, y);
    let occupant_storage = world.read_storage::<Occupant>();
    let position_storage = world.read_storage::<Position>();
    let occupant_count = (&occupant_storage, &position_storage)
        .join()
        .filter(|(_, position)| position.x == x && position.y == y)
        .count();
    if occupant_count > 0 {
        trace!("Occupant found at ({}, {}).", x, y);
        return None;
    }
    trace!("No occupant found at ({}, {}).", x, y);
    return Some(action);
}
*/
