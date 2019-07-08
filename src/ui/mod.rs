use std::fmt;
use bear_lib_terminal::terminal as blt;
use blt::config::font as blt_font;
use bear_lib_terminal::geometry::Size;
use crate::game;
use game::Game;
use crate::settings;
use settings::Settings;

/// Input.
pub mod input;
/// The map console.
pub mod map_console;
/// The root console.
pub mod root_console;

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
        self.refresh();




        /*
        self.map_console.set_default_foreground(WHITE);
        self.map_console.clear();
        let map = &game.map;
        let player = &game.entities[player_id];
        if let Some(fov) = &player.field_of_view {
            if let Some(ls) = &player.light_source {
                map.draw_fov_ls(&mut self.map_console, fov, ls);
            } else {
                map.draw_fov(&mut self.map_console, fov);
            }
            let fov_map = fov.map.lock().unwrap();
            let entities_to_draw: Vec<_> = game.entities
                .iter()
                .filter(|e| e.position.is_some())
                .filter(|e| e.is_in_fov(&fov_map))
                .collect();
            for entity in entities_to_draw {
                entity.draw(&mut self.map_console);
            }
        }
        blit(
            &mut self.map_console,
            (0, 0),
            (self.settings.map.width, self.settings.map.height),
            &mut self.root_console,
            (0, 0),
            1.0,
            1.0,
        );
        if let Some(body) = &game.entities[player_id].body {
            self.root_console.print_ex(
                1,
                self.root_console.height() - 2,
                BackgroundFlag::None,
                TextAlignment::Left,
                format!("HP: {}/{} ", body.current_hit_points, body.total_hit_points),
            );
        }
        self.root_console.flush();
        */
    }

    /// Handle input.
    pub fn handle_input(&mut self, player_id: usize, game: &mut Game) -> bool {
        input::handle_keys(player_id, game)
    }

}

/// Allows us to show this object in tests, etc.
impl fmt::Debug for Ui {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UI")
    }
}
