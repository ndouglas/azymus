use tcod::colors::*;
use tcod::console::*;
use specs::{Component, VecStorage};

/// Something that can be drawn to a console.
#[derive(Clone, Component, Copy, Debug)]
#[storage(VecStorage)]
pub struct Renderable {

    /// The character used to render the object.
    pub char: Option<char>,

    /// The color used to render the object.
    pub foreground_color: Option<Color>,

    /// The color used to render the object.
    pub background_color: Option<Color>,

}

/// Extension for consoles.
pub trait RenderableTrait {

    /// Render some renderable thing.
    fn render_renderable(&mut self, x: i32, y: i32, renderable: &Renderable);

}

/// Extension for consoles.
impl RenderableTrait for Offscreen {

    /// Render some renderable thing.
    fn render_renderable(&mut self, x: i32, y: i32, renderable: &Renderable) {
        if let Some(color) = renderable.foreground_color {
            if let Some(char) = renderable.char {
                self.set_default_foreground(color);
                self.put_char(x, y, char, BackgroundFlag::None);
            }
        }
        if let Some(color) = renderable.background_color {
            self.set_char_background(x, y, color, BackgroundFlag::Set);
        }
    }

}
