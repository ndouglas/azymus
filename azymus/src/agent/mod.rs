use specs::*;
use crate::action::Action;

/// Orc behavior.
pub mod orc;

/// The commands.
#[derive(Clone, Copy, Debug)]
pub enum Agent {
    /// Classic orc.
    Orc,
}

impl Agent {

    /// Get action.
    ///
    /// Returns the action that the entity would like to perform next.
    pub fn get_action(self, _seed: i64, _energy: i32, _entity: Entity, _world: &World) -> Option<Action> {
        use Agent::*;
        match self {
            Orc                  => {
                println!("rawr");
                None
            },
        }
    }

}
