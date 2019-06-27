use crate::component;
use component::renderable::Renderable;
use tcod::colors::Color;

/// Unlit floor color.
pub const FLOOR_UNLIT_COLOR: Color = Color {
    r: 50,
    g: 50,
    b: 150,
};

/// Unlit floor tile.
pub const FLOOR_UNLIT: Renderable = Renderable {
    foreground_color: None,
    background_color: Some(FLOOR_UNLIT_COLOR),
    char: None,
};

/// Lit floor color.
pub const FLOOR_LIT_COLOR: Color = Color {
    r: 200,
    g: 180,
    b: 50,
};

/// Lit floor tile.
pub const FLOOR_LIT: Renderable = Renderable {
    foreground_color: None,
    background_color: Some(FLOOR_LIT_COLOR),
    char: None,
};

/// Unlit wall color.
pub const WALL_UNLIT_COLOR: Color = Color {
    r: 0,
    g: 0,
    b: 100
};
/// Unlit wall tile.
pub const WALL_UNLIT: Renderable = Renderable {
    foreground_color: None,
    background_color: Some(WALL_UNLIT_COLOR),
    char: None,
};

/// Lit wall color.
pub const WALL_LIT_COLOR: Color = Color {
    r: 130,
    g: 110,
    b: 50,
};

/// Lit wall tile.
pub const WALL_LIT: Renderable = Renderable {
    foreground_color: None,
    background_color: Some(WALL_LIT_COLOR),
    char: None,
};

