use specs::*;
use crate::action::Action;
use crate::component;
use component::occupant::Occupant;
use component::position::Position;

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
pub fn check_map_boundaries(x: i32, y: i32, map_height: i32, map_width: i32) -> bool {
    trace!("Checking if coordinates ({}, {}) are outside map boundaries...", x, y);
    if x <= 0 || y <= 0 {
        trace!("Coordinates ({}, {}) were out of bounds.", x, y);
        return false;
    }
    if x >= map_width - 1 || y >= map_height - 1 {
        trace!("Coordinates ({}, {}) were out of bounds.", x, y);
        return false;
    }
    trace!("Coordinates ({}, {}) were not out of bounds.", x, y);
    return true;
}

/// Checks a specific position in the map to see if there's an Occupant there.
pub fn check_map_occupied<'a>(x: i32, y: i32, occupant_storage: &ReadStorage<'a, Occupant>, position_storage: &ReadStorage<'a, Position>) -> bool {
    trace!("Checking if coordinates ({}, {}) are occupied...", x, y);
    let occupant_count = (occupant_storage, position_storage)
        .join()
        .filter(|(_, position)| position.x == x && position.y == y)
        .count();
    if occupant_count > 0 {
        trace!("Occupant found at ({}, {}).", x, y);
        return false;
    }
    trace!("No occupant found at ({}, {}).", x, y);
    return true;
}
