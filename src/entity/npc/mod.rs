use std::collections::VecDeque;
use specs::*;
use rand::*;
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
pub fn get_orc(world: &mut World, x: i32, y: i32, seed: i64) -> Entity {
    let in_seed: &[_] = &[ seed as usize ];
    let mut rng: StdRng = SeedableRng::from_seed(in_seed);
    let entity = world.create_entity()
        .with(Occupant)
        .with(Name {
            name: format!("Orc ({}, {})", x, y),
        })
        .with(Actor {
            energy: rng.gen_range(0, 3) + x % 3 + y % 3,
            speed: rng.gen_range(6, 8) + x % 3 + y % 3,
            command_queue: VecDeque::new(),
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
pub fn get_troll(world: &mut World, x: i32, y: i32, seed: i64) -> Entity {
    let in_seed: &[_] = &[ seed as usize ];
    let mut rng: StdRng = SeedableRng::from_seed(in_seed);
    let entity = world.create_entity()
        .with(Occupant)
        .with(Name {
            name: format!("Troll ({}, {})", x, y),
        })
        .with(Agent {
            agent: AgentType::Orc,
        })
        .with(Actor {
            energy: rng.gen_range(0, 3) + x % 3 + y % 3,
            speed: rng.gen_range(3, 6) + x % 3 + y % 3,
            command_queue: VecDeque::new(),
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
