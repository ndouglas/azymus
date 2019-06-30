use specs::*;
use crate::action;
use action::Action;
use crate::command;
use command::Command;
use crate::component;
use component::actor::Actor;
use component::baton::Baton;
use component::name::Name;
use crate::resource;
use resource::seed::SeedResource;

const DEFAULT_ACTION_COST: i32 = 120;

/// Processes wait commands.
#[derive(Clone, Copy, Debug)]
pub struct WaitSystem;

/// Processes wait commands.
impl<'a> System<'a> for WaitSystem {

    type SystemData = (
        Entities<'a>,
        Read<'a, SeedResource>,
        WriteStorage<'a, Actor>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Baton>,
    );

    fn run(&mut self, data: Self::SystemData) {
        trace!("Entering WaitSystem::run().");
        let (
            entities,
            seed_resource,
            mut actor_storage,
            name_storage,
            baton_storage,
        ) = data;
        let _seed = seed_resource.0;
        for (_entity, _, name, actor) in (&entities, &baton_storage, &name_storage, &mut actor_storage).join() {
            if let Some(command) = actor.command_queue.front() {
                match command {
                    Command::Wait => {
                        debug!("Found wait command for actor {}.", name.name);
                        let _action = Some(Action::Wait);
                        let cost = DEFAULT_ACTION_COST;
                        actor.command_queue.pop_front();
                        debug!("Removing {} energy from actor {} ({} -> {}).", cost, name.name, actor.energy, actor.energy - cost);
                        actor.energy -= cost;
                    },
                    _ => {},
                }
            }
        }
        trace!("Exiting WaitSystem::run().");
    }
}
