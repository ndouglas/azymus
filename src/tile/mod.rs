use tcod::console::*;
use crate::component;

/// The tiles that form the map and structure of the game world.
#[derive(Clone, Debug)]
pub struct Tile {
    /// Indicates a position of the object within the game world.
    pub position: Option<component::position::Position>,
    /// Indicates how the given object is rendered on a map.
    pub renderable: Option<component::renderable::Renderable>,
}

impl Tile {

    /// Constructor.
    pub fn new() -> Self {
        Tile {
            position: None,
            renderable: None,
        }
    }

    /// Render this tile's renderable at the current position.
    pub fn draw(&self, console: &mut Console) {
        trace!("Entering Tile::draw() for tile {:?}.", self);
        if let Some(position) = self.position {
            if let Some(renderable) = self.renderable {
                if let Some(color) = renderable.foreground_color {
                    if let Some(char) = renderable.char {
                        console.set_default_foreground(color);
                        console.put_char(position.x, position.y, char, BackgroundFlag::None);
                    }
                }
                if let Some(color) = renderable.background_color {
                    console.set_char_background(position.x, position.y, color, BackgroundFlag::Set);
                }
            }
        }
        trace!("Exiting Tile::draw().");
    }

}
