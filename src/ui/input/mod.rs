use tcod::input::*;
use tcod::input::KeyCode::*;
use tcod::console::*;
use crate::command;
use command::Command;
use command::CompassDirection;
use crate::game;
use game::Game;

/// Different scenarios where we handle input.
#[derive(Clone, Copy, Debug)]
pub enum Domain {
    /// Hack 'n' Slash
    Explore,
}

/// Handle input from the player.
pub fn handle_keys(root_console: &mut Root, player_id: usize, game: &mut Game) -> bool {
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
        },
        _ => {
            use Domain::*;
            match game.input_domain {
                Explore => {
                    match key {
                        Key { code: Escape, .. } => return true, // exit game
                        Key { code: Up, .. } => Command::Walk(CompassDirection::North).execute(player_id, game),
                        Key { code: Down, .. } => Command::Walk(CompassDirection::South).execute(player_id, game),
                        Key { code: Left, .. } => Command::Walk(CompassDirection::West).execute(player_id, game),
                        Key { code: Right, .. } => Command::Walk(CompassDirection::East).execute(player_id, game),
                        _ => {},
                    }
                }
            }
        },
    }
   false
}
