use crate::action;
use action::Action;
use crate::component;
use component::position::Position;
use crate::game;
use game::Game;

/// Compass directions.
#[derive(Clone, Copy, Debug)]
pub enum CompassDirection {
    /// North.
    North,
    /// Northeast.
    Northeast,
    /// Northwest.
    Northwest,
    /// South.
    South,
    /// Southeast.
    Southeast,
    /// Southwest.
    Southwest,
    /// East.
    East,
    /// West.
    West,
}

/// Actions are processes that modify the game world.
#[derive(Clone, Copy, Debug)]
pub enum Command {
    /// Walk the entity argument in the specified direction.
    Walk(CompassDirection),
    /// Attack (melee) something in the specified direction.
    MeleeAttack(CompassDirection),
    /// Just wait, wasting a turn.
    Wait,
    /// Stall -- don't waste turn, but don't do anything.
    Stall,
}

/// Actions are processes that modify the game world.
impl Command {

    /// Retrieve the action for this command.
    pub fn get_default_action(self) -> Option<Action> {
        trace!("Entering Command::get_default_action().");
        use Command::*;
        match self {
            Walk(compass_direction) => {
                Some(Action::Walk(compass_direction))
            },
            MeleeAttack(compass_direction) => {
                Some(Action::MeleeAttack(compass_direction))
            }
            Wait => {
                Some(Action::Wait)
            },
            Stall => {
                Some(Action::Stall)
            },
        }
    }

    /// List the rules for this command.
    pub fn get_rules(self, id: usize, game: &Game) -> Vec<CommandRule> {
        trace!("Entering Command::get_rules().");
        use Command::*;
        use CommandRule::*;
        match self {
            Walk(compass_direction) => {
                let entity = &game.entities[id];
                if let Some(position1) = entity.position {
                    let position2 = position1.to_direction(compass_direction);
                    vec![
                        PositionsAreAdjacent(position1, position2),
                        PositionIsNotOutOfBounds(position2),
                        TileAtPositionDoesNotBlockMovement(position2),
                        NothingAtPositionBlocksMovement(position2),
                        NothingAtPositionIsValidMeleeAttackTarget(position2),
                    ]
                } else {
                    vec![
                        Deny("Entity has no starting position!".to_string()),
                    ]
                }
            },
            MeleeAttack(compass_direction) => {
                let entity = &game.entities[id];
                if let Some(position1) = entity.position {
                    let position2 = position1.to_direction(compass_direction);
                    vec![
                        PositionsAreAdjacent(position1, position2),
                        PositionIsNotOutOfBounds(position2),
                        SomethingAtPositionIsValidMeleeAttackTarget(position2),
                    ]
                } else {
                    vec![
                        Deny("Entity has no starting position!".to_string()),
                    ]
                }
            },
            Wait => {
                vec![
                    Permit,
                ]
            },
            Stall => {
                vec![
                    Permit,
                ]
            },
        }
    }

    /// Check the rules for this command.
    pub fn check_rules(self, id: usize, game: &Game) -> CommandRuleResult {
        trace!("Entering Command:check_rules().");
        use CommandRuleResult::*;
        for rule in self.get_rules(id, game) {
            match rule.evaluate(id, game) {
                Permitted => {
                    return Permitted;
                },
                Neutral => {
                    continue;
                },
                Denied(string) => {
                    return Denied(string);
                },
                Substituted(command) => {
                    return command.check_rules(id, game);
                }
            }
        }
        return Neutral;
    }

    /// Retrieve the final action for this command.
    pub fn get_final_action(self, id: usize, game: &Game) -> Option<Action> {
        trace!("Entering Command::get_final_action().");
        use CommandRuleResult::*;
        match self.check_rules(id, game) {
            Permitted => self.get_default_action(),
            Neutral => self.get_default_action(),
            Denied(_string) => {
                if id == game.player_id {
                    Some(Action::Stall)
                } else {
                    Some(Action::Wait)
                }
            },
            Substituted(command) => command.get_final_action(id, game),
        }
    }

    /// Get the cost for the anticipated action.
    pub fn get_cost(self, id: usize, game: &Game, action: Action) -> i32 {
        trace!("Entering Command::get_cost().");
        action.get_cost(id, game)
    }

    /// Perform the action.
    pub fn execute(self, id: usize, game: &mut Game) {
        trace!("Entering Command::execute().");
        let mut cost = Action::Wait.get_cost(id, game);
        if let Some(action) = self.get_final_action(id, game) {
            cost = action.get_cost(id, game);
            action.execute(id, game);
        };
        if let Some(actor) = game.entities[id].actor.as_mut() {
            actor.time -= cost;
        };
        trace!("Exiting Command::execute().");
    }

}

