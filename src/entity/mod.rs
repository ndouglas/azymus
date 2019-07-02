use tcod::console::*;

/// Components of the entity object.
pub mod component;

/// The entity object that represents anything that functions in the game world.
#[derive(Clone, Debug)]
pub struct Entity {
    /// Indicates a position of the entity within the game world.
    pub position: Option<component::position::Position>,
    /// Indicates how the given object is rendered on a map.
    pub renderable: Option<component::renderable::Renderable>,
}

impl Entity {

    /// Constructor.
    pub fn new() -> Self {
        Entity {
            position: None,
            renderable: None,
        }
    }

    /// Render this entity's renderable at the current position.
    pub fn draw(&self, console: &mut Console) {
        trace!("Entering Entity::draw() for entity {:?}.", self);
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
        trace!("Exiting Entity::draw().");
    }

}
