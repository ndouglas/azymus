use tcod::colors::*;
use crate::component::renderable::Renderable;

/// Presets.
pub mod preset;

/// Returns the "unlit" version of a color.
pub fn obscure_color(color: &Color) -> Color {
    Color {
        r: color.r / 2,
        g: color.g / 2,
        b: color.b / 2,
    }
}

/// Returns the "unlit" version of a tile.
pub fn obscure_renderable(renderable: &Renderable) -> Renderable {
    Renderable {
        char: renderable.char,
        foreground_color: renderable.foreground_color,
        background_color: match renderable.background_color {
            Some(Color {
                r: 200,
                g: 180,
                b: 50,
            }) => Some(Color {
                r: 50,
                g: 50,
                b: 150,
            }),
            Some(Color {
                r: 130,
                g: 110,
                b: 50,
            }) => Some(Color {
                r: 0,
                g: 0,
                b: 100
            }),
            Some(color) => Some(color),
            None => None,
        },
    }
}