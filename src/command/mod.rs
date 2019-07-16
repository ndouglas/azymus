use crate::action;
use action::Action;
use crate::game;
use game::Game;
use crate::math;
use math::geometry::cell::Cell;
use math::geometry::cell::Cellular;
use math::geometry::compass::Direction as CompassDirection;
use math::geometry::rectangle::Rectangular;
use crate::species;
use species::Species;

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
                let entity = &game.get_entity(id);
                let cell1 = entity.cell;
                if let Some(cell2) = cell1.to_direction(&compass_direction, &game.map.as_rectangle()) {
                    vec![
                        CellsAreAdjacent(cell1, cell2),
                        CellIsNotOutOfBounds(cell2),
                        TileAtCellDoesNotBlockMovement(cell2),
                        NothingAtCellIsValidMeleeAttackTarget(cell2),
                        NothingAtCellBlocksMovement(cell2),
                    ]
                } else {
                    vec![
                        Deny("Entity has no destination cell!".to_string()),
                    ]
                }
            },
            MeleeAttack(compass_direction) => {
                let entity = &game.get_entity(id);
                let cell1 = entity.cell;
                if let Some(cell2) = cell1.to_direction(compass_direction, &game.map.as_rectangle()) {
                    vec![
                        CellsAreAdjacent(cell1, cell2),
                        CellIsNotOutOfBounds(cell2),
                        SomethingAtCellIsValidMeleeAttackTarget(cell2),
                    ]
                } else {
                    vec![
                        Deny("Entity has no starting cell!".to_string()),
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
                let entity = &game.get_entity(id);
                if let Some(cell1) = entity.cell {
                    let cell2 = cell1.to_direction(compass_direction);
                    vec![
                        CellsAreAdjacent(cell1, cell2),
                        CellIsNotOutOfBounds(cell2),
                        TileAtCellDoesNotBlockMovement(cell2),
                        NothingAtCellIsOfSpecies(cell2, Species::Moss),
                        NothingAtCellIsOfSpecies(cell2, Species::MossSeed),
                    ]
                } else {
                    vec![
                        Deny("Entity has no starting cell!".to_string()),
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
        if let Some(actor) = game.world.entity_list.vector[id].actor.as_mut() {
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
    /// Cell is not out of bounds.
    CellIsNotOutOfBounds(Cell),
    /// Cell does not block movement.
    TileAtCellDoesNotBlockMovement(Cell),
    /// The cells are adjacent.
    CellsAreAdjacent(Cell, Cell),
    /// No entity at the location blocks movement.
    NothingAtCellBlocksMovement(Cell),
    /// No entity at the location can be attacked.
    NothingAtCellIsValidMeleeAttackTarget(Cell),
    /// Some entity at the location can be attacked.
    SomethingAtCellIsValidMeleeAttackTarget(Cell),
    /// Don't seed where there's something of the same species.
    NothingAtCellIsOfSpecies(Cell, Species),
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
            CellIsNotOutOfBounds(cell) => {
                trace!("Entering precondition {:?}.", CellIsNotOutOfBounds(cell));
                let map = &game.map;
                if !map.as_rectangle().contains_cell(&cell.as_cell()) {
                    debug!("Cell {:?} is not in bounds of the map.", cell);
                    return Denied("Requested an out-of-bounds cell.".to_string());
                }
                Neutral
            },
            TileAtCellDoesNotBlockMovement(cell) => {
                trace!("Entering precondition {:?}.", TileAtCellDoesNotBlockMovement(cell));
                let map = &game.map;
                if let Some(tile) = map.tile_map.get_tile(&cell.as_cell()) {
                    if tile.blocks_movement {
                        return Denied("The destination cell contains a tile that blocks movement.".to_string());
                    }
                }
                Neutral
            },
            CellsAreAdjacent(cell1, cell2) => {
                trace!("Entering precondition {:?}.", CellsAreAdjacent(cell1, cell2));
                if (cell1.x - cell2.x).abs() > 1 || (cell1.y - cell2.y).abs() > 1 {
                    return Denied("The destination cell is not adjacent to the original cell.".to_string());
                }
                Neutral
            },
            NothingAtCellBlocksMovement(cell) => {
                trace!("Entering precondition {:?}.", NothingAtCellBlocksMovement(cell));
                let entity = &game.get_entity(id);
                let occupants = &game.get_entities(cell.x, cell.y);
                for occupant in occupants {
                    if occupant.blocks_movement {
                        debug!("Entity {} ({}, {}) is blocked by entity {} ({}, {}).", entity.name, entity.cell.unwrap().x, entity.cell.unwrap().y, occupant.name, cell.x, cell.y);
                        return Denied("The destination cell contains an entity that blocks movement.".to_string());
                    }
                }
                debug!("Entity {} ({}, {}) is not blocked by anything at ({}, {}).", entity.name, entity.cell.unwrap().x, entity.cell.unwrap().y, cell.x, cell.y);
                Neutral
            },
            SomethingAtCellIsValidMeleeAttackTarget(cell) => {
                trace!("Entering precondition {:?}.", SomethingAtCellIsValidMeleeAttackTarget(cell));
                let entity = &game.get_entity(id);
                let occupants = &game.get_entities(cell.x, cell.y);
                for occupant in occupants {
                    if entity.would_attack(occupant) {
                        debug!("Entity {} ({}, {}) would attack {} ({}, {}).", entity.name, entity.cell.unwrap().x, entity.cell.unwrap().y, occupant.name, cell.x, cell.y);
                        return Neutral;
                    }
                }
                debug!("Entity {} ({}, {}) would not attack anything at ({}, {}).", entity.name, entity.cell.unwrap().x, entity.cell.unwrap().y, cell.x, cell.y);
                Substituted(Command::Walk(entity.cell.unwrap().direction_to(&cell).unwrap()))
            },
            NothingAtCellIsValidMeleeAttackTarget(cell) => {
                trace!("Entering precondition {:?}.", NothingAtCellIsValidMeleeAttackTarget(cell));
                let entity = &game.get_entity(id);
                let occupants = &game.get_entities(cell.x, cell.y);
                for occupant in occupants {
                    if entity.would_attack(occupant) {
                        debug!("Entity {} ({}, {}) would attack {} ({}, {}).", entity.name, entity.cell.unwrap().x, entity.cell.unwrap().y, occupant.name, cell.x, cell.y);
                        return Substituted(Command::MeleeAttack(entity.cell.unwrap().direction_to(&cell).unwrap()));
                    }
                }
                debug!("Entity {} ({}, {}) would not attack anything at ({}, {}).", entity.name, entity.cell.unwrap().x, entity.cell.unwrap().y, cell.x, cell.y);
                Neutral
            },
            NothingAtCellIsOfSpecies(cell, bad_species) => {
                trace!("Entering precondition {:?}.", NothingAtCellIsOfSpecies(cell, bad_species));
                let entities = &game.get_entities(cell.x, cell.y);
                for entity in entities {
                    if let Some(species) = entity.species {
                        if species == bad_species {
                            return Denied(format!("Found entity {:?} of undesired species {:?} at cell {:?}.", entity, bad_species, cell));
                        }
                    }
                }
                debug!("Did not find entities of undesired species {:?} at cell {:?}.", bad_species, cell);
                Neutral
            },
        }
    }

}
