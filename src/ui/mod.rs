use std::fmt;
use bear_lib_terminal::terminal as blt;
use bear_lib_terminal::Color;
use blt::config::font as blt_font;
use blt::Event;
use bear_lib_terminal::geometry::Size;
use crate::command;
use command::Command;
use command::CompassDirection;
use crate::game;
use game::Game;
use crate::settings;
use settings::Settings;

/// Different scenarios where we handle input.
#[derive(Clone, Copy, Debug)]
pub enum Domain {
    /// Hack 'n' Slash
    Explore,
}

/// The User Interface abstraction.
pub struct Ui {
    /// The settings object.
    pub settings: Settings,
}

/// The User Interface abstraction.
impl Ui {

    /// Constructor.
    pub fn new(settings: &Settings) -> Self {
        Ui {
            settings: settings.clone(),
        }
    }

    /// Open the window.
    pub fn open(&self) {
        blt::open("Azymus", self.settings.display.width as u32, self.settings.display.height as u32);
        blt::set(blt_font::true_type(blt_font::Origin::Root, "resources/azymus/fonts/symbola.ttf", Size::new(0, 10)));
        blt::set(vec![
            blt::config::InputFilter::Group {
                group: blt::config::InputFilterGroup::Keyboard,
                both: false
            },
            blt::config::InputFilter::Group {
                group: blt::config::InputFilterGroup::Mouse,
                both: false
            },
        ]);
    }

    /// Close the window.
    pub fn close(&self) {
        blt::close();
    }

    /// Refresh the window.
    pub fn refresh(&self) {
        blt::refresh();
    }

    /// If the window is closed.
    pub fn is_closed(&self) -> bool {
        return false;
    }

    /// Render a frame.
    pub fn render(&mut self, player_id: usize, game: &Game) {
        blt::clear(None);
        let map = &game.map;
        let player = &game.entities[player_id];
        if let Some(fov) = &player.field_of_view {
            map.draw(&self, fov, game);
        }
        let position = blt::state::mouse::position();
        blt::with_colors(Color::from_rgb(255, 255, 255), Color::from_rgb(0, 0, 0), || {
            let xy_entities = map.get_entities(position.x as usize, position.y as usize);
            let entities = xy_entities
                .iter()
                .map(|&id| game.entities[id].clone());
            if let Some(top_entity) = entities.last() {
                blt::print_xy(position.x + 1, position.y, &top_entity.name);
            }
        });
        self.refresh();
    }

    /// Handle input.
    pub fn handle_input(&mut self, player_id: usize, game: &mut Game) -> bool {
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
                            _ => {
                                println!("{:?}", key);
                            },
                        }
                    },
                }
            },
            Some(MouseScroll {
                delta,
            }) => {
                use Domain::*;
                match game.input_domain {
                    Explore => {
                        let position = blt::state::mouse::position();
                        blt::print(position, &format!("{}", delta));
                    },
                }
            },
            Some(MouseMove {
                x: _,
                y: _,
            }) => {
                use Domain::*;
                match game.input_domain {
                    Explore => {
                    }
                }
            },
            _ => {},
        }
       false
    }

}

/// Allows us to show this object in tests, etc.
impl fmt::Debug for Ui {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UI")
    }
}
