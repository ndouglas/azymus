use tcod::console::*;
use tcod::system::set_fps;

/// Create a root console with a specified height and width.
pub fn get_root_console(width: i32, height: i32) -> Root {
    let root = Root::initializer()
        .font("resources/tcod/fonts/arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(width, height)
        .title("Azymus")
        .init();
    set_fps(60);
    root
}

/// Create a map console with a specified height and width.
pub fn get_map_console(width: i32, height: i32) -> Offscreen {
    let map = Offscreen::new(width, height);
    map
}
