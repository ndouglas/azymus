use specs::*;
use tcod::input::*;
use tcod::input::KeyCode::*;
use crate::command::*;
use crate::resource;
use resource::continue_flag::ContinueFlagResource;
use resource::root_console::RootConsoleResource;

/// Input domains.
#[derive(Clone, Copy, Debug)]
pub enum Domain {
    /// The main domain; moving around a dungeon or world.
    Explore,
}

impl Domain {

    /// Input mapper.
    ///
    /// Returns a command if the input event could be successfully mapped to one.
    pub fn get_command(self, event: Event, _entity: Entity, world: &mut World) -> Option<Command> {
        match self {
            Domain::Explore => {
                match event {
                    Event::Key(key) => {
                        match key {
                            Key { code: Enter, alt: true, .. }  => {
                                trace!("Toggling fullscreen.");
                                let root_console_resource = &mut world.write_resource::<RootConsoleResource>().0;
                                let root_console = &mut root_console_resource.lock().unwrap();
                                let fullscreen = root_console.is_fullscreen();
                                root_console.set_fullscreen(!fullscreen);
                                None
                            },
                            Key { code: Escape, .. }            => {
                                trace!("Quitting game!");
                                let continue_flag_resource = &mut world.write_resource::<ContinueFlagResource>().0;
                                *continue_flag_resource = false;
                                None
                            },
                            Key { code: Up, .. }                => { Some(Command::MoveNorth) },
                            Key { code: Down, .. }              => { Some(Command::MoveSouth) },
                            Key { code: Left, .. }              => { Some(Command::MoveWest) },
                            Key { code: Right, .. }             => { Some(Command::MoveEast) },
                            _                                   => { None },
                        }
                    },
                    _                                       => { None },
                }
            }
        }
    }

    /// Input mapper.
    ///
    /// Returns a command if the input event could be successfully mapped to one.
    pub fn handle_event(self, event: Event, entity: Entity, world: &mut World) {
        if let Some(command) = self.get_command(event, entity, world) {
            command.execute(entity, world);
        }
    }

}
