use specs::*;
use tcod::colors::*;
use crate::component::occupant::Occupant;
use crate::component::position::Position;
use crate::component::renderable::Renderable;
use crate::component::tile::Tile;

/// Presets.
pub mod preset;
use preset::*;

/// Create a tile that may or may not be a wall.
pub fn get_tile(world: &mut World, is_wall: bool, x: i32, y: i32) -> Entity {
    let mut entity_builder = world.create_entity();
    let mut color = FLOOR_LIT_COLOR;
    if is_wall {
        entity_builder = entity_builder.with(Occupant);
        color = WALL_LIT_COLOR;
    }
    let entity = entity_builder
        .with(Tile)
        .with(Position {
            x: x,
            y: y,
        })
        .with(Renderable {
            char: None,
            foreground_color: None,
            background_color: Some(color),
        })
        .build();
    entity
}

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
