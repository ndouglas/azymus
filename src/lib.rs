#![warn(
    clippy::pedantic
)]
#![deny(
    clippy::all,
    missing_docs,
    missing_debug_implementations,
//    missing_copy_implementations,
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

/// EXTERNAL CRATES

/// A rustic FFI for BearLibTerminal;
#[link(name = "lib/libBearLibTerminal.dylib")]
extern crate bear_lib_terminal;

/// A Rust library providing a lightweight logging facade.
#[macro_use]
extern crate log;

/// A quadtree-like structure, but for arbitrary arity.
extern crate ntree;

/// A pretty, easy-to-use logger for Rust.
extern crate pretty_env_logger;

/// A random number generator library for Rust.
extern crate rand;

/// Serde-Derive: Serializing and deserializing macros.
#[macro_use]
extern crate serde_derive;

/// Serde-Json: Serializing and deserializing macros.
// #[macro_use]
extern crate serde_json;

/// Bindings for the tcod library.
extern crate tcod;

/// MODULES

/// Actions are processes that modify the game world.
pub mod action;

/// Agents are algorithms that permit NPC actors to act within the game world.
pub mod agent;

/// Bodies are the forms of "living" entities.
pub mod body;

/// Colors are very important, since this is a graphical game.
pub mod color;

/// Combat manages any sort of attack.
pub mod combat;

/// Commands are attempts to perform actions.
pub mod command;

/// Components are reusable structures for disparate kinds of objects.
pub mod component;

/// Effects are direct modifications of the game world.
pub mod effect;

/// Any object that exists within the game.
pub mod entity;

/// Events that occur and are significant enough for a nearby object to react.
pub mod event;

/// Factions are the groups to which things belong.
pub mod faction;

/// The game structure, run loop, etc.
pub mod game;

/// The map, the current slice of the game world.
pub mod map;

/// A crappy builtin math library.
pub mod math;

/// The message panel.
pub mod messages;

/// The scheduler that determines when and who will be able to issue commands.
pub mod scheduler;

/// The seed input that we use for random number generation.
pub mod seed;

/// A settings system to allow some configuration.
pub mod settings;

/// A species of which an entity can be a part.
pub mod species;

/// The tiles that form the map.
pub mod tile;

/// The UI, specifically consoles, input, etc.
pub mod ui;

/// Perform some initialization stuff.
pub fn init() {
    pretty_env_logger::init();
    trace!("Entering init().");
    trace!("Exiting init().");
}

#[cfg(test)]
mod tests {

    #[test]
    fn basic_sanity_test() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_init() {
        super::init();
    }

}
