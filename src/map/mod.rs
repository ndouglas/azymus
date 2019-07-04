use tcod::console::*;
use crate::component;
use component::field_of_view::FieldOfView;
use component::position::Position;
use component::light_source::LightSource;
use crate::entity;
use entity::Entity;
use crate::tile;
use tile::Tile;
use tcod::map::Map as FovMap;

/// The map generators.
pub mod generator;

/// The map type.
pub type MapType = Vec<Vec<Tile>>;

/// The map object.
#[derive(Clone, Debug)]
pub struct Map {
    /// The actual inner map.
    map: MapType,
    /// The height of the map.
    pub height: i32,
    /// The width of the map.
    pub width: i32,
}

/// The map object.
impl Map {

    /// Constructor.
    pub fn new(map: MapType) -> Self {
        let height = map[0].len() as i32;
        let width = map.len() as i32;
        Map {
            map: map,
            height: height,
            width: width,
        }
    }

    /// Render this entity.
    pub fn draw(&self, console: &mut Console) {
        trace!("Entering Map::draw().");
        for y in 0..console.height() {
            for x in 0..console.width() {
                self.map[x as usize][y as usize].draw(console);
            }
        }
        trace!("Exiting Map::draw().");
    }

    /// Initialize a field-of-view object.
    pub fn get_fov(&self) -> FovMap {
        let height = self.height;
        let width = self.width;
        let mut map = FovMap::new(width, height);
        for y in 0..height {
            for x in 0..width {
                let blocks_light = self.map[x as usize][y as usize].blocks_light;
                let blocks_movement = self.map[x as usize][y as usize].blocks_movement;
                map.set(x, y, !blocks_light, !blocks_movement);
            }
        }
        map
    }

    /// Indicates whether a pair of coordinates are in bounds of this map.
    pub fn is_in_bounds(&self, x: i32, y: i32) -> bool {
        (x >= 0 && y >= 0 && x < self.width - 1 && y < self.height - 1)
    }

    /// Render this entity, taking into account the provided field of view.
    pub fn draw_fov(&self, console: &mut Console, fov: &FieldOfView) {
        trace!("Entering Map::draw().");
        let fov_map = fov.map.lock().unwrap();
        for y in 0..console.height() {
            for x in 0..console.width() {
                if fov_map.is_in_fov(x, y) {
                    self.map[x as usize][y as usize].draw_in_fov(console);
                } else if self.is_in_bounds(x, y) && fov.explored_map[x as usize][y as usize] {
                    self.map[x as usize][y as usize].draw(console);
                }
            }
        }
        trace!("Exiting Map::draw().");
    }

    /// Render this entity, taking into account the provided field of view.
    pub fn draw_fov_ls(&self, console: &mut Console, fov: &FieldOfView, ls: &LightSource) {
        trace!("Entering Map::draw().");
        let fov_x = fov.x;
        let fov_y = fov.y;
        let fov_map = fov.map.lock().unwrap();
        for y in 0..console.height() {
            for x in 0..console.width() {
                if fov_map.is_in_fov(x, y) {
                    self.map[x as usize][y as usize].draw_lighted(console, ls, fov_x, fov_y);
                } else if self.is_in_bounds(x, y) && fov.explored_map[x as usize][y as usize] {
                    self.map[x as usize][y as usize].draw(console);
                }
            }
        }
        trace!("Exiting Map::draw().");
    }

    /// Indicates whether a pair of coordinates are in bounds of this map.
    pub fn get_tile(&self, x: i32, y: i32) -> &Tile {
        &self.map[x as usize][y as usize]
    }

}

/// Get a new map.
pub fn get_map(seed: i64, width: i32, height: i32, level: i32, objects: &mut Vec<Entity>) -> (Map, Position) {
    let (inner_map, position) = generator::algorithm::Algorithm::Simple.generate_map(seed, width, height, level, objects);
    let map = Map::new(inner_map);
    (map, position)
}
