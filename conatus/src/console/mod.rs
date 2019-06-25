use azymus::component::position::Position;
use tcod::console::*;

/// Create a root console with a specified height and width.
pub fn get_root_console(width: i32, height: i32) -> Root {
    let root = Root::initializer()
        .font("resources/tcod/fonts/arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(width, height)
        .title("Conatus")
        .init();
    root
}

/// Create a map console with a specified height and width.
pub fn get_map_console(width: i32, height: i32) -> Offscreen {
    let map = Offscreen::new(width, height);
    map
}

/// Blit map console to root console.
pub fn blit_map_console(map: &mut Offscreen, root: &mut Root) {
    blit(
        map,
        (0, 0),
        (map.width(), map.height()),
        root,
        (0, 0),
        1.0,
        1.0,
    );
}

/// Handle input.
pub fn handle_keys(root: &mut Root, map: &Offscreen, position: &mut Position) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;
    let key = root.wait_for_keypress(true);
    match key {
        Key {
            code: Enter,
            alt: true,
            ..
        } => {
            let fullscreen = root.is_fullscreen();
            root.set_fullscreen(!fullscreen);
        }
        Key { code: Escape, .. } => return true, // exit game
        Key { code: Up, .. } => {
            if position.y <= 0 {
                return false;
            }
            position.y -= 1
        },
        Key { code: Down, .. } => {
            if position.y >= map.height() - 1 {
                return false;
            }
            position.y += 1
        },
        Key { code: Left, .. } => {
            if position.x <= 0 {
                return false;
            }
            position.x -= 1
        },
        Key { code: Right, .. } => {
            if position.x >= map.width() - 1 {
                return false;
            }
            position.x += 1
        },
        _ => {},
    }
   false
}
