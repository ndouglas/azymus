use rand::*;
use std::cmp;
use crate::component;
use component::position::Position;
use crate::entity;
use entity::Entity;
use crate::math;
use math::geometry::rectangle::Rectangle;
use crate::seed;
use seed::SeedType;
use seed::RngType;
use crate::species;
use species::Factory as SpeciesFactory;
use crate::tile;
use tile::factory::Factory as TileFactory;
use super::super::MapGeneratorReturnType;
use super::super::super::MapType;

const ROOM_MAX_SIZE: i32 = 25;
const ROOM_MIN_SIZE: i32 = 6;

/// Creates a room.
fn create_room(room: Rectangle, map: &mut MapType) {
    for x in (room.x + 1)..room.x2() {
        for y in (room.y + 1)..room.y2() {
            if x % 5 != 0 || y % 5 != 0 {
                map[x][y] = TileFactory::Floor.create();
            }
        }
    }
}

fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut MapType) {
    for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
        map[x as usize][y as usize] = TileFactory::Floor.create();
    }
}

fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut MapType) {
    for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
        map[x as usize][y as usize] = TileFactory::Floor.create();
    }
}

fn place_objects(room: Rectangle, seed: SeedType, rng: &mut RngType, level: i32, entities: &mut Vec<Entity>) {
    let num_monsters = rng.gen_range(0, room.width);
    for _ in 0..num_monsters {
        let x = rng.gen_range(room.x + 1, room.x2());
        let y = rng.gen_range(room.y + 1, room.y2());
        let monster_num = rng.gen_range(0, 20);
        let mut monster = if monster_num < 3 {
            let mut orc = SpeciesFactory::Orc.create();
            orc.position = Some(Position {
                w: seed,
                x: x as i32,
                y: y as i32,
                z: level,
            });
            orc
        } else if monster_num < 4 {
            let mut troll = SpeciesFactory::Troll.create();
            troll.position = Some(Position {
                w: seed,
                x: x as i32,
                y: y as i32,
                z: level,
            });
            troll
        } else if monster_num < 8 {
            let mut goblin = SpeciesFactory::Goblin.create();
            goblin.position = Some(Position {
                w: seed,
                x: x as i32,
                y: y as i32,
                z: level,
            });
            goblin
        } else if monster_num < 10 {
            let mut kobold = SpeciesFactory::Kobold.create();
            kobold.position = Some(Position {
                w: seed,
                x: x as i32,
                y: y as i32,
                z: level,
            });
            kobold
        } else if monster_num < 13 {
            let mut chicken = SpeciesFactory::Chicken.create();
            chicken.position = Some(Position {
                w: seed,
                x: x as i32,
                y: y as i32,
                z: level,
            });
            chicken
        } else if monster_num < 17 {
            let mut mushroom = SpeciesFactory::Mushroom.create();
            mushroom.position = Some(Position {
                w: seed,
                x: x as i32,
                y: y as i32,
                z: level,
            });
            mushroom
        } else {
            let mut moss = SpeciesFactory::Moss.create();
            moss.position = Some(Position {
                w: seed,
                x: x as i32,
                y: y as i32,
                z: level,
            });
            moss
        };
        monster.id = entities.len();
        entities.push(monster);
    }
}

/// Generate the map.
pub fn generate_map(seed: SeedType, rng: &mut RngType, width: i32, height: i32, level: i32, entities: &mut Vec<Entity>) -> MapGeneratorReturnType {
    let mut map = vec![vec![TileFactory::Wall.create(); height as usize]; width as usize];
    let mut rooms = vec![];
    let mut starting_position = Position::new(seed, width / 2, height / 2, level);
    for _ in 0..width / 2 {
        let w = rng.gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        let h = rng.gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        let x = rng.gen_range(0, width - w);
        let y = rng.gen_range(0, height - h);
        let new_room = Rectangle::new(x as usize, y as usize, w as usize, h as usize);
        let failed = rooms
            .iter()
            .any(|other_room| new_room.overlaps(other_room));
        if !failed {
            create_room(new_room, &mut map);
            if !rooms.is_empty() {
                place_objects(new_room, seed, rng, level, entities);
            }
            let (new_x, new_y) = new_room.center().as_tuple();
            if rooms.is_empty() {
                starting_position = Position::new(seed, new_x as i32, new_y as i32, level);
            } else {
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center().as_tuple();
                if rng.gen() {
                    create_h_tunnel(prev_x as i32, new_x as i32, prev_y as i32, &mut map);
                    create_v_tunnel(prev_y as i32, new_y as i32, new_x as i32, &mut map);
                } else {
                    create_v_tunnel(prev_y as i32, new_y as i32, prev_x as i32, &mut map);
                    create_h_tunnel(prev_x as i32, new_x as i32, new_y as i32, &mut map);
                }
            }
            rooms.push(new_room);
        }
    }
    (map, starting_position)
}
