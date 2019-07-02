use tcod::input::*;
use tcod::input::KeyCode::*;
use tcod::console::*;
use crate::action;
use action::Action;
use crate::entity;
use entity::Entity;
use entity::component::position::Position;

/// Handle input from the player.
pub fn handle_keys(root_console: &mut Root, player: &mut Entity) -> bool {
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
        Key { code: Up, .. } => Action::Move(Position {
            w: player.position.unwrap().w,
            x: player.position.unwrap().x,
            y: player.position.unwrap().y - 1,
            z: player.position.unwrap().z,
        }).execute(player),
        Key { code: Down, .. } => Action::Move(Position {
            w: player.position.unwrap().w,
            x: player.position.unwrap().x,
            y: player.position.unwrap().y + 1,
            z: player.position.unwrap().z,
        }).execute(player),
        Key { code: Left, .. } => Action::Move(Position {
            w: player.position.unwrap().w,
            x: player.position.unwrap().x - 1,
            y: player.position.unwrap().y,
            z: player.position.unwrap().z,
        }).execute(player),
        Key { code: Right, .. } => Action::Move(Position {
            w: player.position.unwrap().w,
            x: player.position.unwrap().x + 1,
            y: player.position.unwrap().y,
            z: player.position.unwrap().z,
        }).execute(player),
        _ => {},
    }
   false
}
