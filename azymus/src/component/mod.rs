/// Something that can act.
pub mod actor;
/// Something with the ability to act independently.
pub mod agent;
/// Something whose turn it is.
pub mod baton;
/// Something representing an entity's ability to perceive.
pub mod field_of_view;
/// Something that has a name.
pub mod name;
/// Something that is the sole occupant of its position.
pub mod occupant;
/// Something that blocks light.
pub mod opaque;
/// Something currently controlled by the player.
pub mod player;
/// Something we've encountered.
pub mod player_explored;
/// Something that can be positioned in the world.
pub mod position;
/// Something that can be drawn to a console.
pub mod renderable;
/// Something that is a tile in the map.
pub mod tile;
