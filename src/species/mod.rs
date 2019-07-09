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
    /// Goblin.
    Goblin,
    /// Kobold.
    Kobold,
    /// Chicken.
    Chicken,
    /// Mushroom.
    Mushroom,
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
    /// Goblin.
    Goblin,
    /// Kobold.
    Kobold,
    /// Chicken.
    Chicken,
    /// Mushroom.
    Mushroom,
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
                    algorithm: AgentAlgorithm::ApproachAndFightPlayer,
                });
                //orc.light_source = Some(LightSourceFactory::Random.create());
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
                    algorithm: AgentAlgorithm::ApproachAndFightPlayer,
                });
                troll.light_source = Some(LightSourceFactory::Random.create());
                troll.position = Some(Position::default());
                troll.renderable = Some(RenderableFactory::Troll.create());
                troll.blocks_movement = true;
                troll.species = Some(Species::Troll);
                troll
            },
            Goblin => {
                let mut goblin = Entity::new("Goblin".to_string());
                goblin.actor = Some(Actor {
                    time: 0,
                    speed: 9,
                });
                goblin.body = Some(Body {
                    total_hit_points: 15,
                    current_hit_points: 15,
                });
                goblin.agent = Some(Agent {
                    algorithm: AgentAlgorithm::ApproachAndFightPlayer,
                });
                goblin.light_source = None;
                goblin.position = Some(Position::default());
                goblin.renderable = Some(RenderableFactory::Goblin.create());
                goblin.blocks_movement = true;
                goblin.species = Some(Species::Goblin);
                goblin
            },
            Kobold => {
                let mut kobold = Entity::new("Kobold".to_string());
                kobold.actor = Some(Actor {
                    time: 0,
                    speed: 9,
                });
                kobold.body = Some(Body {
                    total_hit_points: 15,
                    current_hit_points: 15,
                });
                kobold.agent = Some(Agent {
                    algorithm: AgentAlgorithm::ApproachAndFightPlayer,
                });
                kobold.light_source = None;
                kobold.position = Some(Position::default());
                kobold.renderable = Some(RenderableFactory::Kobold.create());
                kobold.blocks_movement = true;
                kobold.species = Some(Species::Kobold);
                kobold
            },
            Chicken => {
                let mut chicken = Entity::new("Chicken".to_string());
                chicken.actor = Some(Actor {
                    time: 0,
                    speed: 9,
                });
                chicken.body = Some(Body {
                    total_hit_points: 15,
                    current_hit_points: 15,
                });
                chicken.agent = Some(Agent {
                    algorithm: AgentAlgorithm::ApproachPlayer,
                });
                chicken.light_source = None;
                chicken.position = Some(Position::default());
                chicken.renderable = Some(RenderableFactory::Chicken.create());
                chicken.blocks_movement = true;
                chicken.species = Some(Species::Chicken);
                chicken
            },
            Mushroom => {
                let mut mushroom = Entity::new("Mushroom".to_string());
                mushroom.actor = Some(Actor {
                    time: 0,
                    speed: 9,
                });
                mushroom.body = Some(Body {
                    total_hit_points: 15,
                    current_hit_points: 15,
                });
                mushroom.agent = None;
                mushroom.light_source = None;
                mushroom.position = Some(Position::default());
                mushroom.renderable = Some(RenderableFactory::Mushroom.create());
                mushroom.blocks_movement = false;
                mushroom.species = Some(Species::Mushroom);
                mushroom
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

