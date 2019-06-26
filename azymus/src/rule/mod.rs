use specs::*;
use crate::action::Action;
use crate::resource;
use resource::map_console::MapConsoleResource;

/// Action Rule callbacks.
pub mod action;

/// Action Rule type.
#[derive(Clone, Copy, Debug)]
pub enum ActionRule {
    /// Return None.
    None,
    /// Return the specified action.
    Return(Action),
    /// Check the coordinates against the boundaries of the map.
    CheckMapBoundaries(i32, i32),
    /// Check whether the coordinates are occupied by something.
    CheckMapOccupied(i32, i32),
}

impl ActionRule {

    /// Execute the rule.
    pub fn callback(self, action: Action, _entity: Entity, world: &World) -> Option<Action> {
        use ActionRule::*;
        use action::*;
        match self {
            None => {
                trace!("Executing the none rule.");
                return none_callback();
            },
            Return(action) => {
                trace!("Executing the identity rule.");
                return identity_callback(action);
            },
            CheckMapBoundaries(x, y) => {
                trace!("Executing the check-map-boundaries rule.");
                let map_console_resource = & world.read_resource::<MapConsoleResource>().0;
                let map_console = map_console_resource.lock().unwrap();
                return check_map_boundaries(action, x, y, &map_console);
            },
            CheckMapOccupied(x, y) => {
                trace!("Executing the check-map-occupied rule.");
                return check_map_occupied(action, x, y, world);
            },
        };
    }

}
