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

/// Commands.
pub mod command;

/// Components.
pub mod component;

/// Input.
pub mod input;

/// Map.
pub mod map;

/// Resource.
pub mod resource;

/// Rules.
pub mod rule;

/// World.
pub mod world;

/// Initialize some things.
pub fn init() {
    pretty_env_logger::init();
    trace!("Entering init().");
    trace!("Exiting init().");
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

}
