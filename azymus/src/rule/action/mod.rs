use specs::*;
use tcod::console::*;
use crate::action::Action;
use crate::component;
use component::occupant::Occupant;
use component::position::Position;
use crate::resource;
use resource::map::MapResource;

/// Returns none.  Like, all the time.
pub fn none_callback() -> Option<Action> {
    trace!("Returning none...");
    None
}

/// Returns the specified action.  Like, all the time.
pub fn identity_callback(action: Action) -> Option<Action> {
    trace!("Returning the identity callback for {:?}...", action);
    Some(action)
}

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
    let map_resource = & world.read_resource::<MapResource>().0;
    for tile in &map_resource[x as usize][y as usize] {
        if tile.1 {
            trace!("Occupant found at ({}, {}).", x, y);
            return None;
        }
    }
    return Some(action);
    /*
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
    */
}
