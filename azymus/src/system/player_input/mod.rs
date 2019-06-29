use specs::*;
use crate::component;
use component::command_queue::CommandQueue;
use component::player::Player;
use crate::resource;
use resource::input_domain::InputDomainResource;
use resource::player_input::PlayerInputResource;

/// Renderer.
#[derive(Clone, Copy, Debug)]
pub struct PlayerInputSystem;

/// Renderer.
impl<'a> System<'a> for PlayerInputSystem {

    type SystemData = (
        Read<'a, InputDomainResource>,
        Write<'a, PlayerInputResource>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, CommandQueue>,
    );

    fn run(&mut self, data: Self::SystemData) {
        trace!("Entering PlayerInputSystem::run().");
        let (
            input_domain_resource,
            mut player_input_resource,
            player_storage,
            mut command_queue_storage,
        ) = data;
        let input_domain = input_domain_resource.0;
        if let Some(event) = player_input_resource.0 {
            trace!("Found event {:?} in player input resource.", event);
            player_input_resource.0 = None;
            if let Some(command) = input_domain.get_command(event) {
                for (_, command_queue) in (&player_storage, &mut command_queue_storage).join() {
                    trace!("Inserting command {:?} in player command queue.", command);
                    command_queue
                        .queue
                        .push_back(command);
                }
            }
        }
        trace!("Exiting PlayerInputSystem::run().");
    }
}
