use specs::*;
use super::super::component;
use component::field_of_view::FieldOfView;
use component::position::Position;
use super::super::resource;
use resource::occupant_map::OccupantMapResource;
use resource::opaque_map::OpaqueMapResource;
use tcod::map::*;
use std::sync::{Arc, Mutex};

/// The FOV system.
#[derive(Clone, Copy, Debug)]
pub struct FieldOfViewSystem;

/// The FOV system.
impl<'a> System<'a> for FieldOfViewSystem {

    type SystemData = (
        ReadExpect<'a, OccupantMapResource>,
        ReadExpect<'a, OpaqueMapResource>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, FieldOfView>,
    );

    fn run(&mut self, data: Self::SystemData) {
        trace!("Entering FieldOfViewSystem::run().");
        let (
            occupant_map_resource,
            opaque_map_resource,
            position_storage,
            mut fov_storage,
        ) = data;
        let occupant_map = &occupant_map_resource.0;
        let opaque_map = &opaque_map_resource.0;
        let width = opaque_map.len() as i32;
        let height = opaque_map[0].len() as i32;
        for (position, fov) in (&position_storage, &mut fov_storage).join() {
            trace!("Found position and FOV at ({}, {}).", position.x, position.y);
            if fov.map.is_none() {
                debug!("Creating new FOV for ({}, {}).", position.x, position.y);
                let mut fov_map = Map::new(width, height);
                for y in 0..height {
                    for x in 0..width {
                        fov_map.set(
                            x,
                            y,
                            !opaque_map[x as usize][y as usize],
                            !occupant_map[x as usize][y as usize],
                        );
                    }
                }
                fov.map = Some(Arc::new(Mutex::new(fov_map)));
            }
            let dirty = position.x != fov.x || position.y != fov.y;
            if dirty {
                debug!("Recalculating FOV for ({}, {}).", position.x, position.y);
                let fov_map = &mut fov.map.as_mut().unwrap();
                let mut fov_map = fov_map.lock().unwrap();
                fov_map.compute_fov(
                    position.x,
                    position.y,
                    fov.radius,
                    true,
                    fov.algorithm
                );
                fov.x = position.x;
                fov.y = position.y;
            }
        }
        trace!("Exiting FieldOfViewSystem::run().");
    }
}
