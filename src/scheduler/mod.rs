use crate::command;
use command::Command;
use crate::entity;
use entity::Entity;
use crate::game;
use game::Game;

/// The scheduler.
#[derive(Clone, Copy, Debug)]
pub struct Scheduler {
}

/// The scheduler.
impl Scheduler {

    /// Constructor.
    pub fn new() -> Self {
        Scheduler {
        }
    }

    /// Feed actors.
    pub fn feed(&self, entities: &mut Vec<Entity>) {
        trace!("Entering Scheduler::feed().");
        for entity in entities {
            if let Some(actor) = entity.actor.as_mut() {
                debug!("Feeding entity {} ({}, {}) {} time ({} -> {}).", entity.name, entity.position.unwrap().x, entity.position.unwrap().y, actor.speed, actor.time, actor.time + actor.speed);
                actor.time += actor.speed;
            }
        }
        trace!("Exiting Scheduler::feed().");
    }

    /// Gets the ID of the next actor who should act.
    pub fn next(&self, entities: &Vec<Entity>) -> Option<usize> {
        trace!("Entering Scheduler::next().");
        let mut result = None;
        let mut highest = 0;
        for (id, entity) in entities.iter().enumerate() {
            if let Some(actor) = entity.actor {
                if actor.time > highest {
                    highest = actor.time;
                    debug!("Found new winner {} ({}, {}) ({}).", entity.name, entity.position.unwrap().x, entity.position.unwrap().y, highest);
                    result = Some(id);
                }
            }
        }
        trace!("Exiting Scheduler::next().");
        result
    }

    /// Request a command from the actor.
    pub fn cue(&self, id: usize, game: &mut Game) {
        trace!("Entering Scheduler::cue().");
        let mut command_option = None;
        let entity = &mut game.get_entity_mut(id);
        if let Some(actor) = entity.actor {
            if let Some(agent) = entity.agent {
                command_option = agent.algorithm.get_command(actor.time, id, game);
            }
        }
        if let Some(command) = command_option {
            debug!("Executing {:?} command.", command);
            command.execute(id, game);
        } else {
            debug!("Executing wait command.");
            Command::Wait.execute(id, game);
        }
        trace!("Exiting Scheduler::cue().");
    }

}
