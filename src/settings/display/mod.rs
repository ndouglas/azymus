/// The title of this game.
static DEFAULT_TITLE: &str = "Azymus";
/// Default screen width in characters.
const DEFAULT_WIDTH: i32 = 160;
/// Default screen height in characters.
const DEFAULT_HEIGHT: i32 = 100;
/// Default maximum frames rendered per second.
const DEFAULT_FPS_LIMIT: i32 = 60;
/// Default font file.
static DEFAULT_FONT_FILE: &str = "arial10x10.png";

/// Display-specific settings.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Display {
    /// The title of the game.
    pub title: String,
    /// The width of the display.
    pub width: i32,
    /// The height of the display.
    pub height: i32,
    /// The display limit on FPS.
    pub fps_limit: i32,
    /// The font file used to render the game.
    pub font_file: String,
}

/// Display-specific settings.
impl Display {

    /// Get title.
    pub fn get_title(&self) -> &String {
        &self.title
    }

    /// Set title.
    pub fn set_title(&mut self, title: &String) {
        self.title = title.to_string();
    }

    /// Constructor.
    pub fn new() -> Self {
        Display {
            title: DEFAULT_TITLE.to_string(),
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            fps_limit: DEFAULT_FPS_LIMIT,
            font_file: DEFAULT_FONT_FILE.to_string(),
        }
    }

    /// Get font file path.
    pub fn get_font_file_path(&self) -> String {
        format!("resources/tcod/fonts/{}", &self.font_file)
    }

}

impl Default for Display {
    fn default() -> Self {
        Display::new()
    }
}


#[cfg(test)]
mod tests {

}
