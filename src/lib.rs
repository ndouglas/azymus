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
//#[macro_use]
extern crate serde_derive;

/// Serde-Json: Serializing and deserializing macros.
// #[macro_use]
extern crate serde_json;

/// Bindings for the tcod library.
extern crate tcod;

/// MODULES

/// Simple geometry for screen representations.
pub mod geometry;

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
