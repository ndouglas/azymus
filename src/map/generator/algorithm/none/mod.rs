use specs::*;
use tcod::colors::*;
use crate::component::position::Position;
use crate::component::renderable::Renderable;

/// Generate the map.
pub fn generate_map(world: &mut World, width: i32, height: i32, _seed: i64) -> (i32, i32) {
    for y in 0..height {
        for x in 0..width {
            world.create_entity()
                .with(Position {
                    x: x,
                    y: y,
                })
                .with(Renderable {
                    char: None,
                    foreground_color: None,
                    background_color: Some(LIGHT_BLUE),
                })
                .build();
        }
    }
    (width / 2, height / 2)
}
