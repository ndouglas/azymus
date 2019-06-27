use specs::*;
use std::cmp;
use rand::*;
use crate::map::tile::get_tile;

const ROOM_MAX_SIZE: i32 = 25;
const ROOM_MIN_SIZE: i32 = 6;
const MAX_ROOMS: i32 = 40;

#[derive(Clone, Copy, Debug)]
struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

/// Very simple, just "is this a wall or not?"
pub type MapType = Vec<Vec<bool>>;

impl Rect {

    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    pub fn center(&self) -> (i32, i32) {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        (center_x, center_y)
    }

    pub fn intersects_with(&self, other: &Rect) -> bool {
        (self.x1 <= other.x2)
            && (self.x2 >= other.x1)
            && (self.y1 <= other.y2)
            && (self.y2 >= other.y1)
    }

}

fn create_room(room: Rect, map: &mut MapType) {
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            map[x as usize][y as usize] = false;
        }
    }
}

fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut MapType) {
    for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
        map[x as usize][y as usize] = false;
    }
}

fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut MapType) {
    for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
        map[x as usize][y as usize] = false;
    }
}

/// Generate the map.
pub fn generate_map(world: &mut World, width: i32, height: i32, seed: i64) -> (i32, i32) {
    let in_seed: &[_] = &[ seed as usize ];
    let mut rng: StdRng = SeedableRng::from_seed(in_seed);
    let mut map = vec![vec![true; height as usize]; width as usize];
    let mut starting_position = (0, 0);
    let mut rooms = vec![];
    for _ in 0..MAX_ROOMS {
        let w = rng.gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        let h = rng.gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        let x = rng.gen_range(0, width - w);
        let y = rng.gen_range(0, height - h);
        let new_room = Rect::new(x, y, w, h);
        let failed = rooms
            .iter()
            .any(|other_room| new_room.intersects_with(other_room));
        if !failed {
            create_room(new_room, &mut map);
            let (new_x, new_y) = new_room.center();
            if rooms.is_empty() {
                starting_position = (new_x, new_y);
            } else {
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
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
    for y in 0..height {
        for x in 0..width {
            let is_wall = map[x as usize][y as usize];
            get_tile(world, is_wall, x, y);
        }
    }
    starting_position
}
