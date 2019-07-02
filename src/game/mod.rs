use tcod::console::*;
use crate::entity;
use crate::map;
use crate::settings;
use crate::ui;

/// Setup and run the main game loop.
pub fn run() {
    let settings = settings::get_settings();
    let mut root_console = ui::root_console::get_root_console(&settings);
    let mut map_console = ui::map_console::get_map_console(&settings);
    let width = map_console.width();
    let height = map_console.height();
    let mut player = entity::get_player();
    let mut npc = entity::get_npc();
    let seed: i64 = 0;
    let (map, position) = map::get_map(seed, width, height, 0);
    player.move_to(position.x, position.y, 0);
    npc.move_to(position.x + 20, position.y, 0);
    while !root_console.window_closed() {
        use tcod::colors::*;
        use tcod::console::*;
        map_console.set_default_foreground(WHITE);
        map_console.clear();
        map.draw(&mut map_console);
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
