use std::fmt;
use tcod::map::Map as FovMap;
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
use crate::faction;
//use faction::Faction;
use faction::Standing as FactionStanding;
use crate::map;
use map::Map;
use crate::math;
use math::geometry::cell::{Cell, Cellular};
use crate::species;
use species::Species;

/// The entity object that represents anything that functions in the game world.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Entity {
    /// A unique (hopefully) ID for this entity.
    pub id: usize,
    /// The name of this entity.
    pub name: String,
    /// The species of this entity.
    pub species: Option<Species>,
    /// The factions of this entity.
    pub faction_standings: Option<Vec<FactionStanding>>,
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
    /// Indicates the cell of the object.
    pub cell: Cell,
    /// Indicates how the given object is rendered on a map.
    pub renderable: Option<Renderable>,
    /// Whether this object prevents movement.
    pub blocks_movement: bool,
}


impl Entity {

    /// Constructor.
    pub fn new(name: String) -> Self {
        trace!("Entering Entity::new().");
        Entity {
            id: std::usize::MAX,
            name: name,
            species: None,
            faction_standings: None,
            body: None,
            actor: None,
            agent: None,
            field_of_view: None,
            light_source: None,
            position: None,
            cell: Cell::default(),
            renderable: None,
            blocks_movement: false,
        }
    }

    /// Set properties from another entity.
    pub fn set(&mut self, entity: &Entity) {
        // Skip ID.
        self.name = entity.name.clone();
        self.species = entity.species;
        // Skip faction standings.
        self.body = entity.body;
        self.actor = entity.actor;
        self.agent = entity.agent;
        self.field_of_view = entity.field_of_view.clone();
        self.light_source = entity.light_source;
        // Skip position.
        self.renderable = entity.renderable.clone();
        self.blocks_movement = entity.blocks_movement;
    }

    /// If the entity would attack another entity.
    pub fn would_attack(&self, entity: &Entity) -> bool {
        use Species::*;
        if !entity.blocks_movement {
            return false;
        }
        match (self.species, entity.species) {
            (Some(Orc), Some(Troll)) => false,
            (Some(Troll), Some(Orc)) => false,
            (Some(Human), Some(Orc)) => true,
            (Some(Human), Some(Troll)) => true,
            (Some(Orc), Some(Human)) => true,
            (Some(Troll), Some(Human)) => true,
            (Some(Troll), Some(Troll)) => false,
            (Some(Orc), Some(Orc)) => false,
            (Some(Goblin), Some(Chicken)) => false,
            (Some(Kobold), Some(Chicken)) => false,
            (Some(Goblin), Some(Moss)) => false,
            (Some(Kobold), Some(Moss)) => false,
            (Some(Goblin), _ ) => true,
            (Some(Kobold), _ ) => true,
            (Some(Chicken), Some(Moss)) => true,
            (Some(Human), _) => true,
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

/// Implement cellular for this point.
impl Cellular for Entity {

    /// Create a cell from this object.
    fn as_cell(&self) -> Cell {
        if let Some(position) = self.position {
            Cell {
                x: position.x as usize,
                y: position.y as usize,
            }
        } else {
            Cell {
                x: 0,
                y: 0,
            }
        }
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
    player.species = Some(Species::Human);
    player.field_of_view = Some(FieldOfView::new(map.get_fov(), 12));
    if let Some(fov) = player.field_of_view.as_mut() {
        fov.light_walls = true;
    }
    player.light_source = Some(LightSourceFactory::Torch.create());
    player.position = Some(Position::default());
    player.renderable = Some(RenderableFactory::Player.create());
    player.blocks_movement = true;
    player
}

/// Format this object for user display.
impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "'{}' {}", self.name, self.cell)
    }
}
