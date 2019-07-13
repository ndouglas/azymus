use rand::prelude::*;
use std::cmp;
use crate::math;
use math::geometry::rectangle::Rectangle;
use crate::seed;
use seed::SeedType;
use crate::tile;
use tile::Factory as TileFactory;
use super::TileMapType;

const ROOM_MAX_SIZE: usize = 25;
const ROOM_MIN_SIZE: usize = 6;

/// Creates a room.
fn create_room(room: Rectangle, map: &mut TileMapType) {
    for x in (room.x + 1)..room.x2() {
        for y in (room.y + 1)..room.y2() {
            if map[x][y].blocks_movement {
                if x % 5 != 0 || y % 5 != 0 {
                    map[x][y] = TileFactory::Floor.create();
                }
            }
        }
    }
}

/// Creates a horizontal tunnel.
fn create_h_tunnel(x1: usize, x2: usize, y: usize, map: &mut TileMapType) {
    for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
        if map[x][y].blocks_movement {
            map[x][y] = TileFactory::Floor.create();
        }
    }
}

/// Creates a vertical tunnel.
fn create_v_tunnel(y1: usize, y2: usize, x: usize, map: &mut TileMapType) {
    for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
        if map[x][y].blocks_movement {
            map[x][y] = TileFactory::Floor.create();
        }
    }
}

/// Generate the map.
pub fn generate_tile_map(seed: SeedType, width: usize, height: usize) -> TileMapType {
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let mut map = vec![vec![TileFactory::Wall.create(); height]; width];
    let mut rooms = vec![];
    for _ in 0..width / 2 {
        let w = rng.gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        let h = rng.gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        let x = rng.gen_range(0, width - w);
        let y = rng.gen_range(0, height - h);
        let new_room = Rectangle::new(x, y, w, h);
        let failed = rooms
            .iter()
            .any(|other_room| new_room.overlaps(other_room));
        if !failed {
            create_room(new_room, &mut map);
            let (new_x, new_y) = new_room.center().as_tuple();
            if rooms.is_empty() {
                map[new_x][new_y].starting_position = true;
            } else {
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center().as_tuple();
                if rng.gen() {
                    create_h_tunnel(prev_x, new_x, prev_y, &mut map);
                    create_v_tunnel(prev_y, new_y, new_x, &mut map);
                } else {
                    create_v_tunnel(prev_y, new_y, prev_x, &mut map);
                    create_h_tunnel(prev_x, new_x, new_y, &mut map);
                }
            }
            rooms.push(new_room);
        }
    }
    map
}
