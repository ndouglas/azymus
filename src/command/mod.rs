use rand::Rng;
use rand::distributions::{Distribution, Standard};
use crate::action;
use action::Action;
use crate::component;
use component::position::Position;
use crate::game;
use game::Game;
use crate::math;
use math::geometry::cell::Cellular;
use math::geometry::rectangle::Rectangular;
use crate::species;
use species::Species;

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

impl Distribution<CompassDirection> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CompassDirection {
        match rng.gen_range(0, 8) {
            0 => CompassDirection::North,
            1 => CompassDirection::Northeast,
            2 => CompassDirection::Northwest,
            3 => CompassDirection::South,
            4 => CompassDirection::Southeast,
            5 => CompassDirection::Southwest,
            6 => CompassDirection::East,
            _ => CompassDirection::West,
        }
    }
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
    /// Moss: Bloom,
    MossBloom,
    /// Moss: Seed,
    MossSeed(CompassDirection),
    /// Moss: Die,
    MossDie,
}

/// Actions are processes that modify the game world.
impl Command {

    /// Retrieve the action for this command.
    pub fn get_default_action(self) -> Option<Action> {
        trace!("Entering Command::get_default_action() for command {:?}.", self);
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
            MossBloom => {
                Some(Action::MossBloom)
            },
            MossSeed(compass_direction) => {
                Some(Action::MossSeed(compass_direction))
            },
            MossDie => {
                Some(Action::MossDie)
            },
        }
    }

    /// List the preconditions for this command.
    pub fn get_preconditions(self, id: usize, game: &Game) -> Vec<CommandPrecondition> {
        trace!("Entering Command::get_preconditions() for command {:?}.", self);
        use Command::*;
        use CommandPrecondition::*;
        match self {
            Walk(compass_direction) => {
                let entity = &game.entities[id];
                if let Some(position1) = entity.position {
                    let position2 = position1.to_direction(compass_direction);
                    vec![
                        PositionsAreAdjacent(position1, position2),
                        PositionIsNotOutOfBounds(position2),
                        TileAtPositionDoesNotBlockMovement(position2),
                        NothingAtPositionIsValidMeleeAttackTarget(position2),
                        NothingAtPositionBlocksMovement(position2),
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
            MossBloom => {
                vec![
                    Permit,
                ]
            },
            MossSeed(compass_direction) => {
                let entity = &game.entities[id];
                if let Some(position1) = entity.position {
                    let position2 = position1.to_direction(compass_direction);
                    vec![
                        PositionsAreAdjacent(position1, position2),
                        PositionIsNotOutOfBounds(position2),
                        TileAtPositionDoesNotBlockMovement(position2),
                        NothingAtPositionIsOfSpecies(position2, Species::Moss),
                        NothingAtPositionIsOfSpecies(position2, Species::MossSeed),
                    ]
                } else {
                    vec![
                        Deny("Entity has no starting position!".to_string()),
                    ]
                }
            },
            MossDie => {
                vec![
                    Permit,
                ]
            },
        }
    }

    /// Check the preconditions for this command.
    pub fn check_preconditions(self, id: usize, game: &Game) -> CommandPreconditionResult {
        trace!("Entering Command::check_preconditions() for command {:?}.", self);
        use CommandPreconditionResult::*;
        for precondition in self.get_preconditions(id, game) {
            debug!("Checking precondition {:?} for command {:?}.", precondition, self);
            let precondition_name = format!("{:?}", precondition);
            match precondition.evaluate(id, game) {
                Permitted => {
                    debug!("Precondition {} for command {:?} evaluated to Permitted.", precondition_name, self);
                    return Permitted;
                },
                Neutral => {
                    debug!("Precondition {} for command {:?} evaluated to Neutral.", precondition_name, self);
                    continue;
                },
                Denied(string) => {
                    debug!("Precondition {} for command {:?} evaluated to Denied({}).", precondition_name, self, string);
                    return Denied(string);
                },
                Substituted(command) => {
                    debug!("Precondition {} for command {:?} evaluated to Substituted({:?}).", precondition_name, self, command);
                    return Substituted(command);
                }
            }
        }
        return Neutral;
    }

    /// Retrieve the final action for this command.
    pub fn get_final_action(self, id: usize, game: &Game) -> Option<Action> {
        trace!("Entering Command::get_final_action() for command {:?}.", self);
        use CommandPreconditionResult::*;
        match self.check_preconditions(id, game) {
            Permitted => {
                debug!("Returning default action for command {:?}.", self);
                self.get_default_action()
            },
            Neutral => {
                debug!("Returning default action for command {:?}.", self);
                self.get_default_action()
            },
            Denied(_string) => {
                if id == game.player_id {
                    debug!("Returning stall command.");
                    Some(Action::Stall)
                } else {
                    debug!("Returning wait command.");
                    Some(Action::Wait)
                }
            },
            Substituted(command) => {
                debug!("Returning substituted command {:?}.", command);
                command.get_final_action(id, game)
            },
        }
    }

    /// Get the cost for the anticipated action.
    pub fn get_cost(self, id: usize, game: &Game, action: Action) -> i32 {
        trace!("Entering Command::get_cost() for command {:?}.", self);
        action.get_cost(id, game)
    }

    /// Perform the action.
    pub fn execute(self, id: usize, game: &mut Game) {
        trace!("Entering Command::execute() for command {:?}.", self);
        let mut cost = Action::Wait.get_cost(id, game);
        if let Some(action) = self.get_final_action(id, game) {
            cost = action.get_cost(id, game);
            if let Some(effect) = action.execute(id, game) {
                effect.execute(id, game);
            }
        };
        if let Some(actor) = game.entities[id].actor.as_mut() {
            actor.time -= cost;
        };
        if id == game.player_id && cost > 0 {
            game.should_advance = true;
        }
        trace!("Exiting Command::execute() for command {:?}.", self);
    }

}

/// Different ways in which a precondition might affect the intended action.
#[derive(Clone, Debug)]
pub enum CommandPreconditionResult {
    /// The precondition permitted this action to proceed immediately without further checks.  (E.g. in god mode)
    Permitted,
    /// The precondition gave no opinion either way on this attempt.
    /// This should be the most common result.
    Neutral,
    /// The attempted command failed; a message was provided.
    Denied(String),
    /// The attempted command was replaced with another.
    /// (E.g. "moving" to a tile with a hostile occupant -> attack)
    Substituted(Command),
}

/// Preconditions that govern the translation of commands into actions.
///
/// A Command Precondition takes the Command and its context and indicates
/// whether it is permissible for the command to continue.
#[derive(Clone, Debug)]
pub enum CommandPrecondition {
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
    /// Don't seed where there's something of the same species.
    NothingAtPositionIsOfSpecies(Position, Species),
}


/// Preconditions that govern the translation of commands into actions.
impl CommandPrecondition {

    /// Evaluates the given precondition with the specified context.
    pub fn evaluate(self, id: usize, game: &Game) -> CommandPreconditionResult {
        trace!("Entering CommandPrecondition::evaluate() with precondition {:?}.", self);
        use CommandPreconditionResult::*;
        use CommandPrecondition::*;
        match self {
            Permit => Permitted,
            Deny(string) => Denied(string),
            Substitute(command) => Substituted(command),
            PositionIsNotOutOfBounds(position) => {
                trace!("Entering precondition {:?}.", PositionIsNotOutOfBounds(position));
                let map = &game.map;
                if !map.as_rectangle().contains_cell(&position.as_cell()) {
                    debug!("Position {:?} is not in bounds of the map.", position);
                    return Denied("Requested an out-of-bounds position.".to_string());
                }
                Neutral
            },
            TileAtPositionDoesNotBlockMovement(position) => {
                trace!("Entering precondition {:?}.", TileAtPositionDoesNotBlockMovement(position));
                let map = &game.map;
                if let Some(tile) = map.tile_map.get_tile(&position.as_cell()) {
                    if tile.blocks_movement {
                        return Denied("The destination position contains a tile that blocks movement.".to_string());
                    }
                }
                Neutral
            },
            PositionsAreAdjacent(position1, position2) => {
                trace!("Entering precondition {:?}.", PositionsAreAdjacent(position1, position2));
                if (position1.x - position2.x).abs() > 1 || (position1.y - position2.y).abs() > 1 {
                    return Denied("The destination position is not adjacent to the original position.".to_string());
                }
                Neutral
            },
            NothingAtPositionBlocksMovement(position) => {
                trace!("Entering precondition {:?}.", NothingAtPositionBlocksMovement(position));
                let entity = &game.entities[id];
                let occupants = &game.get_entities(position.x, position.y);
                for occupant in occupants {
                    if occupant.blocks_movement {
                        debug!("Entity {} ({}, {}) is blocked by entity {} ({}, {}).", entity.name, entity.position.unwrap().x, entity.position.unwrap().y, occupant.name, position.x, position.y);
                        return Denied("The destination position contains an entity that blocks movement.".to_string());
                    }
                }
                debug!("Entity {} ({}, {}) is not blocked by anything at ({}, {}).", entity.name, entity.position.unwrap().x, entity.position.unwrap().y, position.x, position.y);
                Neutral
            },
            SomethingAtPositionIsValidMeleeAttackTarget(position) => {
                trace!("Entering precondition {:?}.", SomethingAtPositionIsValidMeleeAttackTarget(position));
                let entity = &game.entities[id];
                let occupants = &game.get_entities(position.x, position.y);
                for occupant in occupants {
                    if entity.would_attack(occupant) {
                        debug!("Entity {} ({}, {}) would attack {} ({}, {}).", entity.name, entity.position.unwrap().x, entity.position.unwrap().y, occupant.name, position.x, position.y);
                        return Neutral;
                    }
                }
                debug!("Entity {} ({}, {}) would not attack anything at ({}, {}).", entity.name, entity.position.unwrap().x, entity.position.unwrap().y, position.x, position.y);
                Substituted(Command::Walk(entity.position.unwrap().direction_to(&position).unwrap()))
            },
            NothingAtPositionIsValidMeleeAttackTarget(position) => {
                trace!("Entering precondition {:?}.", NothingAtPositionIsValidMeleeAttackTarget(position));
                let entity = &game.entities[id];
                let occupants = &game.get_entities(position.x, position.y);
                for occupant in occupants {
                    if entity.would_attack(occupant) {
                        debug!("Entity {} ({}, {}) would attack {} ({}, {}).", entity.name, entity.position.unwrap().x, entity.position.unwrap().y, occupant.name, position.x, position.y);
                        return Substituted(Command::MeleeAttack(entity.position.unwrap().direction_to(&position).unwrap()));
                    }
                }
                debug!("Entity {} ({}, {}) would not attack anything at ({}, {}).", entity.name, entity.position.unwrap().x, entity.position.unwrap().y, position.x, position.y);
                Neutral
            },
            NothingAtPositionIsOfSpecies(position, bad_species) => {
                trace!("Entering precondition {:?}.", NothingAtPositionIsOfSpecies(position, bad_species));
                let entities = &game.get_entities(position.x, position.y);
                for entity in entities {
                    if let Some(species) = entity.species {
                        if species == bad_species {
                            return Denied(format!("Found entity {:?} of undesired species {:?} at position {:?}.", entity, bad_species, position));
                        }
                    }
                }
                debug!("Did not find entities of undesired species {:?} at position {:?}.", bad_species, position);
                Neutral
            },
        }
    }

}
