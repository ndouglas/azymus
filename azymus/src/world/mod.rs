use specs::*;
use crate::resource;
use resource::continue_flag::ContinueFlagResource;
use resource::map_console::MapConsoleResource;
use resource::root_console::RootConsoleResource;
use tcod::input::Event;
use tcod::console::*;
use std::ops::DerefMut;

/// Extensions for the World.
pub trait WorldExtension {

    /// Should we continue in the main loop?
    fn should_continue(&self) -> bool;

    /// Should we continue in the main loop?
    fn wait_for_keypress(&self) -> Event;

    /// Blit map console to root console.
    fn blit_map_console(&mut self);

    /// Tick.
    fn tick(&mut self);

}

/// Extensions for the World.
impl WorldExtension for World {

    /// Should we continue in the main loop?
    fn should_continue(&self) -> bool {
        let continue_flag = self.read_resource::<ContinueFlagResource>().0;
        let window_closed = (self.read_resource::<RootConsoleResource>().0)
            .lock()
            .unwrap()
            .window_closed();
        return continue_flag && !window_closed;
    }

    /// Should we continue in the main loop?
    fn wait_for_keypress(&self) -> Event {
        let key = (self.read_resource::<RootConsoleResource>().0)
            .lock()
            .unwrap()
            .wait_for_keypress(true);
        Event::Key(key)
    }

    /// Blit map console to root console.
    fn blit_map_console(&mut self) {
        let map_console_resource = &mut self.write_resource::<MapConsoleResource>().0;
        let mut map_console = map_console_resource.lock().unwrap();
        let root_console_resource = &mut self.write_resource::<RootConsoleResource>().0;
        let mut root_console = root_console_resource.lock().unwrap();
        let coordinates = (map_console.width(), map_console.height());
        blit(map_console.deref_mut(), (0, 0), coordinates, root_console.deref_mut(), (0, 0), 1.0, 1.0);
    }

    /// Tick.
    fn tick(&mut self) {
    }

}
