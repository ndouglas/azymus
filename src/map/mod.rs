use tcod::map::Map as FovMap;
use crate::math;
use math::geometry::rectangle::Rectangle;
use math::geometry::rectangle::Rectangular;
use crate::seed;
use seed::SeedType;

/// The entity map.
pub mod entity_map;
use entity_map::EntityMap;

/// The tile map.
pub mod tile_map;
use tile_map::TileMap;
use tile_map::Factory as TileMapFactory;

/// The map -- the world, as far as we're concerned.
#[derive(Debug)]
pub struct Map {
    /// The width.
    pub width: usize,
    /// The height.
    pub height: usize,
    /// The actual tile map.
    pub tile_map: TileMap,
    /// The entity map.
    pub entity_map: EntityMap,
}

/// The map.
impl Map {

    /// Constructor.
    pub fn new(tile_map: TileMap) -> Self {
        let width = tile_map.width;
        let height = tile_map.height;
        Map {
            width: width,
            height: height,
            tile_map: tile_map,
            entity_map: EntityMap::new(width, height),
        }
    }

    /// Initialize a field-of-view object.
    pub fn get_fov(&self) -> FovMap {
        let width = self.width;
        let height = self.height;
        let mut map = FovMap::new(width as i32, height as i32);
        for y in 0..height {
            for x in 0..width {
                let blocks_light = self.tile_map.vector[x][y].blocks_light;
                let blocks_movement = self.tile_map.vector[x][y].blocks_movement;
                map.set(x as i32, y as i32, !blocks_light, !blocks_movement);
            }
        }
        map
    }

}


/// Allows us to create a rectangle representation of this map.
impl Rectangular for Map {

    /// Create a rectangle from this object.
    fn as_rectangle(&self) -> Rectangle {
        Rectangle {
            x: 0,
            y: 0,
            width: self.width,
            height: self.height,
        }
    }

}

/// Get a new map.
pub fn get_map(seed: SeedType, width: i32, height: i32) -> Map {
    let inner_map = TileMapFactory::Simple.create(seed, width as usize, height as usize);
    let map = Map::new(inner_map);
    map
}

