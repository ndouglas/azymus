use specs::*;
use crate::component::field_of_view::FieldOfView;
use crate::component::occupant::Occupant;
use crate::component::player::Player;
use crate::component::position::Position;
use crate::component::renderable::Renderable;
use tcod::colors::*;
use tcod::map::FovAlgorithm;

/// Create a player entity and return it.
pub fn get_player(world: &mut World, x: i32, y: i32) -> Entity {
    let entity = world.create_entity()
        .with(Player)
        .with(Occupant)
        .with(Position {
            x: x,
            y: y,
        })
        .with(Renderable {
            char: Some('@'),
            foreground_color: Some(WHITE),
            background_color: None,
        })
        .with(FieldOfView {
            algorithm: FovAlgorithm::Basic,
            radius: 10,
            map: None,
            x: -1,
            y: -1,
        })
        .build();
    entity
}
