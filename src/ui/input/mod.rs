use tcod::input::*;
use tcod::input::KeyCode::*;
use tcod::console::*;

/// Handle input from the player.
pub fn handle_keys(root_console: &mut Root, x: &mut i32, y: &mut i32) -> bool {
    let key = root_console.wait_for_keypress(true);
    match key {
        Key {
            code: Enter,
            alt: true,
            ..
        } => {
            // Alt+Enter: toggle fullscreen
            let fullscreen = root_console.is_fullscreen();
            root_console.set_fullscreen(!fullscreen);
        }
        Key { code: Escape, .. } => return true, // exit game
        Key { code: Up, .. } => *y -= 1,
        Key { code: Down, .. } => *y += 1,
        Key { code: Left, .. } => *x -= 1,
        Key { code: Right, .. } => *x += 1,
        _ => {},
    }

   false
}
