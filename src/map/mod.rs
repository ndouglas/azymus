use std::collections::{HashMap, HashSet};
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
    pub height: usize,
    /// The width of the map.
    pub width: usize,
    /// The spatial hash map.
    pub spatial_hash: HashMap<(usize, usize), HashSet<usize>>,
}

/// The map object.
impl Map {

    /// Constructor.
    pub fn new(map: MapType) -> Self {
        let height = map[0].len();
        let width = map.len();
        let mut spatial_hash = HashMap::new();
        for y in 0..height {
            for x in 0..width {
                spatial_hash.insert((x, y), HashSet::new());
            }
        }
        Map {
            map: map,
            height: height,
            width: width,
            spatial_hash: spatial_hash,
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
        let mut map = FovMap::new(width as i32, height as i32);
        for y in 0..height {
            for x in 0..width {
                let blocks_light = self.map[x][y].blocks_light;
                let blocks_movement = self.map[x][y].blocks_movement;
                map.set(x as i32, y as i32, !blocks_light, !blocks_movement);
            }
        }
        map
    }

    /// Indicates whether a position is in bounds of this map.
    pub fn is_position_in_bounds(&self, position: &Position) -> bool {
        self.is_in_bounds(position.x as usize, position.y as usize)
    }

    /// Indicates whether a pair of coordinates are in bounds of this map.
    pub fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        (x < self.width - 1 && y < self.height - 1)
    }

    /// Render this entity, taking into account the provided field of view.
    pub fn draw_fov(&self, console: &mut Console, fov: &FieldOfView) {
        trace!("Entering Map::draw().");
        let fov_map = fov.map.lock().unwrap();
        for y in 0..console.height() {
            for x in 0..console.width() {
                if fov_map.is_in_fov(x, y) {
                    self.map[x as usize][y as usize].draw_in_fov(console);
                } else if self.is_in_bounds(x as usize, y as usize) && fov.explored_map[x as usize][y as usize] {
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
                } else if self.is_in_bounds(x as usize, y as usize) && fov.explored_map[x as usize][y as usize] {
                    self.map[x as usize][y as usize].draw(console);
                }
            }
        }
        trace!("Exiting Map::draw().");
    }

    /// Indicates whether a pair of coordinates are in bounds of this map.
    pub fn get_tile(&self, x: usize, y: usize) -> &Tile {
        &self.map[x][y]
    }

    /// Indicates whether a pair of coordinates are in bounds of this map.
    pub fn get_tile_at_position(&self, position: &Position) -> &Tile {
        self.get_tile(position.x as usize, position.y as usize)
    }

    /// Removes an entity at the specified position.
    pub fn remove_entity(&mut self, id: usize, x: usize, y: usize) {
        if let Some(set) = self.spatial_hash.get_mut(&(x, y)) {
            set.remove(&id);
        }
    }

    /// Adds an entity at the specified position.
    pub fn insert_entity(&mut self, id: usize, x: usize, y: usize) {
        if let Some(set) = self.spatial_hash.get_mut(&(x, y)) {
            set.insert(id);
        }
    }

    /// Adds an entity at the specified position.
    pub fn move_entity(&mut self, id: usize, x1: usize, y1: usize, x2: usize, y2: usize) {
        self.remove_entity(id, x1, y1);
        self.insert_entity(id, x2, y2);
    }

    /// Gets entity IDs at a specific location.
    pub fn get_entities(&self, x: usize, y: usize) -> HashSet<usize> {
        self.spatial_hash.get(&(x, y)).unwrap().clone()
    }

}

/// Get a new map.
pub fn get_map(seed: i64, width: i32, height: i32, level: i32, entities: &mut Vec<Entity>) -> (Map, Position) {
    let (inner_map, position) = generator::algorithm::Algorithm::Simple.generate_map(seed, width, height, level, entities);
    let mut map = Map::new(inner_map);
    for entity in entities {
        entity.field_of_view = Some(FieldOfView::new(map.get_fov(), 10));
        if let Some(position) = &entity.position {
            map.insert_entity(entity.id, position.x as usize, position.y as usize);
        }
    }
    (map, position)
}
