use bear_lib_terminal::terminal as blt;
use blt::Event;
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
pub fn handle_keys(player_id: usize, game: &mut Game) -> bool {
    let event = blt::wait_event();
    use Event::*;
    match event {
        Some(Close) => return true,
        Some(KeyPressed {
            key,
            ctrl: _ctrl,
            shift: _shift,
        }) => {
            use Domain::*;
            match game.input_domain {
                Explore => {
                    use blt::KeyCode;
                    use KeyCode::*;
                    match key {
                        Escape => return true,
                        Up => Command::Walk(CompassDirection::North).execute(player_id, game),
                        Down => Command::Walk(CompassDirection::South).execute(player_id, game),
                        Left => Command::Walk(CompassDirection::West).execute(player_id, game),
                        Right => Command::Walk(CompassDirection::East).execute(player_id, game),
                        _ => {},
                    }
                },
            }
        }
        _ => {},
    }
   false
}
