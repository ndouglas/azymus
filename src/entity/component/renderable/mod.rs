use tcod::colors::*;

/// Indicates how the given object is rendered on a map.
#[derive(Clone, Copy, Debug)]
pub struct Renderable {
    /// The character used to render the object.
    pub char: Option<char>,
    /// The color used to render the object.
    pub foreground_color: Option<Color>,
    /// The color used to render the object.
    pub background_color: Option<Color>,
}
