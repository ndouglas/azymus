use rand::*;
use crate::command;
use command::Command;
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
                let player = &game.get_entity(game.player_id);
                let player_cell = &player.cell;
                command_to_move_towards(id, player_cell, game)
            },
            ApproachAndFightPlayer => {
                let entity = &game.get_entity(id);
                let player = &game.get_entity(game.player_id);
                let player_cell = &player.cell;
                let entity_cell = &entity.cell;
                if entity_cell.distance_to(player_cell) >= 2.0 {
                    command_to_move_towards(id, player_cell, game)
                } else {
                    command_to_attack(id, player_cell, game)
                }
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
                let entity = &game.get_entity(id);
                let cell = entity.cell;
                let entities = &game.map
                    .entity_map
                    .get_entity_ids_in_moore_neighborhood(&cell)
                    .iter()
                    .map(|&id| game.get_entity(id).clone() )
                    .filter(|e| e.species.is_some() && e.species.unwrap() == Species::Moss)
                    .collect::<Vec<_>>();
                let count = entities.len();
                match count {
                    1 | 3 | 5 | 8 => {
                        let map = &game.map;
                        let mut seed_cells: Vec<Cell> = vec![];
                        let cell = &entity.cell;
                        debug!("{} is following the moss-seed rule.", entity);
                        for neighbor_cell in cell.get_moore_neighborhood(&map.as_rectangle()) {
                            if !map.as_rectangle().contains_cell(&cell) {
                                continue;
                            }
                            let mut seed_here: bool = true;
                            if let Some(tile) = map.tile_map.get_tile(&cell) {
                                if tile.blocks_movement || tile.blocks_light {
                                    seed_here = false;
                                }
                            }
                            if let Some(entities) = map.entity_map.get_entity_ids_cell(&cell) {
                                for id in entities {
                                    let entity = &game.get_entity(id);
                                    if let Some(species) = entity.species {
                                        if species == Species::MossSeed || species == Species::Moss {
                                            seed_here = false;
                                            break;
                                        }
                                    }
                                }
                            }
                            if seed_here {
                                seed_cells.push(*cell);
                            }
                        }
                        if seed_cells.len() == 0 {
                            return None;
                        }
                        let mut rng = thread_rng();
                        let index = rng.gen_range(0, seed_cells.len());
                        let direction = cell.direction_to(&seed_cells[index]);
                        return Some(Command::MossSeed(direction));
                    },
                    _ => return Some(Command::MossDie),
                    //_ => {},
                }
            },
            BeMossSeed => {
                let moss_seed = &game.get_entity(id);
                let cell = moss_seed.cell;
                let entities = &game.map
                    .entity_map
                    .get_entity_ids_in_moore_neighborhood(&cell)
                    .iter()
                    .map(|&id| game.get_entity(id).clone())
                    .filter(|e| e.species.is_some() && e.species.unwrap() == Species::Moss)
                    .collect::<Vec<_>>();
                let count = entities.len();
                match count {
                    3 | 5 | 7 => return Some(Command::MossBloom),
                    _ => return None,
                }
            },
        }
    }

}

fn command_to_move_towards(id: usize, cell: &Cell, game: &Game) -> Option<Command> {
    let compass_direction = &game.get_entity(id).cell.direction_to(cell);
    Some(Command::Walk(*compass_direction))
}

fn command_to_attack(id: usize, cell: &Cell, game: &Game) -> Option<Command> {
    let compass_direction = &game.get_entity(id).cell.direction_to(cell);
    Some(Command::MeleeAttack(*compass_direction))
}
