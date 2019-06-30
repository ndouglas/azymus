use tcod::input::*;
use tcod::input::KeyCode::*;
use crate::command::*;

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
    pub fn get_command(self, event: Event) -> Option<Command> {
        use Command::*;
        use CompassDirection::*;
        match self {
            Domain::Explore => {
                match event {
                    Event::Key(key) => {
                        match key {
                            Key { code: Up, .. }                => { Some(Walk(North)) },
                            Key { code: Down, .. }              => { Some(Walk(South)) },
                            Key { code: Left, .. }              => { Some(Walk(West)) },
                            Key { code: Right, .. }             => { Some(Walk(East)) },
                            _                                   => { None },
                        }
                    },
                    _                                       => { None },
                }
            }
        }
    }

}
