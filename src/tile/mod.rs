use tcod::colors::*;
use tcod::console::*;
use crate::component;
use component::position::Position;
use component::renderable::{Renderable, Factory as RenderableFactory};

/// The tiles that form the map and structure of the game world.
#[derive(Clone, Debug)]
pub struct Tile {
    /// Indicates a position of the object within the game world.
    pub position: Option<Position>,
    /// Indicates how the given object is rendered on a map.
    pub renderable: Option<Renderable>,
    /// Whether this object prevents movement.
    pub blocks_movement: bool,
    /// Whether this object is opaque.
    pub blocks_light: bool,
    /// Ambient light level.
    pub light_level: i8,
}

impl Tile {

    /// Constructor.
    pub fn new() -> Self {
        Tile {
            position: None,
            renderable: None,
            blocks_movement: false,
            blocks_light: false,
            light_level: 0,
        }
    }

    /// Render this tile's renderable at the current position.
    pub fn draw(&self, console: &mut Console) {
        trace!("Entering Tile::draw() for tile {:?}.", self);
        if let Some(position) = self.position {
            if let Some(renderable) = self.renderable {
                renderable.draw(position.x, position.y, console);
            }
        }
        trace!("Exiting Tile::draw().");
    }

    /// Render this tile's renderable with a simple FOV illumination.
    pub fn draw_in_fov(&self, console: &mut Console) {
        trace!("Entering Tile::draw_illuminated() for tile {:?}.", self);
        if let Some(position) = self.position {
            if let Some(Renderable {
                background_color: Some(Color {
                    r,
                    g,
                    b,
                }),
                ..
            }) = self.renderable {
                let renderable = Renderable {
                    char: self.renderable.unwrap().char,
                    background_color: Some(Color {
                        r: b,
                        g: g,
                        b: r,
                    }),
                    foreground_color: self.renderable.unwrap().foreground_color,
                };
                renderable.draw(position.x, position.y, console);
            }
        }
        trace!("Exiting Tile::draw_illuminated().");
    }

    /// Render this tile's renderable with a source of illumination.
    pub fn draw_lighted(&self, console: &mut Console, intensity: i32, color: Color) {
        trace!("Entering Tile::draw_illuminated() for tile {:?}.", self);
        if let Some(position) = self.position {
            if let Some(Renderable {
                background_color: Some(Color {
                    r,
                    g,
                    b,
                }),
                ..
            }) = self.renderable {
                let multiplier = intensity as f64 / 256 as f64;
                let new_r = (r as f64 + ((color.r - r) as f64 * multiplier)) as u8;
                let new_g = (g as f64 + ((color.g - g) as f64 * multiplier)) as u8;
                let new_b = (b as f64 + ((color.b - b) as f64 * multiplier)) as u8;
                let renderable = Renderable {
                    char: self.renderable.unwrap().char,
                    background_color: Some(Color {
                        r: new_r,
                        g: new_g,
                        b: new_b,
                    }),
                    foreground_color: self.renderable.unwrap().foreground_color,
                };
                renderable.draw(position.x, position.y, console);
            }
        }
        trace!("Exiting Tile::draw_illuminated().");
    }

    /// Create a floor tile.
    pub fn floor(w: i64, x: i32, y: i32, z: i32) -> Self {
        Tile {
            position: Some(Position::new(w, x, y, z)),
            renderable: Some(RenderableFactory::Floor.create()),
            blocks_movement: false,
            blocks_light: false,
            light_level: 0,
        }
    }

    /// Create a wall tile.
    pub fn wall(w: i64, x: i32, y: i32, z: i32) -> Self {
        Tile {
            position: Some(Position::new(w, x, y, z)),
            renderable: Some(RenderableFactory::Wall.create()),
            blocks_movement: true,
            blocks_light: true,
            light_level: 0,
        }
    }

}
