use tcod::console::*;
use crate::component;
use component::light_source::{LightSource, Factory as LightSourceFactory};
use component::position::Position;
use component::renderable::{Renderable, Factory as RenderableFactory};

/// The tiles that form the map and structure of the game world.
#[derive(Clone, Debug)]
pub struct Tile {
    /// Indicates that this tile has a light source.
    pub light_source: Option<LightSource>,
    /// Indicates a position of the object within the game world.
    pub position: Option<Position>,
    /// Indicates how the given object is rendered on a map.
    pub renderable: Option<Renderable>,
    /// Whether this object prevents movement.
    pub blocks_movement: bool,
    /// Whether this object is opaque.
    pub blocks_light: bool,
}

impl Tile {

    /// Constructor.
    pub fn new() -> Self {
        Tile {
            light_source: None,
            position: None,
            renderable: None,
            blocks_movement: false,
            blocks_light: false,
        }
    }

    /// Render this tile's renderable at the current position.
    pub fn draw(&self, console: &mut Console) {
        trace!("Entering Tile::draw() for tile {:?}.", self);
        if let Some(position) = self.position {
            if let Some(renderable) = &self.renderable {
                renderable.draw(position.x, position.y, console);
            }
        }
        trace!("Exiting Tile::draw().");
    }

    /// Render this tile's renderable with a simple FOV illumination.
    pub fn draw_in_fov(&self, console: &mut Console) {
        trace!("Entering Tile::draw_illuminated() for tile {:?}.", self);
        self.draw(console);
        trace!("Exiting Tile::draw_illuminated().");
    }

    /// Render this tile's renderable with a source of illumination.
    pub fn draw_lighted(&self, console: &mut Console, ls: &LightSource, lsx: i32, lsy: i32) {
        trace!("Entering Tile::draw_illuminated() for tile {:?}.", self);
        if let Some(position) = self.position {
            if let Some(renderable) = &self.renderable {
                if let Some(color) = renderable.background_color {
                    let transformed_color = ls.transform_color_at(
                        color,
                        lsx,
                        lsy,
                        position.x,
                        position.y
                    );
                    let renderable = renderable.with_background_color(Some(transformed_color));
                    renderable.draw(position.x, position.y, console);
                }
            }
        }
        trace!("Exiting Tile::draw_illuminated().");
    }

    /// Create a floor tile.
    pub fn floor(w: i64, x: i32, y: i32, z: i32) -> Self {
        Tile {
            light_source: None,
            position: Some(Position::new(w, x, y, z)),
            renderable: Some(RenderableFactory::Floor.create()),
            blocks_movement: false,
            blocks_light: false,
        }
    }

    /// Create a wall tile.
    pub fn wall(w: i64, x: i32, y: i32, z: i32) -> Self {
        Tile {
            light_source: None,
            position: Some(Position::new(w, x, y, z)),
            renderable: Some(RenderableFactory::Wall.create()),
            blocks_movement: true,
            blocks_light: true,
        }
    }

    /// Create a sconce tile.
    pub fn sconce(w: i64, x: i32, y: i32, z: i32) -> Self {
        Tile {
            light_source: Some(LightSourceFactory::Torch.create()),
            position: Some(Position::new(w, x, y, z)),
            renderable: Some(RenderableFactory::Floor.create()),
            blocks_movement: false,
            blocks_light: false,
        }
    }

}
