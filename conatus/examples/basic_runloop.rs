/// An experimental roguelike (library), written in Rust.
extern crate azymus;

/// Testing scaffolds for Azymus.
extern crate conatus;

/// A Rust library providing a lightweight logging facade.
#[macro_use]
extern crate log;

/// A pretty, easy-to-use logger for Rust.
extern crate pretty_env_logger;

/// Bindings for the tcod library.
extern crate tcod;
use tcod::colors::*;
use tcod::console::*;

fn main() {
    pretty_env_logger::init();
    trace!("Entering main().");
    let mut root = conatus::console::get_root_console(160, 100);
    while !root.window_closed() {
        root.set_default_foreground(WHITE);
        root.clear();
        root.put_char(1, 1, '@', BackgroundFlag::None);
        root.flush();
        root.wait_for_keypress(true);
    }
    trace!("Exiting main().");
}
