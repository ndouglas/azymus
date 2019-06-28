use specs::*;
use crate::component;
use component::actor::Actor;
use component::name::Name;

/// Renderer.
#[derive(Clone, Copy, Debug)]
pub struct ActorFeederSystem;

/// Renderer.
impl<'a> System<'a> for ActorFeederSystem {

    type SystemData = (
        ReadStorage<'a, Name>,
        WriteStorage<'a, Actor>,
    );

    fn run(&mut self, data: Self::SystemData) {
        trace!("Entering ActorFeederSystem::run().");
        let (
            name_storage,
            mut actor_storage,
        ) = data;
        for (name, actor) in (&name_storage, &mut actor_storage).join() {
            trace!("Feeding {} {} energy ({} -> {}).", name.name, actor.speed, actor.energy, actor.speed + actor.energy);
            actor.energy += actor.speed;
        }
        trace!("Exiting ActorFeederSystem::run().");
    }
}
