use tcod::input::*;
use tcod::input::KeyCode::*;
use tcod::console::*;
use crate::command;
use command::Command;
use crate::component;
use component::position::Position;
use crate::game;
use game::Game;

/// Handle input from the player.
pub fn handle_keys(root_console: &mut Root, player_id: usize, game: &mut Game) -> bool {
    let player = &game.entities[player_id];
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
        Key { code: Up, .. } => Command::Walk(Position {
            w: player.position.unwrap().w,
            x: player.position.unwrap().x,
            y: player.position.unwrap().y - 1,
            z: player.position.unwrap().z,
        }).execute(player_id, game),
        Key { code: Down, .. } => Command::Walk(Position {
            w: player.position.unwrap().w,
            x: player.position.unwrap().x,
            y: player.position.unwrap().y + 1,
            z: player.position.unwrap().z,
        }).execute(player_id, game),
        Key { code: Left, .. } => Command::Walk(Position {
            w: player.position.unwrap().w,
            x: player.position.unwrap().x - 1,
            y: player.position.unwrap().y,
            z: player.position.unwrap().z,
        }).execute(player_id, game),
        Key { code: Right, .. } => Command::Walk(Position {
            w: player.position.unwrap().w,
            x: player.position.unwrap().x + 1,
            y: player.position.unwrap().y,
            z: player.position.unwrap().z,
        }).execute(player_id, game),
        _ => {},
    }
   false
}
