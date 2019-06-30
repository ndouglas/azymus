use specs::*;
use crate::resource;
use resource::turn_flag::TurnFlagResource;

/// Receives and acts on player input.
#[derive(Clone, Copy, Debug)]
pub struct TurnFlagResetterSystem;

/// Receives and acts on player input.
impl<'a> System<'a> for TurnFlagResetterSystem {

    type SystemData = (
        Write<'a, TurnFlagResource>,
    );

    fn run(&mut self, data: Self::SystemData) {
        trace!("Entering TurnFlagResetterSystem::run().");
        let mut turn_flag_resource = data.0;
        turn_flag_resource.0 = true;
        trace!("Exiting TurnFlagResetterSystem::run().");
    }
}
