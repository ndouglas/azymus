use specs::*;
use crate::action;
use action::Action;
use crate::component;
use component::name::Name;
use component::position::Position;
use crate::resource;
use resource::occupant_map::OccupantMapResource;
use resource::seed::SeedResource;

/// Processes walk actions.
#[derive(Clone, Copy, Debug)]
pub struct WalkSystem;

/// Processes walk actions.
impl<'a> System<'a> for WalkSystem {

    type SystemData = (
        Entities<'a>,
        Read<'a, SeedResource>,
        WriteExpect<'a, OccupantMapResource>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Action>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        trace!("Entering WalkSystem::run().");
        let (
            entities,
            seed_resource,
            mut occupant_map_resource,
            name_storage,
            mut action_storage,
            mut position_storage,
        ) = data;
        let _seed = seed_resource.0;
        let mut acted_entities = vec![];
        for (entity, name, action, position) in (&entities, &name_storage, &mut action_storage, &mut position_storage).join() {
            if let Action::Walk((x1, y1), (x2, y2)) = action {
                debug!("Walking entity {} to position ({}, {}).", name.name, x2, y2);
                occupant_map_resource.0[*x1 as usize][*y1 as usize] = false;
                occupant_map_resource.0[*x2 as usize][*y2 as usize] = true;
                position.x = *x2;
                position.y = *y2;
                acted_entities.push(entity);
            }
        }
        for entity in acted_entities {
            if let Some(_) = action_storage.remove(entity) {
                if let Some(name) = name_storage.get(entity) {
                    debug!("Removed walk action component from entity {}!", name.name);
                }
            }
        }
        trace!("Exiting WalkSystem::run().");
    }
}
