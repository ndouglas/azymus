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
use crate::tile;
use tile::Tile;
use tcod::map::Map as FovMap;
use crate::ui;
use ui::Ui;

/// The map generators.
pub mod generator;

/// The quad-tree for spatial trees.
pub mod quadtree;
use quadtree::QuadTreeRegion;
use quadtree::QuadTreePoint;

/// Our quad-tree type.
pub type QuadTreeType = NTree<QuadTreeRegion, QuadTreePoint>;

/// The map type.
pub type MapType = Vec<Vec<Tile>>;

/// The map object.
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

    /// Get a light source quadtree for the map.
    pub fn get_ls_tree(&self, game: &Game) -> QuadTreeType {
        let mut ls_tree: QuadTreeType = NTree::new(QuadTreeRegion {
            x: 0,
            y: 0,
            width: self.width as i32,
            height: self.height as i32,
        }, 4);
        for y in 0..self.height {
            for x in 0..self.width {
                let ids = self.get_entities(x, y)
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
                    //println!("({}, {}) -> {:?}", x, y, ls_vector);
                    if let Some(renderable) = &self.map[x][y].renderable {
                        self.draw_tile_renderable(x, y, &renderable, game, &ls_vector);
                    }
                    let mut occupant_found: bool = false;
                    for id in self.get_entities(x, y).iter().collect::<Vec<&usize>>() {
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
                    if let Some(renderable) = &self.map[x][y].renderable {
                        self.draw_tile_renderable(x, y, &renderable, game, &ls_vector);
                    }
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
            bg_color = color;
        }
        if let Some(color) = renderable.foreground_color {
            fg_color = color;
        }
        if let Some(char) = renderable.char {
            the_char = char;
        }
        for id in ls_vector {
            let entity = &game.entities[*id];
            if let Some(position) = entity.position {
                if let Some(light_source) = entity.light_source {
                    bg_color = light_source.transform_color_at(bg_color, position.x, position.y, x as i32, y as i32);
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
            bg_color = color;
        }
        if let Some(color) = renderable.foreground_color {
            fg_color = color;
        }
        if let Some(char) = renderable.char {
            the_char = char;
        }
        blt::with_colors(fg_color, bg_color, || blt::put_xy(x as i32, y as i32, the_char));
        trace!("Exiting Renderable::draw_entity_renderable().");
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

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Map")
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
