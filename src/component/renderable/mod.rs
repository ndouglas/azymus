use tcod::colors::*;

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
        }
    }

}