/*
use std::collections::{HashMap, HashSet};
use std::cmp;
use std::fmt;
use bear_lib_terminal::terminal as blt;
use ::ntree::NTree;
use crate::component;
use component::field_of_view::FieldOfView;
use component::position::Position;
use component::renderable::Renderable;
use crate::entity;
use entity::Entity;
use crate::game;
use game::Game;
use crate::seed;
use seed::SeedType;
use seed::RngType;
use crate::species;
use species::Species;
use crate::tile;
use tile::Tile;
use crate::ui;
use ui::Ui;

/// Our quad-tree type.
pub type QuadTreeType = NTree<QuadTreeRegion, QuadTreePoint>;

/// The map object.
pub struct Map0 {
    /// The actual inner map.
    pub tile_map: TileMapType,
    /// The height of the map.
    pub height: usize,
    /// The width of the map.
    pub width: usize,
    /// The spatial hash map.
    pub spatial_hash: HashMap<(usize, usize), HashSet<usize>>,
}

/// The map object.
impl Map0 {

    /// Constructor.
    pub fn new(tile_map: TileMapType) -> Self {
        let height = tile_map[0].len();
        let width = tile_map.len();
        let mut spatial_hash = HashMap::new();
        for y in 0..height {
            for x in 0..width {
                spatial_hash.insert((x, y), HashSet::new());
            }
        }
        Map0 {
            tile_map: tile_map,
            height: height,
            width: width,
            spatial_hash: spatial_hash,
        }
    }

    /// Indicates whether a position is in bounds of this map.
    pub fn is_position_in_bounds(&self, position: &Position) -> bool {
        self.is_in_bounds(position.x as usize, position.y as usize)
    }

    /// Indicates whether a pair of coordinates are in bounds of this map.
    pub fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        (x < self.width - 1 && y < self.height - 1)
    }

    /// Get a light source quadtree for the map.
    pub fn get_ls_tree(&self, game: &Game) -> QuadTreeType {
        let mut ls_tree: QuadTreeType = NTree::new(QuadTreeRegion {
            x: 0,
            y: 0,
            width: self.width as i32,
            height: self.height as i32,
        }, 16);
        for y in 0..self.height {
            for x in 0..self.width {
                let ids = self.get_entities(x, y)
                    .unwrap_or(HashSet::new())
                    .iter()
                    .filter(|&id| game.entities[*id].light_source.is_some())
                    .map(|&id| id)
                    .collect::<Vec<usize>>();
                for id in ids {
                    ls_tree.insert(QuadTreePoint {
                        id: id,
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
        }
        ls_tree
    }

    /// Get a quadtree for all entities of a specific species for this map.
    pub fn get_species_tree(&self, game: &Game, species: Species) -> QuadTreeType {
        let mut ls_tree: QuadTreeType = NTree::new(QuadTreeRegion {
            x: 0,
            y: 0,
            width: self.width as i32,
            height: self.height as i32,
        }, 4);
        for y in 0..self.height {
            for x in 0..self.width {
                let ids = self.get_entities(x, y)
                    .unwrap_or(HashSet::new())
                    .iter()
                    .filter(|&id| game.entities[*id].species.is_some())
                    .filter(|&id| game.entities[*id].species.unwrap() == species)
                    .map(|&id| id)
                    .collect::<Vec<usize>>();
                for id in ids {
                    ls_tree.insert(QuadTreePoint {
                        id: id,
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
        }
        ls_tree
    }

    /// Render this map, taking into account the provided field of view.
    pub fn draw(&self, _ui: &Ui, fov: &FieldOfView, game: &Game) {
        trace!("Entering Map::draw().");
        let fov_map = fov.map.lock().unwrap().clone();
        let ls_tree = self.get_ls_tree(game);
        let radius: i32 = 12;
        for y in 0..self.height {
            for x in 0..self.width {
                if fov_map.is_in_fov(x as i32, y as i32) {
                    let region = QuadTreeRegion {
                        x: cmp::max(x as i32 - radius, 0),
                        y: cmp::max(y as i32 - radius, 0),
                        width: cmp::min(x  as i32 + radius, self.width as i32 - 1),
                        height: cmp::min(y as i32 + radius, self.height as i32 - 1),
                    };
                    let ls_vector = ls_tree
                        .range_query(&region)
                        .map(|x| x.id)
                        .collect::<Vec<usize>>();
                    let renderable = &self.tile_map[x][y].renderable;
                    self.draw_tile_renderable(x, y, &renderable, game, &ls_vector);
                    let mut occupant_found: bool = false;
                    for id in self.get_entities(x, y)
                        .unwrap_or(HashSet::new())
                        .iter()
                        .collect::<Vec<&usize>>() {
                        if occupant_found {
                            break;
                        }
                        let entity = &game.entities[*id];
                        occupant_found = entity.blocks_movement;
                        if let Some(renderable) = &entity.renderable {
                            self.draw_entity_renderable(x, y, &renderable);
                        }
                    }
                } else if self.is_in_bounds(x, y) && fov.explored_map[x][y] {
                    let ls_vector = vec![];
                    let renderable = &self.tile_map[x][y].renderable;
                    self.draw_tile_renderable(x, y, &renderable, game, &ls_vector);
                }
            }
        }
        trace!("Exiting Map::draw().");
    }

    /// Render this object at the specified position.
    pub fn draw_tile_renderable(&self, x: usize, y: usize, renderable: &Renderable, game: &Game, ls_vector: &Vec<usize>) {
        trace!("Entering Renderable::draw_tile_renderable().");
        use bear_lib_terminal::geometry::Point;
        let point = Point::new(x as i32, y as i32);
        let mut bg_color = blt::pick_background_color(point);
        let mut fg_color = blt::pick_foreground_color(point, 0);
        let mut the_char = ' ';
        if let Some(color) = renderable.background_color {
            bg_color = color.to_blt();
        }
        if let Some(color) = renderable.foreground_color {
            fg_color = color.to_blt();
        }
        if let Some(char) = renderable.char {
            the_char = char;
        }
        for id in ls_vector {
            let entity = &game.entities[*id];
            if let Some(position) = entity.position {
                if let Some(light_source) = entity.light_source {
                    if let Some(fov) = &entity.field_of_view {
                        let fov_map = fov.map.lock().unwrap();
                        if fov_map.is_in_fov(x as i32, y as i32) {
                            bg_color = light_source.transform_color_at(bg_color, position.x, position.y, x as i32, y as i32);
                        }
                    }
                }
            }
        }
        blt::with_colors(fg_color, bg_color, || blt::put_xy(x as i32, y as i32, the_char));
        trace!("Exiting Renderable::draw_entity_renderable().");
    }

    /// Draw an entity renderable.
    pub fn draw_entity_renderable(&self, x: usize, y: usize, renderable: &Renderable) {
        trace!("Entering Renderable::draw_entity_renderable().");
        use bear_lib_terminal::geometry::Point;
        let point = Point::new(x as i32, y as i32);
        let mut bg_color = blt::pick_background_color(point);
        let mut fg_color = blt::pick_foreground_color(point, 0);
        let mut the_char = ' ';
        if let Some(color) = renderable.background_color {
            bg_color = color.to_blt();
        }
        if let Some(color) = renderable.foreground_color {
            fg_color = color.to_blt();
        }
        if let Some(char) = renderable.char {
            the_char = char;
        }
        blt::with_colors(fg_color, bg_color, || blt::put_xy(x as i32, y as i32, the_char));
        trace!("Exiting Renderable::draw_entity_renderable().");
    }

    /// Indicates whether a pair of coordinates are in bounds of this map.
    pub fn get_tile(&self, x: usize, y: usize) -> &Tile {
        &self.tile_map[x][y]
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
    pub fn get_entities(&self, x: usize, y: usize) -> Option<HashSet<usize>> {
        if let Some(hashset) = self.spatial_hash.get(&(x, y)) {
            return Some(hashset.clone());
        }
        None
    }

    /// Gets entity IDs at a specific location.
    pub fn get_entities_around(&self, x: usize, y: usize) -> Vec<usize> {
        let mut result: Vec<usize> = vec![];
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == dy && dx == 0 {
                    continue;
                }
                let final_x = (x as i32 + dx) as usize;
                let final_y = (y as i32 + dy) as usize;
                if !self.is_in_bounds(final_x, final_y) {
                    continue;
                }
                if let Some(entities) = self.get_entities(final_x, final_y) {
                    result.extend(entities)
                }
            }
        }
        result
    }

}

impl fmt::Debug for Map0 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Map0")
    }
}


*/
