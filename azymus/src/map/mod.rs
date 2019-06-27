use crate::component;
use component::position::Position;
use component::renderable::Renderable;

/// Map generators.
pub mod generator;
/// Map tiles.
pub mod tile;

/// The type of thing we use for map tiles.
/// tile, occupant, opaque, position, renderable
pub type MapTileType = (bool, bool, bool, Renderable);

/// The type of thing we use for maps.
pub type MapType = Vec<Vec<Vec<MapTileType>>>;

/// Map coordinate type.
pub type MapCoordinateType = (i32, i32);
