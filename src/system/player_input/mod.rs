use specs::*;
use crate::component;
use component::actor::Actor;
use component::player::Player;
use crate::resource;
use resource::input_domain::InputDomainResource;
use resource::input_flag::InputFlagResource;
use resource::player_input::PlayerInputResource;

/// Receives and acts on player input.
#[derive(Clone, Copy, Debug)]
pub struct PlayerInputSystem;

/// Receives and acts on player input.
impl<'a> System<'a> for PlayerInputSystem {

    type SystemData = (
        Read<'a, InputDomainResource>,
        Write<'a, InputFlagResource>,
        Write<'a, PlayerInputResource>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Actor>,
    );

    fn run(&mut self, data: Self::SystemData) {
        trace!("Entering PlayerInputSystem::run().");
        let (
            input_domain_resource,
            mut input_flag_resource,
            mut player_input_resource,
            player_storage,
            mut actor_storage,
        ) = data;
        let input_domain = input_domain_resource.0;
        if let Some(event) = player_input_resource.0 {
            debug!("Found event {:?} in player input resource.", event);
            player_input_resource.0 = None;
            input_flag_resource.0 = false;
            if let Some(command) = input_domain.get_command(event) {
                for (_, actor) in (&player_storage, &mut actor_storage).join() {
                    debug!("Inserting command {:?} in player command queue.", command);
                    actor
                        .command_queue
                        .push_back(command);
                }
            }
        }
        trace!("Exiting PlayerInputSystem::run().");
    }
}
