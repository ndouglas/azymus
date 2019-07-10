use crate::combat;
use combat::attack;
use crate::command;
use command::CompassDirection;
use crate::effect;
use effect::Effect;
use crate::game;
use game::Game;

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
    /// Moss lifecycle.
    MossLifecycle,
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
            MossLifecycle => TIME_PER_TURN * 8,
        }
    }

    /// Perform the action.
    pub fn execute(&self, id: usize, game: &mut Game) {
        trace!("Entering Action::execute().");
        use Action::*;
        match self {
            Walk(compass_direction) => {
                let entity = &game.entities[id];
                if let Some(position1) = &entity.position {
                    let position2 = position1.to_direction(*compass_direction);
                    debug!("Entity {} elected to move ({:?}).", entity.name, compass_direction);
                    Effect::MoveEntity(position1.clone(), position2).execute(id, game);
                }
            },
            MeleeAttack(compass_direction) => {
                let entity = &game.entities[id];
                if let Some(entity_position) = &entity.position {
                    let target_position = entity_position.to_direction(*compass_direction);
                    debug!("Entity {} elected to attack ({:?}).", entity.name, compass_direction);
                    if let Some(target_entity) = &game.get_entities(target_position.x, target_position.y).iter().filter(|x| x.body.is_some()).nth(0) {
                        println!("Entity {} ({}, {}) attacks target entity {} ({}, {})!", entity.name, entity_position.x, entity_position.y, target_entity.name, target_position.x, target_position.y);
                        attack(id, target_entity.id, game);
                    }
                }
            },
            Wait => {
                let entity = &game.entities[id];
                debug!("Entity {} elected to wait ({}).", entity.name, entity.actor.unwrap().time);
            },
            Stall => {
                let entity = &game.entities[id];
                debug!("Entity {} elected to stall for time ({}).", entity.name, entity.actor.unwrap().time);
            },
            MossLifecycle => {
                let entity = &game.entities[id];
                debug!("Entity {} is following moss lifecycle rules!", entity.name);
                let _map = &game.map;
                if let Some(_position) = & entity.position {
                    let _neighbors: i32 = 0;
                }
            },
        }
    }

}
