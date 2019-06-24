/// An experimental roguelike (library), written in Rust.
extern crate azymus;
use azymus::component::renderable::*;

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
    let mut root_console = conatus::console::get_root_console(160, 100);
    let mut map_console = conatus::console::get_map_console(160, 95);
    let mut x = 80;
    let mut y = 50;
    while !root_console.window_closed() {
        map_console.clear();
        map_console.render_renderable(x, y, &Renderable {
            char: Some('@'),
            foreground_color: Some(WHITE),
            background_color: None,
        });
        conatus::console::blit_map_console(&mut map_console, &mut root_console);
        root_console.flush();
        let exit = conatus::console::handle_keys(&mut root_console, &mut x, &mut y);
        if exit {
            break
        }
    }
    trace!("Exiting main().");
}
