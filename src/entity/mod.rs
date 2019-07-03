use tcod::console::*;
use crate::component;
use component::field_of_view::FieldOfView;
use component::light_source::{LightSource, Factory as LightSourceFactory};
use component::position::Position;
use component::renderable::{Renderable, Factory as RenderableFactory};
use crate::map;
use map::Map;

/// The entity object that represents anything that functions in the game world.
#[derive(Clone, Debug)]
pub struct Entity {
    /// Indicates the object's ability to perceive the world around it.
    pub field_of_view: Option<FieldOfView>,
    /// A light source attached to or possessed by this entity.
    pub light_source: Option<LightSource>,
    /// Indicates a position of the object within the game world.
    pub position: Option<Position>,
    /// Indicates how the given object is rendered on a map.
    pub renderable: Option<Renderable>,
}

impl Entity {

    /// Constructor.
    pub fn new() -> Self {
        Entity {
            field_of_view: None,
            light_source: None,
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
        if let Some(fov) = &mut self.field_of_view {
            fov.update(x, y);
        }
        trace!("Exiting Entity::move_to().");
    }

}

/// Get a "player" entity.
pub fn get_player(map: &Map) -> Entity {
    let mut player = Entity::new();
    player.field_of_view = Some(FieldOfView::new(map.get_fov(), 10));
    player.light_source = Some(LightSourceFactory::Torch.create());
    player.position = Some(Position::default());
    player.renderable = Some(RenderableFactory::Player.create());
    player
}

/// Get an "NPC" entity.
pub fn get_npc(map: &Map) -> Entity {
    let mut npc = Entity::new();
    npc.field_of_view = Some(FieldOfView::new(map.get_fov(), 8));
    npc.light_source = Some(LightSourceFactory::Torch.create());
    npc.position = Some(Position::default());
    npc.renderable = Some(RenderableFactory::Npc.create());
    npc
}
