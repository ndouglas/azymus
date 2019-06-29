use specs::*;
use crate::component;
use component::actor::Actor;
use component::baton::Baton;
use component::name::Name;

/// Passes the baton to the next actor.
#[derive(Clone, Copy, Debug)]
pub struct BatonPasserSystem;

/// Passes the baton to the next actor.
impl<'a> System<'a> for BatonPasserSystem {

    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Actor>,
        WriteStorage<'a, Baton>,
    );

    fn run(&mut self, data: Self::SystemData) {
        trace!("Entering BatonPasserSystem::run().");
        let (
            entities,
            name_storage,
            actor_storage,
            mut baton_storage,
        ) = data;
        let mut actor_tuples = (&entities, &name_storage, &actor_storage)
            .join()
            .collect::<Vec<_>>();
        actor_tuples
            .sort_by(|a, b| b.2.energy.cmp(&a.2.energy));
        debug!("{:?}", actor_tuples);
        if let Some(highest_energy_entity) = actor_tuples.first() {
            debug!("Passing baton to {}...", highest_energy_entity.1.name);
            if let Ok(_) = baton_storage.insert(highest_energy_entity.0, Baton) {
                debug!("Passed baton to {}...", highest_energy_entity.1.name);
            }
        };
        trace!("Exiting BatonPasserSystem::run().");
    }
}
