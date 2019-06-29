use specs::*;
use crate::component;
use component::actor::Actor;
use component::name::Name;
use crate::resource;
use resource::input_flag::InputFlagResource;

/// Feeds energy to every actor every turn.
#[derive(Clone, Copy, Debug)]
pub struct ActorFeederSystem;

/// Feeds energy to every actor every turn.
impl<'a> System<'a> for ActorFeederSystem {

    type SystemData = (
        Read<'a, InputFlagResource>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Actor>,
    );

    fn run(&mut self, data: Self::SystemData) {
        trace!("Entering ActorFeederSystem::run().");
        let (
            input_flag_resource,
            name_storage,
            mut actor_storage,
        ) = data;
        if !input_flag_resource.0 {
            for (name, actor) in (&name_storage, &mut actor_storage).join() {
                debug!("Feeding {} {} energy ({} -> {}).", name.name, actor.speed, actor.energy, actor.speed + actor.energy);
                actor.energy += actor.speed;
            }
        }
        trace!("Exiting ActorFeederSystem::run().");
    }
}
