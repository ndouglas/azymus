use bear_lib_terminal::Color;

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
    /// A goblin.
    Goblin,
    /// A kobold.
    Kobold,
    /// A chicken.
    Chicken,
    /// A mushroom.
    Mushroom,
    /// Moss.
    Moss,
    /// A human,
    Human,
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
                foreground_color: Some(Color::from_rgb(255, 255, 255)),
                background_color: None,
            },
            Orc => Renderable {
                char: Some('o'),
                foreground_color: Some(Color::from_rgb(64, 128, 64)),
                background_color: None,
            },
            Troll => Renderable {
                char: Some('T'),
                foreground_color: Some(Color::from_rgb(0, 255, 0)),
                background_color: None,
            },
            Goblin => Renderable {
                char: Some('g'),
                foreground_color: Some(Color::from_rgb(0, 164, 96)),
                background_color: None,
            },
            Kobold => Renderable {
                char: Some('k'),
                foreground_color: Some(Color::from_rgb(0, 255, 128)),
                background_color: None,
            },
            Chicken => Renderable {
                char: Some('c'),
                foreground_color: Some(Color::from_rgb(230, 230, 230)),
                background_color: None,
            },
            Mushroom => Renderable {
                char: Some('🍄'),
                foreground_color: Some(Color::from_rgb(230, 230, 230)),
                background_color: None,
            },
            Moss => Renderable {
                char: Some('#'),
                foreground_color: Some(Color::from_rgb(173, 223, 173)),
                background_color: None,
            },
            Human => Renderable {
                char: Some('h'),
                foreground_color: Some(Color::from_rgb(115, 115, 255)),
                background_color: None,
            },
            Floor => Renderable {
                char: Some('.'),
                foreground_color: Some(Color::from_rgb(0, 0, 0)),
                background_color: Some(Color::from_rgb(32, 32, 32)),
            },
            Wall => Renderable {
                char: Some('#'),
                foreground_color: Some(Color::from_rgb(0, 0, 0)),
                background_color: Some(Color::from_rgb(16, 16, 16)),
            },
        }
    }

}
