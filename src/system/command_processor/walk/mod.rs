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
use component::position::Position;
use crate::resource;
use resource::occupant_map::OccupantMapResource;
use resource::seed::SeedResource;
use crate::rule;
use rule::action::check_map_boundaries;
use rule::action::check_map_occupied;
// fn check_map_boundaries(x: i32, y: i32, map_height: i32, map_width: i32) -> Option<Action>
// fn check_map_occupied<'a>(x: i32, y: i32, occupant_storage: &ReadStorage<'a, Occupant>, position_storage: &ReadStorage<'a, Position>) -> Option<Action> {

const DEFAULT_ACTION_COST: i32 = 120;

/// Processes move commands.
#[derive(Clone, Copy, Debug)]
pub struct WalkSystem;

/// Processes move commands.
impl<'a> System<'a> for WalkSystem {

    type SystemData = (
        Entities<'a>,
        Read<'a, SeedResource>,
        ReadExpect<'a, OccupantMapResource>,
        WriteStorage<'a, Actor>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Baton>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Action>,
    );

    fn run(&mut self, data: Self::SystemData) {
        trace!("Entering MoveSystem::run().");
        let (
            entities,
            seed_resource,
            occupant_map_resource,
            mut actor_storage,
            name_storage,
            baton_storage,
            position_storage,
            mut action_storage,
        ) = data;
        let _seed = seed_resource.0;
        let occupant_map = & occupant_map_resource.0;
        for (entity, _, name, actor, position) in (&entities, &baton_storage, &name_storage, &mut actor_storage, &position_storage).join() {
            debug!("Found WTF ({:?}) command on actor {}...", actor.command_queue.front(), name.name);
            if let Some(Command::Walk(compass_direction)) = actor.command_queue.front() {
                debug!("Found Walk({:?}) command on actor {}...", compass_direction, name.name);
                use CompassDirection::*;
                let action_option = match compass_direction {
                    North => {
                        let x2 = position.x;
                        let y2 = position.y - 1;
                        if !check_map_boundaries(x2, y2, 100, 160) {
                            None
                        } else if !check_map_occupied(x2, y2, &occupant_map) {
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
                        } else if !check_map_occupied(x2, y2, &occupant_map) {
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
                        } else if !check_map_occupied(x2, y2, &occupant_map) {
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
                        } else if !check_map_occupied(x2, y2, &occupant_map) {
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
                    }
                }
                let cost = DEFAULT_ACTION_COST;
                debug!("Removing {} energy from actor {} ({} -> {}).", cost, name.name, actor.energy, actor.energy - cost);
                actor.energy -= cost;
                debug!("Removing {:?} command from actor {}.", actor.command_queue.front(), name.name);
                actor.command_queue.pop_front();
            }
        }
        trace!("Exiting WalkSystem::run().");
    }
}
