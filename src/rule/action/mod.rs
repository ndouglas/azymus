use crate::action::Action;

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
pub fn is_map_point_occupied<'a>(x: i32, y: i32, occupant_map: &Vec<Vec<bool>>) -> bool {
    trace!("Checking if coordinates ({}, {}) are occupied...", x, y);
    if occupant_map[x as usize][y as usize] {
        trace!("Occupant found at ({}, {}).", x, y);
        return true;
    }
    trace!("No occupant found at ({}, {}).", x, y);
    return false;
}

/// Checks a specific position in the map to see if there's an Occupant there.
pub fn is_map_point_combatant<'a>(x: i32, y: i32, combatant_map: &Vec<Vec<bool>>) -> bool {
    trace!("Checking if coordinates ({}, {}) are occupied by a combatant...", x, y);
    if combatant_map[x as usize][y as usize] {
        trace!("Combatant found at ({}, {}).", x, y);
        return true;
    }
    trace!("No Combatant found at ({}, {}).", x, y);
    return false;
}
