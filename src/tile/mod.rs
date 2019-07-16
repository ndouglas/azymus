use crate::component;
use component::renderable::{Renderable, Factory as RenderableFactory};

/// The tiles that form the map and structure of the game world.
#[derive(Clone, Debug)]
pub struct Tile {
    /// Indicates how the given object is rendered on a map.
    pub renderable: Renderable,
    /// Whether this object prevents movement.
    pub blocks_movement: bool,
    /// Whether this object is opaque.
    pub blocks_light: bool,
}

impl Tile {

    /// Create a floor tile.
    pub fn floor() -> Self {
        Tile {
            renderable: RenderableFactory::Floor.create(),
            blocks_movement: false,
            blocks_light: false,
        }
    }

    /// Create a wall tile.
    pub fn wall() -> Self {
        Tile {
            renderable: RenderableFactory::Wall.create(),
            blocks_movement: true,
            blocks_light: true,
        }
    }

}
