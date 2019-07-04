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
}

/// Actions are processes that modify the game world.
impl Action {

    /// Get the cost of performing this action.
    pub fn get_cost(&self, _id: usize, _game: &Game) -> i32 {
        trace!("Entering Action::get_cost().");
        TIME_PER_TURN
    }

    /// Perform the action.
    pub fn execute(&self, id: usize, game: &mut Game) {
        trace!("Entering Action::execute().");
        use Action::*;
        match self {
            Walk(compass_direction) => {
                let entity = &mut game.entities[id];
                if let Some(position) = entity.position {
                    use CompassDirection::*;
                    match compass_direction {
                        North => entity.move_to(position.x, position.y - 1, position.z),
                        South => entity.move_to(position.x, position.y + 1, position.z),
                        West => entity.move_to(position.x - 1, position.y, position.z),
                        East => entity.move_to(position.x + 1, position.y, position.z),
                    }
                }
            },
            Wait => {
                let entity = &game.entities[id];
                println!("Entity {} elected to wait ({}).", entity.name, entity.actor.unwrap().time);
            },
        }
    }

}
