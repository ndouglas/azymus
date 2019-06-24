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

fn main() {
    pretty_env_logger::init();
    trace!("Entering main().");
    trace!("Exiting main().");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_basic_sanity_test() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_main() {
        main();
    }

}
