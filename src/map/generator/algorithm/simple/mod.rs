use rand::*;
use std::cmp;
use crate::component;
use component::position::Position;
use crate::entity;
use entity::Entity;
use crate::species;
use species::Factory as SpeciesFactory;
use crate::tile;
use tile::Tile;
use super::super::MapGeneratorReturnType;
use super::super::super::MapType;

const ROOM_MAX_SIZE: i32 = 25;
const ROOM_MIN_SIZE: i32 = 6;

/// A rectangular room.
#[derive(Clone, Copy, Debug)]
struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

/// A rectangular room carved out of solid rock.
impl Rect {

    /// Constructor.
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    /// Retrieve the center of this room.
    pub fn center(&self) -> (i32, i32) {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        (center_x, center_y)
    }

    /// Indicates whether this room intersects with another.
    pub fn intersects_with(&self, other: &Rect) -> bool {
        (self.x1 <= other.x2)
            && (self.x2 >= other.x1)
            && (self.y1 <= other.y2)
            && (self.y2 >= other.y1)
    }

}

/// Creates a room.
fn create_room(seed: i64, level: i32, room: Rect, map: &mut MapType) {
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            map[x as usize][y as usize] = Tile::floor(seed, x, y, level);
        }
    }
}

fn create_h_tunnel(seed: i64, level: i32, x1: i32, x2: i32, y: i32, map: &mut MapType) {
    for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
        map[x as usize][y as usize] = Tile::floor(seed, x, y, level);
    }
}

fn create_v_tunnel(seed: i64, level: i32, y1: i32, y2: i32, x: i32, map: &mut MapType) {
    for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
        map[x as usize][y as usize] = Tile::floor(seed, x, y, level);
    }
}

fn place_objects(room: Rect, seed: i64, level: i32, entities: &mut Vec<Entity>) {
    let in_seed: &[_] = &[seed as usize];
    let mut rng: StdRng = SeedableRng::from_seed(in_seed);
    let num_monsters = rng.gen_range(0, (room.y2 - room.y1).abs());
    for _ in 0..num_monsters {
        let x = rng.gen_range(room.x1 + 1, room.x2);
        let y = rng.gen_range(room.y1 + 1, room.y2);
        let monster = if rng.gen_range(0, 10) < 8 {
            let mut orc = SpeciesFactory::Orc.create();
            orc.position = Some(Position {
                w: seed,
                x: x,
                y: y,
                z: level,
            });
            orc
        } else {
            let mut troll = SpeciesFactory::Troll.create();
            troll.position = Some(Position {
                w: seed,
                x: x,
                y: y,
                z: level,
            });
            troll
        };
        entities.push(monster);
    }
}

/// Generate the map.
pub fn generate_map(seed: i64, width: i32, height: i32, level: i32, entities: &mut Vec<Entity>) -> MapGeneratorReturnType {
    let in_seed: &[_] = &[seed as usize];
    let mut rng: StdRng = SeedableRng::from_seed(in_seed);
    let mut map = vec![vec![Tile::new(); height as usize]; width as usize];
    for y in 0..height {
        for x in 0..width {
            map[x as usize][y as usize] = Tile::wall(seed, x, y, level);
        }
    }
    let mut rooms = vec![];
    let mut starting_position = Position::new(seed, width / 2, height / 2, level);
    for _ in 0..width / 2 {
        let w = rng.gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        let h = rng.gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        let x = rng.gen_range(0, width - w);
        let y = rng.gen_range(0, height - h);
        let new_room = Rect::new(x, y, w, h);
        let failed = rooms
            .iter()
            .any(|other_room| new_room.intersects_with(other_room));
        if !failed {
            create_room(seed, level, new_room, &mut map);
            if !rooms.is_empty() {
                place_objects(new_room, seed, level, entities);
            }
            let (new_x, new_y) = new_room.center();
            if rooms.is_empty() {
                starting_position = Position::new(seed, new_x, new_y, level);
            } else {
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                if rng.gen() {
                    create_h_tunnel(seed, level, prev_x, new_x, prev_y, &mut map);
                    create_v_tunnel(seed, level, prev_y, new_y, new_x, &mut map);
                } else {
                    create_v_tunnel(seed, level, prev_y, new_y, prev_x, &mut map);
                    create_h_tunnel(seed, level, prev_x, new_x, new_y, &mut map);
                }
            }
            rooms.push(new_room);
        }
    }
    (map, starting_position)
}
