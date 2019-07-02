use tcod::console::*;
use crate::component;

/// The entity object that represents anything that functions in the game world.
#[derive(Clone, Debug)]
pub struct Entity {
    /// Indicates a position of the object within the game world.
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

    /// Move to a specific location.
    pub fn move_to(&mut self, x: i32, y: i32, z: i32) {
        trace!("Entering Entity::move_to() for entity {:?}.", self);
        if let Some(position) = self.position.as_mut() {
            position.x = x;
            position.y = y;
            position.z = z;
        }
        trace!("Exiting Entity::move_to().");
    }

}

/// Get a "player" entity.
pub fn get_player() -> Entity {
    use component::position::Position;
    use component::renderable::Factory;
    let mut player = Entity::new();
    player.position = Some(Position::default());
    player.renderable = Some(Factory::Player.create());
    player
}

/// Get an "NPC" entity.
pub fn get_npc() -> Entity {
    use component::position::Position;
    use component::renderable::Factory;
    let mut npc = Entity::new();
    npc.position = Some(Position::default());
    npc.renderable = Some(Factory::Npc.create());
    npc
}
