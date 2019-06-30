#![warn(
    clippy::pedantic
)]
#![deny(
    clippy::all,
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    warnings
)]

//! Azymus is nominally a roguelike game.
//!
//! I'm writing it to play with simulation and procedural generation.
//!
//! I don't know where I'm going with this.

/// A Rust library providing a lightweight logging facade.
#[macro_use]
extern crate log;

/// A pretty, easy-to-use logger for Rust.
extern crate pretty_env_logger;

/// Parallel ECS.
extern crate specs;

/// Custom derive macros for Specs components.
#[macro_use]
extern crate specs_derive;

/// Bindings for the tcod library.
extern crate tcod;

/// Actions.
pub mod action;

/// Agents.
pub mod agent;

/// Commands.
pub mod command;

/// Components.
pub mod component;

/// Entities.
pub mod entity;

/// Input.
pub mod input;

/// Map.
pub mod map;

/// Resource.
pub mod resource;

/// Rules.
pub mod rule;

/// Systems.
pub mod system;

/// User Interface.
pub mod ui;

/// World.
pub mod world;

fn main() {
    pretty_env_logger::init();
    trace!("Entering main().");
    world::run_game_loop();
    trace!("Exiting main().");
}


#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

}
