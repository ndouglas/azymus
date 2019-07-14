use rand::*;
use crate::command;
use command::Command;
use crate::component;
use component::position::Position;
//use crate::entity;
//use entity::Entity;
use crate::game;
use game::Game;
use crate::math;
use math::geometry::cell::Cell;
use math::geometry::compass::Direction as CompassDirection;
use math::geometry::rectangle::Rectangular;
use crate::species;
use species::Species;

/// Something that can act autonomously.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Agent {
    /// The algorithm used by this agent.
    pub algorithm: Algorithm,
}

/// Algorithms used to vend commands when given a context.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Algorithm {
    /// Just move South.
    JustMoveSouth,
    /// Approach the player.
    ApproachPlayer,
    /// Approach and fight the player.
    ApproachAndFightPlayer,
    /// Be a chicken.
    BeChicken,
    /// Be a mushroom.
    BeMushroom,
    /// Just be moss.
    BeMoss,
    /// Just be a moss seed.
    BeMossSeed,
}

/// Algorithms used to vend commands when given a context.
impl Algorithm {

    /// Get the command that this agent would like to execute.
    pub fn get_command(self, time: i32, id: usize, game: &Game) -> Option<Command> {
        trace!("Entering Algorithm::get_command().");
        use Algorithm::*;
        if time <= 0 {
            return None;
        }
        match self {
            JustMoveSouth => {
                Some(Command::Walk(CompassDirection::South))
            },
            ApproachPlayer => {
                let player = &game.entities[game.player_id];
                if let Some(player_position) = &player.position {
                    return command_to_move_towards(id, player_position, game);
                }
                None
            },
            ApproachAndFightPlayer => {
                let player = &game.entities[game.player_id];
                if let Some(player_position) = &player.position {
                    let entity = &game.entities[id];
                    if let Some(entity_position) = &entity.position {
                        if entity_position.distance_to(player_position) >= 2.0 {
                            return command_to_move_towards(id, player_position, game)
                        } else {
                            return command_to_attack(id, player_position, game);
                        }
                    }
                }
                None
            },
            BeChicken => {
                let mut rng = thread_rng();
                if rng.gen::<bool>() {
                    let direction = rng.gen::<CompassDirection>();
                    Some(Command::Walk(direction))
                } else {
                    None
                }
            },
            BeMushroom => {
                None
            },
            BeMoss => {
                let entity = &game.entities[id];
                if let Some(position) = entity.position {
                    let entities = &game.map
                        .entity_map
                        .hash_get_entity_ids_in_moore_neighborhood(&Cell::new(position.x as usize, position.y as usize))
                        .iter()
                        .map(|&id| &game.entities[id])
                        .cloned()
                        .filter(|e| e.species.is_some() && e.species.unwrap() == Species::Moss)
                        .collect::<Vec<_>>();
                    let count = entities.len();
                    match count {
                        1 | 3 | 5 | 8 => {
                            let map = &game.map;
                            let mut seed_positions: Vec<Position> = vec![];
                            if let Some(position) = &entity.position {
                                debug!("Entity {} ({}, {}) is following the moss-seed rule.", entity.name, position.x, position.y);
                                for dy in -1..=1 {
                                    for dx in -1..=1 {
                                        if dx == dy && dx == 0 {
                                            continue;
                                        }
                                        let final_x = (position.x + dx) as usize;
                                        let final_y = (position.y + dy) as usize;
                                        let cell = Cell::new(final_x, final_y);
                                        if !map.as_rectangle().contains_cell(&cell) {
                                            continue;
                                        }
                                        let mut seed_here: bool = true;
                                        if let Some(tile) = map.tile_map.get_tile(&cell) {
                                            if tile.blocks_movement || tile.blocks_light {
                                                seed_here = false;
                                            }
                                        }
                                        if let Some(entities) = map.entity_map.hash_get_entity_ids(&cell) {
                                            for id in entities {
                                                let entity = &game.entities[id];
                                                if let Some(species) = entity.species {
                                                    if species == Species::MossSeed || species == Species::Moss {
                                                        seed_here = false;
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                        if seed_here {
                                            seed_positions.push(Position {
                                                w: position.w,
                                                x: final_x as i32,
                                                y: final_y as i32,
                                                z: position.z,
                                            });
                                        }
                                    }
                                }
                            }
                            if seed_positions.len() == 0 {
                                return None;
                            }
                            let mut rng = thread_rng();
                            let index = rng.gen_range(0, seed_positions.len());
                            if let Some(direction) = position.direction_to(&seed_positions[index]) {
                                return Some(Command::MossSeed(direction));
                            }
                        },
                        _ => return Some(Command::MossDie),
                        //_ => {},
                    }
                }
                None
            },
            BeMossSeed => {
                let moss_seed = &game.entities[id];
                if let Some(position) = moss_seed.position {
                    let entities = &game.map
                        .entity_map
                        .hash_get_entity_ids_in_moore_neighborhood(&Cell::new(position.x as usize, position.y as usize))
                        .iter()
                        .map(|&id| &game.entities[id])
                        .cloned()
                        .filter(|e| e.species.is_some() && e.species.unwrap() == Species::Moss)
                        .collect::<Vec<_>>();
                    let count = entities.len();
                    match count {
                        3 | 5 | 7 => return Some(Command::MossBloom),
                        _ => return None,
                    }
                }
                None
            },
        }
    }

}

fn get_direction_to(id: usize, position: &Position, game: &Game) -> Option<CompassDirection> {
    let entity = &game.entities[id];
    if let Some(entity_position) = &entity.position {
        return entity_position.direction_to(position);
    }
    None
}

fn command_to_move_towards(id: usize, position: &Position, game: &Game) -> Option<Command> {
    if let Some(compass_direction) = get_direction_to(id, position, game) {
        return Some(Command::Walk(compass_direction));
    }
    None
}

fn command_to_attack(id: usize, position: &Position, game: &Game) -> Option<Command> {
    if let Some(compass_direction) = get_direction_to(id, position, game) {
        return Some(Command::MeleeAttack(compass_direction));
    }
    None
}
