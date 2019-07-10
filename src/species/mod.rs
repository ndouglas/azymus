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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    /// Moss.
    Moss,
    /// Moss Seed.
    MossSeed,
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
    /// Moss.
    Moss,
    /// Moss Seed.
    MossSeed,
}

/// Factory.
impl Factory {

    /// Create an entity of the specified type.
    pub fn create(&self) -> Entity {
        use Factory::*;
        match self {
            Orc => {
                let mut orc = Entity::new("Orc".to_string());
                self.apply(&mut orc);
                orc
            },
            Troll => {
                let mut troll = Entity::new("Troll".to_string());
                self.apply(&mut troll);
                troll
            },
            Goblin => {
                let mut goblin = Entity::new("Goblin".to_string());
                self.apply(&mut goblin);
                goblin
            },
            Kobold => {
                let mut kobold = Entity::new("Kobold".to_string());
                self.apply(&mut kobold);
                kobold
            },
            Chicken => {
                let mut chicken = Entity::new("Chicken".to_string());
                self.apply(&mut chicken);
                chicken
            },
            Mushroom => {
                let mut mushroom = Entity::new("Mushroom".to_string());
                self.apply(&mut mushroom);
                mushroom
            },
            Moss => {
                let mut moss = Entity::new("Moss".to_string());
                self.apply(&mut moss);
                moss
            },
            MossSeed => {
                let mut moss_seed = Entity::new("MossSeed".to_string());
                self.apply(&mut moss_seed);
                moss_seed
            },
            Human => {
                let mut human = Entity::new("Human".to_string());
                self.apply(&mut human);
                human
            },
        }
    }

    /// Create an entity of the specified type.
    pub fn apply(&self, entity: &mut Entity) {
        use Factory::*;
        match self {
            Orc => {
                entity.actor = Some(Actor {
                    time: 0,
                    speed: 11,
                });
                entity.body = Some(Body {
                    total_hit_points: 15,
                    current_hit_points: 15,
                });
                entity.agent = Some(Agent {
                    algorithm: AgentAlgorithm::ApproachAndFightPlayer,
                });
                entity.position = Some(Position::default());
                entity.renderable = Some(RenderableFactory::Orc.create());
                entity.blocks_movement = true;
                entity.species = Some(Species::Orc);
            },
            Troll => {
                entity.actor = Some(Actor {
                    time: 0,
                    speed: 9,
                });
                entity.body = Some(Body {
                    total_hit_points: 25,
                    current_hit_points: 25,
                });
                entity.agent = Some(Agent {
                    algorithm: AgentAlgorithm::ApproachAndFightPlayer,
                });
                entity.light_source = Some(LightSourceFactory::Random.create());
                entity.position = Some(Position::default());
                entity.renderable = Some(RenderableFactory::Troll.create());
                entity.blocks_movement = true;
                entity.species = Some(Species::Troll);
            },
            Goblin => {
                entity.actor = Some(Actor {
                    time: 0,
                    speed: 12,
                });
                entity.body = Some(Body {
                    total_hit_points: 5,
                    current_hit_points: 5,
                });
                entity.agent = Some(Agent {
                    algorithm: AgentAlgorithm::ApproachAndFightPlayer,
                });
                entity.light_source = None;
                entity.position = Some(Position::default());
                entity.renderable = Some(RenderableFactory::Goblin.create());
                entity.blocks_movement = true;
                entity.species = Some(Species::Goblin);
            },
            Kobold => {
                entity.actor = Some(Actor {
                    time: 0,
                    speed: 14,
                });
                entity.body = Some(Body {
                    total_hit_points: 7,
                    current_hit_points: 7,
                });
                entity.agent = Some(Agent {
                    algorithm: AgentAlgorithm::ApproachAndFightPlayer,
                });
                entity.light_source = None;
                entity.position = Some(Position::default());
                entity.renderable = Some(RenderableFactory::Kobold.create());
                entity.blocks_movement = true;
                entity.species = Some(Species::Kobold);
            },
            Chicken => {
                entity.actor = Some(Actor {
                    time: 0,
                    speed: 14,
                });
                entity.body = Some(Body {
                    total_hit_points: 2,
                    current_hit_points: 2,
                });
                entity.agent = Some(Agent {
                    algorithm: AgentAlgorithm::BeChicken,
                });
                entity.light_source = None;
                entity.position = Some(Position::default());
                entity.renderable = Some(RenderableFactory::Chicken.create());
                entity.blocks_movement = true;
                entity.species = Some(Species::Chicken);
            },
            Mushroom => {
                entity.actor = Some(Actor {
                    time: 0,
                    speed: 12,
                });
                entity.body = Some(Body {
                    total_hit_points: 2,
                    current_hit_points: 2,
                });
                entity.agent = Some(Agent {
                    algorithm: AgentAlgorithm::BeMushroom,
                });;
                entity.light_source = None;
                entity.position = Some(Position::default());
                entity.renderable = Some(RenderableFactory::Mushroom.create());
                entity.blocks_movement = false;
                entity.species = Some(Species::Mushroom);
            },
            Moss => {
                entity.actor = Some(Actor {
                    time: 0,
                    speed: 12,
                });
                entity.body = Some(Body {
                    total_hit_points: 2,
                    current_hit_points: 2,
                });
                entity.agent = Some(Agent {
                    algorithm: AgentAlgorithm::BeMoss,
                });
                entity.light_source = Some(LightSourceFactory::Moss.create());
                entity.position = Some(Position::default());
                entity.renderable = Some(RenderableFactory::Moss.create());
                entity.blocks_movement = false;
                entity.species = Some(Species::Moss);
            },
            MossSeed => {
                entity.actor = Some(Actor {
                    time: 0,
                    speed: 12,
                });
                entity.body = None;
                entity.agent = Some(Agent {
                    algorithm: AgentAlgorithm::BeMossSeed,
                });
                entity.light_source = None;
                entity.position = Some(Position::default());
                entity.renderable = None;
                entity.blocks_movement = false;
                entity.species = Some(Species::MossSeed);
            },
            Human => {
                entity.actor = Some(Actor {
                    time: 0,
                    speed: 12,
                });
                entity.body = Some(Body {
                    total_hit_points: 10,
                    current_hit_points: 10,
                });
                entity.agent = Some(Agent {
                    algorithm: AgentAlgorithm::ApproachPlayer,
                });
                entity.light_source = Some(LightSourceFactory::Torch.create());
                entity.position = Some(Position::default());
                entity.renderable = Some(RenderableFactory::Human.create());
                entity.blocks_movement = true;
                entity.species = Some(Species::Human);
            },
        }
    }

}

