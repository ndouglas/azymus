use crate::effect;
use effect::Effect;
use crate::game;
use game::Game;
use crate::math;
use math::geometry::compass::Direction as CompassDirection;
use math::geometry::rectangle::Rectangular;
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
            MossBloom => TIME_PER_TURN * 14,
            MossSeed(_) => TIME_PER_TURN * 20,
            MossDie => 1,
        }
    }

    /// Perform the action.
    pub fn execute(&self, id: usize, game: &mut Game) -> Option<Effect> {
        trace!("Entering Action::execute().");
        use Action::*;
        match self {
            Walk(compass_direction) => {
                let entity = &game.get_entity(id);
                let cell = &entity.cell;
                if let Some(cell2) = cell.to_direction(compass_direction, &game.map.as_rectangle()) {
                    debug!("{} elected to move ({:?}).", entity, compass_direction);
                    return Some(Effect::MoveEntity(*cell, cell2));
                }
                None
            },
            MeleeAttack(compass_direction) => {
                let entity = &game.get_entity(id);
                let cell = &entity.cell;
                if let Some(target_cell) = cell.to_direction(compass_direction, &game.map.as_rectangle()) {
                    debug!("Entity {} elected to attack ({:?}).", entity.name, compass_direction);
                    if let Some(target_entity) = &game.get_entities(target_cell.x as i32, target_cell.y as i32)
                        .iter()
                        .filter(|x| x.body.is_some()).nth(0) {
                            println!("Entity {} attacks target entity {}!", entity, target_entity);
                            return Some(Effect::DamageEntityBody(target_entity.id, 7));
                    }
                }
                None
            },
            Wait => {
                let entity = &game.get_entity(id);
                debug!("Entity {} elected to wait ({}).", entity.name, entity.actor.unwrap().time);
                None
            },
            Stall => {
                let entity = &game.get_entity(id);
                debug!("Entity {} elected to stall for time ({}).", entity.name, entity.actor.unwrap().time);
                None
            },
            MossBloom => {
                let entity = &game.get_entity(id);
                let cell = &entity.cell;
                debug!("{} is following the moss-bloom rule.", entity);
                Some(Effect::ChangeEntitySpecies(SpeciesFactory::Moss))
            },
            MossSeed(compass_direction) => {
                let entity = &game.get_entity(id);
                debug!("Entity {} is following moss lifecycle rules!", entity.name);
                let entity_cell = &entity.cell;
                if let Some(target_cell) = entity_cell.to_direction(compass_direction, &game.map.as_rectangle()) {
                    return Some(Effect::CreateEntity(target_cell, SpeciesFactory::MossSeed));
                }
                None
            },
            MossDie => {
                Some(Effect::RemoveEntity)
            }
        }
    }

}
