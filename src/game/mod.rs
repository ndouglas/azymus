use tcod::console::*;

use crate::settings;
use crate::ui;

/// Setup and run the main game loop.
pub fn run() {
    let settings = settings::get_settings();
    let mut root_console = ui::root_console::get_root_console(&settings);
    let mut x = root_console.width() / 2;
    let mut y = root_console.height() / 2;
    while !root_console.window_closed() {
        use tcod::colors::*;
        use tcod::console::*;
        root_console.set_default_foreground(WHITE);
        root_console.clear();
        root_console.put_char(x, y, '@', BackgroundFlag::None);
        root_console.flush();
        // handle keys and exit game if needed
        let exit = ui::input::handle_keys(&mut root_console, &mut x, &mut y);
        if exit {
            break
        }
    }
}
