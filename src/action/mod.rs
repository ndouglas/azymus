use crate::command;
use command::CompassDirection;
use crate::effect;
use effect::Effect;
use crate::game;
use game::Game;
use crate::species;
use species::Factory as SpeciesFactory;

const TIME_PER_TURN: i32 = 120;

/// Actions are processes that modify the game world.
#[derive(Clone, Copy, Debug)]
pub enum Action {
    /// Walk the entity to the specified direction.
    Walk(CompassDirection),
    /// Attack (melee) something in the specified direction.
    MeleeAttack(CompassDirection),
    /// Just wait, wasting a turn.
    Wait,
    /// Stall -- don't waste turn, but don't do anything.
    Stall,
    /// Moss: Bloom.
    MossBloom,
    /// Moss: Seed.
    MossSeed(CompassDirection),
    /// Moss: Die off.
    MossDie,
}

/// Actions are processes that modify the game world.
impl Action {

    /// Get the cost of performing this action.
    pub fn get_cost(self, _id: usize, _game: &Game) -> i32 {
        trace!("Entering Action::get_cost().");
        use Action::*;
        match self {
            Walk(_) => TIME_PER_TURN,
            MeleeAttack(_) => TIME_PER_TURN,
            Wait => TIME_PER_TURN,
            Stall => 0,
            MossBloom => TIME_PER_TURN * 4,
            MossSeed(_) => TIME_PER_TURN * 2,
            MossDie => 1,
        }
    }

    /// Perform the action.
    pub fn execute(&self, id: usize, game: &mut Game) -> Option<Effect> {
        trace!("Entering Action::execute().");
        use Action::*;
        match self {
            Walk(compass_direction) => {
                let entity = &game.entities[id];
                if let Some(position1) = &entity.position {
                    let position2 = position1.to_direction(*compass_direction);
                    debug!("Entity {} elected to move ({:?}).", entity.name, compass_direction);
                    return Some(Effect::MoveEntity(position1.clone(), position2));
                }
                None
            },
            MeleeAttack(compass_direction) => {
                let entity = &game.entities[id];
                if let Some(entity_position) = &entity.position {
                    let target_position = entity_position.to_direction(*compass_direction);
                    debug!("Entity {} elected to attack ({:?}).", entity.name, compass_direction);
                    if let Some(target_entity) = &game.get_entities(target_position.x, target_position.y)
                        .iter()
                        .filter(|x| x.body.is_some()).nth(0) {
                            println!("Entity {} ({}, {}) attacks target entity {} ({}, {})!", entity.name, entity_position.x, entity_position.y, target_entity.name, target_position.x, target_position.y);
                            return Some(Effect::DamageEntityBody(target_entity.id, 7));
                    }
                }
                None
            },
            Wait => {
                let entity = &game.entities[id];
                debug!("Entity {} elected to wait ({}).", entity.name, entity.actor.unwrap().time);
                None
            },
            Stall => {
                let entity = &game.entities[id];
                debug!("Entity {} elected to stall for time ({}).", entity.name, entity.actor.unwrap().time);
                None
            },
            MossBloom => {
                let entity = &game.entities[id];
                if let Some(position) = &entity.position {
                    debug!("Entity {} ({}, {}) is following the moss-bloom rule.", entity.name, position.x, position.y);
                    return Some(Effect::ChangeEntitySpecies(SpeciesFactory::Moss));
                }
                None
            },
            MossSeed(compass_direction) => {
                let entity = &game.entities[id];
                debug!("Entity {} is following moss lifecycle rules!", entity.name);
                if let Some(entity_position) = &entity.position {
                    let target_position = entity_position.to_direction(*compass_direction);
                    return Some(Effect::CreateEntity(target_position, SpeciesFactory::MossSeed));
                }
                None
            },
            MossDie => {
                Some(Effect::RemoveEntity)
            }
        }
    }

}
