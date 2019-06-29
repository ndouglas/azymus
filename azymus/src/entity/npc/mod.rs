use specs::*;
use crate::agent;
use agent::Agent as AgentType;
use crate::component;
use component::actor::Actor;
use component::agent::Agent;
use component::field_of_view::FieldOfView;
use component::name::Name;
use component::occupant::Occupant;
use component::position::Position;
use component::renderable::Renderable;
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
            speed: 10,
        })
        .with(Agent {
            agent: AgentType::Orc,
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
        .with(Agent {
            agent: AgentType::Orc,
        })
        .with(Actor {
            energy: 0,
            speed: 9,
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
