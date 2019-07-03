use tcod::colors::*;
use tcod::console::*;

/// Indicates how the given object is rendered on a map.
#[derive(Clone, Copy, Debug)]
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
        Renderable {
            char: None,
            foreground_color: None,
            background_color: None,
        }
    }

    /// Render this object at the specified position.
    pub fn draw(&self, x: i32, y: i32, console: &mut Console) {
        if let Some(color) = self.foreground_color {
            if let Some(char) = self.char {
                console.set_default_foreground(color);
                console.put_char(x, y, char, BackgroundFlag::None);
            }
        }
        if let Some(color) = self.background_color {
            console.set_char_background(x, y, color, BackgroundFlag::Set);
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
    /// A little test NPC.
    Npc,
    /// A floor (dark).
    Floor,
    /// A wall (dark).
    Wall,
}

impl Factory {

    /// Creates a renderable for the given value.
    pub fn create(self) -> Renderable {
        use Factory::*;
        match self {
            Player => Renderable {
                char: Some('@'),
                foreground_color: Some(WHITE),
                background_color: None,
            },
            Npc => Renderable {
                char: Some('@'),
                foreground_color: Some(YELLOW),
                background_color: None,
            },
            Floor => Renderable {
                char: None,
                foreground_color: None,
                background_color: Some(Color {
                    r: 16,
                    g: 16,
                    b: 16,
                }),
            },
            Wall => Renderable {
                char: None,
                foreground_color: None,
                background_color: Some(Color {
                    r: 0,
                    g: 0,
                    b: 0,
                }),
            },
        }
    }

}
