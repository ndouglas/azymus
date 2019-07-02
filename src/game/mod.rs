use tcod::console::*;

use crate::entity;
use crate::settings;
use crate::ui;

/// Setup and run the main game loop.
pub fn run() {
    let settings = settings::get_settings();
    let mut root_console = ui::root_console::get_root_console(&settings);
    let mut map_console = ui::map_console::get_map_console(&settings);
    let width = map_console.width();
    let height = map_console.height();
    let x = width / 2;
    let y = height / 2;
    let mut player = entity::Entity::new();
    player.position = Some(entity::component::position::Position {
        w: 0,
        x: x,
        y: y,
        z: 0,
    });
    player.renderable = Some(entity::component::renderable::Renderable {
        char: Some('@'),
        background_color: None,
        foreground_color: Some(tcod::colors::WHITE),

    });
    let mut npc = entity::Entity::new();
    npc.position = Some(entity::component::position::Position {
        w: 0,
        x: x - 20,
        y: y,
        z: 0,
    });
    npc.renderable = Some(entity::component::renderable::Renderable {
        char: Some('@'),
        background_color: None,
        foreground_color: Some(tcod::colors::YELLOW),

    });
    while !root_console.window_closed() {
        use tcod::colors::*;
        use tcod::console::*;
        map_console.set_default_foreground(WHITE);
        map_console.clear();
        player.draw(&mut map_console);
        npc.draw(&mut map_console);
        blit(
            &mut map_console,
            (0, 0),
            (width, height),
            &mut root_console,
            (0, 0),
            1.0,
            1.0,
        );
        root_console.flush();
        // handle keys and exit game if needed
        let exit = ui::input::handle_keys(&mut root_console, &mut player);
        if exit {
            break
        }
    }
}
