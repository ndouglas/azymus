use crate::color;
use color::Color;
use crate::component;
use component::renderable::Renderable;
use super::Tile;

/// The factory.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Factory {
    /// Floor.
    Floor,
    /// Wall.
    Wall,
}

impl Factory {

    /// Generate the map.
    pub fn create(&self) -> Tile {
        use Factory::*;
        match self {
            Floor => Tile {
                renderable: Renderable {
                    char: Some('.'),
                    foreground_color: Some(Color::from_rgb(0, 0, 0)),
                    background_color: Some(Color::from_rgb(32, 32, 32)),
                },
                blocks_movement: false,
                blocks_light: false,
            },
            Wall => Tile {
                renderable: Renderable {
                    char: Some('#'),
                    foreground_color: Some(Color::from_rgb(0, 0, 0)),
                    background_color: Some(Color::from_rgb(16, 16, 16)),
                },
                blocks_movement: true,
                blocks_light: true,
            },
        }
    }

}
