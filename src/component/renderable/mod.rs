use tcod::colors::*;
use tcod::console::*;

/// Indicates how the given object is rendered on a map.
#[derive(Clone, Debug)]
pub struct Renderable {
    /// The character used to render the object.
    pub char: Option<char>,
    /// The color used to render the object.
    pub foreground_color: Option<Color>,
    /// The color used to render the object.
    pub background_color: Option<Color>,
}

/// Indicates how the given object is rendered on a map.
impl Renderable {

    /// Constructor.
    pub fn new() -> Renderable {
        trace!("Entering Renderable::to_north().");
        Renderable {
            char: None,
            foreground_color: None,
            background_color: None,
        }
    }

    /// Render this object at the specified position.
    pub fn draw(&self, x: i32, y: i32, console: &mut Console) {
        trace!("Entering Renderable::draw().");
        if let Some(color) = self.foreground_color {
            if let Some(char) = self.char {
                console.set_default_foreground(color);
                console.put_char(x, y, char, BackgroundFlag::None);
            }
        }
        if let Some(color) = self.background_color {
            console.set_char_background(x, y, color, BackgroundFlag::Set);
        }
        trace!("Exiting Renderable::draw().");
    }

    /// Return a modified version of this renderable.
    pub fn with_char(&self, char: Option<char>) -> Renderable {
        Renderable {
            char: char,
            foreground_color: self.foreground_color,
            background_color: self.background_color,
        }
    }

    /// Return a modified version of this renderable.
    pub fn with_foreground_color(&self, foreground_color: Option<Color>) -> Renderable {
        Renderable {
            char: self.char,
            foreground_color: foreground_color,
            background_color: self.background_color,
        }
    }

    /// Return a modified version of this renderable.
    pub fn with_background_color(&self, background_color: Option<Color>) -> Renderable {
        Renderable {
            char: self.char,
            foreground_color: self.foreground_color,
            background_color: background_color,
        }
    }

}

/// Creates a default instance.
impl Default for Renderable {

    /// Creates a default instance.
    fn default() -> Self {
        Renderable::new()
    }

}

/// A factory for renderables.
#[derive(Clone, Copy, Debug)]
pub enum Factory {
    /// The PC.
    Player,
    /// An orc.
    Orc,
    /// A troll.
    Troll,
    /// A floor (dark).
    Floor,
    /// A wall (dark).
    Wall,
}

impl Factory {

    /// Creates a renderable for the given value.
    pub fn create(self) -> Renderable {
        trace!("Entering Factory::create().");
        use Factory::*;
        match self {
            Player => Renderable {
                char: Some('@'),
                foreground_color: Some(WHITE),
                background_color: None,
            },
            Orc => Renderable {
                char: Some('o'),
                foreground_color: Some(DESATURATED_GREEN),
                background_color: None,
            },
            Troll => Renderable {
                char: Some('T'),
                foreground_color: Some(GREEN),
                background_color: None,
            },
            Floor => Renderable {
                char: Some('.'),
                foreground_color: Some(BLACK),
                background_color: Some(Color {
                    r: 16,
                    g: 16,
                    b: 16,
                }),
            },
            Wall => Renderable {
                char: Some('#'),
                foreground_color: Some(BLACK),
                background_color: Some(Color {
                    r: 0,
                    g: 0,
                    b: 0,
                }),
            },
        }
    }

}
