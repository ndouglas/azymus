use tcod::map::Map as FovMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::agent;
use agent::Agent;
use crate::body;
use body::Body;
use crate::component;
use component::actor::Actor;
use component::field_of_view::FieldOfView;
use component::light_source::{LightSource, Factory as LightSourceFactory};
use component::position::Position;
use component::renderable::{Renderable, Factory as RenderableFactory};
use crate::map;
use map::Map;
use crate::species;
use species::Species;

/// The entity object that represents anything that functions in the game world.
#[derive(Clone, Debug)]
pub struct Entity {
    /// A unique (hopefully) ID for this entity.
    pub id: usize,
    /// The name of this entity.
    pub name: String,
    /// The species of this entity.
    pub species: Option<Species>,
    /// The body of this entity.
    pub body: Option<Body>,
    /// Something that gets dispensed time and has an opportunity to act.
    pub actor: Option<Actor>,
    /// Something that can act autonomously.
    pub agent: Option<Agent>,
    /// Indicates the object's ability to perceive the world around it.
    pub field_of_view: Option<FieldOfView>,
    /// A light source attached to or possessed by this entity.
    pub light_source: Option<LightSource>,
    /// Indicates a position of the object within the game world.
    pub position: Option<Position>,
    /// Indicates how the given object is rendered on a map.
    pub renderable: Option<Renderable>,
    /// Whether this object prevents movement.
    pub blocks_movement: bool,
}


impl Entity {

    /// Constructor.
    pub fn new(name: String) -> Self {
        trace!("Entering Entity::new().");
        static ID: AtomicUsize = AtomicUsize::new(0);
        let id = ID.load(Ordering::SeqCst);
        ID.fetch_add(1, Ordering::SeqCst);
        Entity {
            id: id,
            name: name,
            species: None,
            body: None,
            actor: None,
            agent: None,
            field_of_view: None,
            light_source: None,
            position: None,
            renderable: None,
            blocks_movement: false,
        }
    }

    /// If the entity would attack another entity.
    pub fn would_attack(&self, entity: &Entity) -> bool {
        use Species::*;
        match (self.species, entity.species) {
            (Some(Orc), Some(Troll)) => false,
            (Some(Troll), Some(Orc)) => false,
            (Some(Human), Some(Orc)) => true,
            (Some(Human), Some(Troll)) => true,
            (Some(Orc), Some(Human)) => true,
            (Some(Troll), Some(Human)) => true,
            (Some(Troll), Some(Troll)) => false,
            (Some(Orc), Some(Orc)) => false,
            (Some(Human), Some(Human)) => false,
            (Some(Goblin), _ ) => true,
            (Some(Kobold), _ ) => true,
            (_, Some(Goblin)) => true,
            (_, Some(Kobold)) => true,
            (_, _) => false,
        }
    }

    /// If this entity is in the FOV.
    pub fn is_in_fov(&self, fov: &FovMap) -> bool {
        if let Some(position) = &self.position {
            return fov.is_in_fov(position.x, position.y);
        }
        false
    }

}

/// Get a "player" entity.
pub fn get_player(map: &Map) -> Entity {
    trace!("Entering get_player().");
    let mut player = Entity::new("Player".to_string());
    player.actor = Some(Actor {
        time: 0,
        speed: 12,
    });
    player.body = Some(Body {
        total_hit_points: 32767,
        current_hit_points: 32767,
    });
    player.field_of_view = Some(FieldOfView::new(map.get_fov(), 12));
    player.light_source = Some(LightSourceFactory::Torch.create());
    player.position = Some(Position::default());
    player.renderable = Some(RenderableFactory::Player.create());
    player.blocks_movement = true;
    player.species = Some(Species::Human);
    player
}
