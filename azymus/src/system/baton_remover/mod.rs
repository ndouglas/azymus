use specs::*;
use crate::component;
use component::baton::Baton;
use component::name::Name;

/// Removes the baton from the current actor.
#[derive(Clone, Copy, Debug)]
pub struct BatonRemoverSystem;

/// Removes the baton from the current actor.
impl<'a> System<'a> for BatonRemoverSystem {

    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Baton>,
    );

    fn run(&mut self, data: Self::SystemData) {
        trace!("Entering BatonRemoverSystem::run().");
        let (
            entities,
            name_storage,
            mut baton_storage,
        ) = data;
        let mut baton_entities = vec![];
        for (entity, name, _) in (&entities, &name_storage, &baton_storage).join() {
            baton_entities.push((entity, name));
        }
        for (entity, name) in baton_entities {
            debug!("Stripping baton from {}...", name.name);
            if let Some(_) = baton_storage.remove(entity) {
                debug!("Removed baton from {}...", name.name);
            }
        }
        trace!("Exiting BatonRemoverSystem::run().");
    }
}