/// Different ways in which a rule might affect the intended action.
#[derive(Clone, Debug)]
pub enum CommandRuleResult {
    /// The rule permitted this action to proceed immediately without further checks.  (E.g. in god mode)
    Permitted,
    /// The rule gave no opinion either way on this attempt.
    /// This should be the most common result.
    Neutral,
    /// The attempted command failed; a message was provided.
    Denied(String),
    /// The attempted command was replaced with another.
    /// (E.g. "moving" to a tile with a hostile occupant -> attack)
    Substituted(Command),
}

/// Rules that govern the translation of commands into actions.
///
/// A Command Rule takes the Command and its context and indicates
/// whether it is permissible for the command to continue.
#[derive(Clone, Debug)]
pub enum CommandRule {
    /// Permits the command.
    Permit,
    /// Substitutes a command.
    Substitute(Command),
    /// Denies the command with the specified message.
    Deny(String),
    /// Position is not out of bounds.
    PositionIsNotOutOfBounds(Position),
    /// Position does not block movement.
    TileAtPositionDoesNotBlockMovement(Position),
    /// The positions are adjacent.
    PositionsAreAdjacent(Position, Position),
    /// No entity at the location blocks movement.
    NothingAtPositionBlocksMovement(Position),
    /// No entity at the location can be attacked.
    NothingAtPositionIsValidMeleeAttackTarget(Position),
    /// Some entity at the location can be attacked.
    SomethingAtPositionIsValidMeleeAttackTarget(Position),
}


/// Rules that govern the translation of commands into actions.
impl CommandRule {

    /// Evaluates the given rule with the specified context.
    pub fn evaluate(self, id: usize, game: &Game) -> CommandRuleResult {
        trace!("Entering CommandRule::evaluate().");
        use CommandRuleResult::*;
        use CommandRule::*;
        match self {
            Permit => Permitted,
            Deny(string) => Denied(string),
            Substitute(command) => Substituted(command),
            PositionIsNotOutOfBounds(position) => {
                trace!("Entering rule {:?}.", PositionIsNotOutOfBounds(position));
                let map = &game.map;
                if !map.is_in_bounds(position.x, position.y) {
                    debug!("Position {:?} is not in bounds of the map.", position);
                    return Denied("Requested an out-of-bounds position.".to_string());
                }
                Neutral
            },
            TileAtPositionDoesNotBlockMovement(position) => {
                trace!("Entering rule {:?}.", TileAtPositionDoesNotBlockMovement(position));
                let map = &game.map;
                if map.get_tile(position.x, position.y).blocks_movement {
                    return Denied("The destination position contains a tile that blocks movement.".to_string());
                }
                Neutral
            },
            PositionsAreAdjacent(position1, position2) => {
                trace!("Entering rule {:?}.", PositionsAreAdjacent(position1, position2));
                if (position1.x - position2.x).abs() > 1 || (position1.y - position2.y).abs() > 1 {
                    return Denied("The destination position is not adjacent to the original position.".to_string());
                }
                Neutral
            },
            NothingAtPositionBlocksMovement(position) => {
                trace!("Entering rule {:?}.", NothingAtPositionBlocksMovement(position));
                let occupants = &game.get_entities(position.x, position.y);
                for occupant in occupants {
                    if occupant.blocks_movement {
                        return Denied("The destination position contains an entity that blocks movement.".to_string());
                    }
                }
                Neutral
            },
            SomethingAtPositionIsValidMeleeAttackTarget(position) => {
                trace!("Entering rule {:?}.", SomethingAtPositionIsValidMeleeAttackTarget(position));
                let entity = &game.entities[id];
                let occupants = &game.get_entities(position.x, position.y);
                for occupant in occupants {
                    if entity.would_attack(occupant) {
                        return Neutral;
                    }
                }
                Substituted(Command::Walk(entity.position.unwrap().direction_to(position).unwrap()))
            },
            NothingAtPositionIsValidMeleeAttackTarget(position) => {
                trace!("Entering rule {:?}.", NothingAtPositionIsValidMeleeAttackTarget(position));
                let entity = &game.entities[id];
                let occupants = &game.get_entities(position.x, position.y);
                for occupant in occupants {
                    if entity.would_attack(occupant) {
                        return Substituted(Command::MeleeAttack(entity.position.unwrap().direction_to(position).unwrap()));
                    }
                }
                Neutral
            },
        }
    }

}
