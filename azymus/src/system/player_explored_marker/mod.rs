use specs::*;
use super::super::component;
use component::field_of_view::FieldOfView;
use component::player::Player;
use component::player_explored::PlayerExplored;
use component::position::Position;
use component::tile::Tile;

/// Renderer.
#[derive(Clone, Copy, Debug)]
pub struct PlayerExploredMarkerSystem;

/// Renderer.
impl<'a> System<'a> for PlayerExploredMarkerSystem {

    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Tile>,
        ReadStorage<'a, FieldOfView>,
        WriteStorage<'a, PlayerExplored>,
    );

    fn run(&mut self, data: Self::SystemData) {
        trace!("Entering PlayerExploredMarkerSystem::run().");
        let (
            entities,
            player_storage,
            position_storage,
            tile_storage,
            fov_storage,
            mut player_explored_storage,
        ) = data;
        for (_, fov) in (&player_storage, &fov_storage).join() {
            if let Some(fov_map) = &fov.map {
                if let fov_map = fov_map.lock().unwrap() {
                    let mut explore_entities = vec![];
                    for (entity, position, _, _) in (&*entities, &position_storage, &tile_storage, !&player_explored_storage).join() {
                        if fov_map.is_in_fov(position.x, position.y) {
                            explore_entities.push(entity);
                        }
                    }
                    for entity in explore_entities {
                        player_explored_storage
                            .insert(entity, PlayerExplored)
                            .expect("Could not insert player-explored component for entity.");
                    }
                }
            }
        }
        trace!("Exiting PlayerExploredMarkerSystem::run().");
    }
}
