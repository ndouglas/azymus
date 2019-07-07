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
