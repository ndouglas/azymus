use crate::agent;
use agent::{Agent, Algorithm as AgentAlgorithm};
use crate::body;
use body::Body;
use crate::component;
use component::actor::Actor;
use component::light_source::Factory as LightSourceFactory;
use component::position::Position;
use component::renderable::Factory as RenderableFactory;
use crate::entity;
use entity::Entity;

/// The species that we support.
#[derive(Clone, Copy, Debug)]
pub enum Species {
    /// Human.
    Human,
    /// Orc.
    Orc,
    /// Troll.
    Troll,
}

/// Factory.
#[derive(Clone, Copy, Debug)]
pub enum Factory {
    /// Human.
    Human,
    /// Orc.
    Orc,
    /// Troll.
    Troll,
}

/// Factory.
impl Factory {

    /// Create an entity of the specified type.
    pub fn create(&self) -> Entity {
        use Factory::*;
        match self {
            Orc => {
                let mut orc = Entity::new("Orc".to_string());
                orc.actor = Some(Actor {
                    time: 0,
                    speed: 11,
                });
                orc.body = Some(Body {
                    total_hit_points: 10,
                    current_hit_points: 10,
                });
                orc.agent = Some(Agent {
                    algorithm: AgentAlgorithm::ApproachPlayer,
                });
                orc.light_source = Some(LightSourceFactory::Candle.create());
                orc.position = Some(Position::default());
                orc.renderable = Some(RenderableFactory::Orc.create());
                orc.blocks_movement = true;
                orc.species = Some(Species::Orc);
                orc
            },
            Troll => {
                let mut troll = Entity::new("Troll".to_string());
                troll.actor = Some(Actor {
                    time: 0,
                    speed: 9,
                });
                troll.body = Some(Body {
                    total_hit_points: 15,
                    current_hit_points: 15,
                });
                troll.agent = Some(Agent {
                    algorithm: AgentAlgorithm::ApproachPlayer,
                });
                troll.light_source = None;
                troll.position = Some(Position::default());
                troll.renderable = Some(RenderableFactory::Troll.create());
                troll.blocks_movement = true;
                troll.species = Some(Species::Troll);
                troll
            },
            Human => {
                let mut human = Entity::new("Human".to_string());
                human.actor = Some(Actor {
                    time: 0,
                    speed: 12,
                });
                human.body = Some(Body {
                    total_hit_points: 6,
                    current_hit_points: 6,
                });
                human.agent = Some(Agent {
                    algorithm: AgentAlgorithm::ApproachPlayer,
                });
                human.light_source = Some(LightSourceFactory::Torch.create());
                human.position = Some(Position::default());
                human.renderable = Some(RenderableFactory::Human.create());
                human.blocks_movement = true;
                human.species = Some(Species::Human);
                human
            },
        }
    }

}

