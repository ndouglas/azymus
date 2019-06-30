use crate::action::Action;

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
    pub fn callback(self) -> Option<Action> {
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
            _ => { return none_callback(); }
        };
    }

}
