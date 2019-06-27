use specs::*;
use crate::component::actor::Actor;
use crate::component::field_of_view::FieldOfView;
use crate::component::name::Name;
use crate::component::occupant::Occupant;
use crate::component::position::Position;
use crate::component::renderable::Renderable;
use tcod::colors::*;
use tcod::map::FovAlgorithm;

/// Create an orc entity and return it.
pub fn get_orc(world: &mut World, x: i32, y: i32, _seed: i64) -> Entity {
    let entity = world.create_entity()
        .with(Occupant)
        .with(Name {
            name: "Orc".to_string(),
        })
        .with(Actor {
            energy: 0,
            speed: 100,
            queue: vec![],
        })
        .with(Position {
            x: x,
            y: y,
        })
        .with(Renderable {
            char: Some('o'),
            foreground_color: Some(DESATURATED_GREEN),
            background_color: None,
        })
        .with(FieldOfView {
            algorithm: FovAlgorithm::Basic,
            radius: 12,
            map: None,
            x: -1,
            y: -1,
        })
        .build();
    entity
}

/// Create a troll entity and return it.
pub fn get_troll(world: &mut World, x: i32, y: i32, _seed: i64) -> Entity {
    let entity = world.create_entity()
        .with(Occupant)
        .with(Name {
            name: "Troll".to_string(),
        })
        .with(Actor {
            energy: 0,
            speed: 70,
            queue: vec![],
        })
        .with(Position {
            x: x,
            y: y,
        })
        .with(Renderable {
            char: Some('T'),
            foreground_color: Some(DARK_GREEN),
            background_color: None,
        })
        .with(FieldOfView {
            algorithm: FovAlgorithm::Basic,
            radius: 7,
            map: None,
            x: -1,
            y: -1,
        })
        .build();
    entity
}
