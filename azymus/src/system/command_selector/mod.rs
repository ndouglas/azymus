use specs::*;
use crate::component;
use component::actor::Actor;
use component::agent::Agent;
use component::baton::Baton;
use component::name::Name;
use component::player::Player;
use crate::resource;
use resource::input_flag::InputFlagResource;
use resource::seed::SeedResource;

/// Determines which command the actor with the baton will perform.
#[derive(Clone, Copy, Debug)]
pub struct CommandSelectorSystem;

/// Determines which command the actor with the baton will perform.
impl<'a> System<'a> for CommandSelectorSystem {

    type SystemData = (
        Entities<'a>,
        Read<'a, SeedResource>,
        Write<'a, InputFlagResource>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Baton>,
        WriteStorage<'a, Actor>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Agent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        trace!("Entering CommandSelectorSystem::run().");
        let (
            entities,
            seed_resource,
            mut input_flag_resource,
            name_storage,
            baton_storage,
            mut actor_storage,
            player_storage,
            agent_storage,
        ) = data;
        let seed = seed_resource.0;
        for (entity, _, name, actor) in (&entities, &baton_storage, &name_storage, &mut actor_storage).join() {
            if let Some(command) = actor.command_queue.front() {
                debug!("Command {:?} is currently enqueued for actor {}.", command, name.name);
            } else {
                debug!("No command currently enqueued for actor {}.", name.name);
                if let Some(agent) = agent_storage.get(entity) {
                    debug!("Consulting agent for actor {}.", name.name);
                    if let Some(command) = agent.agent.get_next_command(seed, actor.energy) {
                        debug!("Enqueueing command {:?} for actor {}.", command, name.name);
                        actor.command_queue.push_back(command);
                    } else {
                        debug!("Agent {:?} for actor {} failed to provide a command.  Doing nothing.", agent, name.name);
                    }
                } else {
                    // Player or something.
                    debug!("No agent found for actor {}...", name.name);
                    if let Some(_) = player_storage.get(entity) {
                        input_flag_resource.0 = true;
                    }
                }
            }
        }
        trace!("Exiting CommandSelectorSystem::run().");
    }
}
