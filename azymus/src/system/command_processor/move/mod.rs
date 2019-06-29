use specs::*;
use crate::action;
use action::Action;
use crate::command;
use command::Command;
use command::CompassDirection;
use crate::component;
use component::actor::Actor;
use component::baton::Baton;
use component::name::Name;
use component::occupant::Occupant;
use component::position::Position;
use crate::resource;
use resource::seed::SeedResource;
use crate::rule;
use rule::action::check_map_boundaries;
use rule::action::check_map_occupied;
// fn check_map_boundaries(x: i32, y: i32, map_height: i32, map_width: i32) -> Option<Action>
// fn check_map_occupied<'a>(x: i32, y: i32, occupant_storage: &ReadStorage<'a, Occupant>, position_storage: &ReadStorage<'a, Position>) -> Option<Action> {

const DEFAULT_ACTION_COST: i32 = 120;

/// Processes move commands.
#[derive(Clone, Copy, Debug)]
pub struct MoveSystem;

/// Processes move commands.
impl<'a> System<'a> for MoveSystem {

    type SystemData = (
        Entities<'a>,
        Read<'a, SeedResource>,
        WriteStorage<'a, Actor>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Baton>,
        ReadStorage<'a, Occupant>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Action>,
    );

    fn run(&mut self, data: Self::SystemData) {
        trace!("Entering MoveSystem::run().");
        let (
            entities,
            seed_resource,
            mut actor_storage,
            name_storage,
            baton_storage,
            occupant_storage,
            position_storage,
            mut action_storage,
        ) = data;
        let _seed = seed_resource.0;
        for (entity, _, name, actor, position) in (&entities, &baton_storage, &name_storage, &mut actor_storage, &position_storage).join() {
            debug!("Found WTF ({:?}) command on actor {}...", actor.command_queue.front(), name.name);
            if let Some(Command::Move(compass_direction)) = actor.command_queue.front() {
                debug!("Found Move({:?}) command on actor {}...", compass_direction, name.name);
                use CompassDirection::*;
                let action_option = match compass_direction {
                    North => {
                        let x2 = position.x;
                        let y2 = position.y - 1;
                        if !check_map_boundaries(x2, y2, 100, 160) {
                            None
                        } else if !check_map_occupied(x2, y2, &occupant_storage, &position_storage) {
                            None
                        } else {
                            Some(Action::Walk((position.x, position.y), (x2, y2)))
                        }
                    },
                    South => {
                        let x2 = position.x;
                        let y2 = position.y + 1;
                        if !check_map_boundaries(x2, y2, 100, 160) {
                            None
                        } else if !check_map_occupied(x2, y2, &occupant_storage, &position_storage) {
                            None
                        } else {
                            Some(Action::Walk((position.x, position.y), (x2, y2)))
                        }
                    },
                    West => {
                        let x2 = position.x - 1;
                        let y2 = position.y;
                        if !check_map_boundaries(x2, y2, 100, 160) {
                            None
                        } else if !check_map_occupied(x2, y2, &occupant_storage, &position_storage) {
                            None
                        } else {
                            Some(Action::Walk((position.x, position.y), (x2, y2)))
                        }
                    },
                    East => {
                        let x2 = position.x + 1;
                        let y2 = position.y;
                        if !check_map_boundaries(x2, y2, 100, 160) {
                            None
                        } else if !check_map_occupied(x2, y2, &occupant_storage, &position_storage) {
                            None
                        } else {
                            Some(Action::Walk((position.x, position.y), (x2, y2)))
                        }
                    },
                };
                if let Some(action) = action_option {
                    debug!("Inserting action component {:?} for actor {}.", action, name.name);
                    if let Ok(_) = action_storage.insert(entity, action) {
                        debug!("Inserted an action component {:?} for actor {}.", action, name.name);
                        let cost = DEFAULT_ACTION_COST;
                        debug!("Removing {:?} command from actor {}.", actor.command_queue.front(), name.name);
                        actor.command_queue.pop_front();
                        debug!("Removing {} energy from actor {} ({} -> {}).", cost, name.name, actor.energy, actor.energy - cost);
                        actor.energy -= cost;
                    }
                }
            }
        }
        trace!("Exiting MoveSystem::run().");
    }
}


/*
/// Checks the map boundaries to make sure we aren't moving something off the map.
pub fn check_map_boundaries(action: Action, x: i32, y: i32, map_console: &Offscreen) -> Option<Action> {
    trace!("Checking if coordinates ({}, {}) are outside map boundaries...", x, y);
    if x <= 0 || y <= 0 {
        trace!("Coordinates ({}, {}) were out of bounds.", x, y);
        return None;
    }
    if x >= map_console.width() - 1 || y >= map_console.height() - 1 {
        trace!("Coordinates ({}, {}) were out of bounds.", x, y);
        return None;
    }
    trace!("Coordinates ({}, {}) were not out of bounds.", x, y);
    Some(action)
}

/// Checks a specific position in the map to see if there's an Occupant there.
pub fn check_map_occupied(action: Action, x: i32, y: i32, world: &World) -> Option<Action> {
    trace!("Checking if coordinates ({}, {}) are occupied...", x, y);
    let occupant_storage = world.read_storage::<Occupant>();
    let position_storage = world.read_storage::<Position>();
    let occupant_count = (&occupant_storage, &position_storage)
        .join()
        .filter(|(_, position)| position.x == x && position.y == y)
        .count();
    if occupant_count > 0 {
        trace!("Occupant found at ({}, {}).", x, y);
        return None;
    }
    trace!("No occupant found at ({}, {}).", x, y);
    return Some(action);
}
*/
