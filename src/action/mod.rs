use crate::command;
use command::CompassDirection;
use crate::game;
use game::Game;

const TIME_PER_TURN: i32 = 120;

/// Actions are processes that modify the game world.
#[derive(Clone, Copy, Debug)]
pub enum Action {
    /// Walk the entity to the specified direction.
    Walk(CompassDirection),
    /// Just wait, wasting a turn.
    Wait,
    /// Stall -- don't waste turn, but don't do anything.
    Stall,
}

/// Actions are processes that modify the game world.
impl Action {

    /// Get the cost of performing this action.
    pub fn get_cost(self, _id: usize, _game: &Game) -> i32 {
        trace!("Entering Action::get_cost().");
        use Action::*;
        match self {
            Stall => 0,
            _ => TIME_PER_TURN,
        }
    }

    /// Perform the action.
    pub fn execute(&self, id: usize, game: &mut Game) {
        trace!("Entering Action::execute().");
        use Action::*;
        match self {
            Walk(compass_direction) => {
                let entity = &mut game.entities[id];
                entity.move_to_direction(*compass_direction);
            },
            Wait => {
                let entity = &game.entities[id];
                println!("Entity {} elected to wait ({}).", entity.name, entity.actor.unwrap().time);
            },
            Stall => {
                let entity = &game.entities[id];
                println!("Entity {} elected to stall for time ({}).", entity.name, entity.actor.unwrap().time);
            },
        }
    }

}
