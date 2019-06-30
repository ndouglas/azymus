
/// An experimental roguelike (library), written in Rust.
extern crate azymus;
use azymus::world::*;


/// Testing scaffolds for Azymus.
extern crate conatus;

/// A Rust library providing a lightweight logging facade.
#[macro_use]
extern crate log;

/// A pretty, easy-to-use logger for Rust.
extern crate pretty_env_logger;

/// Specs
extern crate specs;

/// Bindings for the tcod library.
extern crate tcod;

fn main() {
    pretty_env_logger::init();
    trace!("Entering main().");
    run_game_loop();
    trace!("Exiting main().");
}
