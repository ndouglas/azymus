use crate::component;
use component::renderable::Renderable;

/// The factory.
pub mod factory;

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

}
