use specs::*;
use std::cmp;
use rand::*;
use crate::component;
use component::occupant::Occupant;
use component::opaque::Opaque;
use component::position::Position;
use component::renderable::Renderable;
use component::tile::Tile;
use crate::entity;
use entity::npc::get_orc;
use entity::npc::get_troll;
use crate::resource;
use resource::occupant_map::OccupantMapResource;
use resource::opaque_map::OpaqueMapResource;
use crate::map::tile::preset::*;

const ROOM_MAX_SIZE: i32 = 25;
const ROOM_MIN_SIZE: i32 = 6;
const MAX_ROOMS: i32 = 40;
const MAX_ROOM_MONSTERS: i32 = 3;

#[derive(Clone, Copy, Debug)]
struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}


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

fn create_room(room: Rect, map: &mut Vec<Vec<bool>>) {
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            map[x as usize][y as usize] = false;
        }
    }
}

fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Vec<Vec<bool>>) {
    for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
        map[x as usize][y as usize] = false;
    }
}

fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Vec<Vec<bool>>) {
    for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
        map[x as usize][y as usize] = false;
    }
}

fn place_objects(world: &mut World, room: Rect, seed: i64, occupant_map: &mut Vec<Vec<bool>>) {
    let in_seed: &[_] = &[ seed as usize ];
    let mut rng: StdRng = SeedableRng::from_seed(in_seed);
    let num_monsters = rng.gen_range(0, MAX_ROOM_MONSTERS + 1);
    for _ in 0..num_monsters {
        let x = rng.gen_range(room.x1 + 1, room.x2);
        let y = rng.gen_range(room.y1 + 1, room.y2);
        if (rng.gen_range(0, 10) + x + y) % 10 < 8 {
            occupant_map[x as usize][y as usize] = true;
            get_orc(world, x, y, seed);
        } else {
            occupant_map[x as usize][y as usize] = true;
            get_troll(world, x, y, seed);
        };
    }
}

/// Generate the map.
pub fn generate_map(world: &mut World, width: i32, height: i32, seed: i64) -> (i32, i32) {
    let in_seed: &[_] = &[ seed as usize ];
    let mut rng: StdRng = SeedableRng::from_seed(in_seed);
    let mut map = vec![vec![true; height as usize]; width as usize];
    let mut occupant_map = vec![vec![false; height as usize]; width as usize];
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
            place_objects(world, new_room, seed, &mut occupant_map);
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
            let mut entity_builder = world.create_entity();
            let mut color = FLOOR_LIT_COLOR;
            if is_wall {
                entity_builder = entity_builder
                    .with(Occupant)
                    .with(Opaque);
                color = WALL_LIT_COLOR;
                occupant_map[x as usize][y as usize] = true;
            }
            let tile = Tile;
            let position = Position {
                x: x,
                y: y,
            };
            let renderable = Renderable {
                char: None,
                foreground_color: None,
                background_color: Some(color),
            };
            entity_builder
                .with(tile)
                .with(position)
                .with(renderable)
                .build();
        }
    }
    world.add_resource(OpaqueMapResource(map));
    world.add_resource(OccupantMapResource(occupant_map));
    starting_position
}
