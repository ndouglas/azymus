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

/// Azymus (the library).
extern crate azymus;

/// A Rust library providing a lightweight logging facade.
#[macro_use]
extern crate log;

/// A pretty, easy-to-use logger for Rust.
extern crate pretty_env_logger;


#[cfg_attr(tarpaulin, skip)]
fn main() {
    use azymus::settings;
    use azymus::ui;
    azymus::init();
    trace!("Entering main().");
    let settings = settings::get_settings();
    let mut root_console = ui::root_console::get_root_console(&settings);
    while !root_console.window_closed() {
        use tcod::colors::*;
        use tcod::console::*;
        root_console.set_default_foreground(WHITE);
        root_console.clear();
        root_console.put_char(1, 1, '@', BackgroundFlag::None);
        root_console.flush();
        root_console.wait_for_keypress(true);
    }
    trace!("Exiting main().");
}

#[cfg(test)]
mod tests {

    #[test]
    fn basic_sanity_test() {
        assert_eq!(2 + 2, 4);
    }

}
