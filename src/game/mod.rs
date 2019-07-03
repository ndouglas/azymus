use tcod::colors::*;
use tcod::console::*;
use crate::entity;
use entity::Entity;
use entity::get_player;
use entity::get_npc;
use crate::map;
use map::Map;
use map::get_map;
use crate::settings;
use settings::get_settings;
use crate::ui;
use ui::input::handle_keys;
use ui::map_console::get_map_console;
use ui::root_console::get_root_console;

/// Setup and run the main game loop.
pub fn run() {
    let settings = get_settings();
    let mut root_console = get_root_console(&settings);
    let mut map_console = get_map_console(&settings);
    let width = map_console.width();
    let height = map_console.height();
    let seed: i64 = 0;
    let (map, position) = get_map(seed, width, height, 0);
    let mut player = get_player(&map);
    let mut npc = get_npc(&map);
    player.move_to(position.x, position.y, 0);
    npc.move_to(position.x + 20, position.y, 0);
    let mut objects = [
        player,
        npc,
    ];
    while !root_console.window_closed() {
        render_all(&mut root_console, &mut map_console, &objects, &map);
        let mut player = &mut objects[0];
        let exit = handle_keys(&mut root_console, &mut player);
        if exit {
            break
        }
    }
}

fn render_all(root_console: &mut Root, map_console: &mut Offscreen, objects: &[Entity], map: &Map) {
    map_console.set_default_foreground(WHITE);
    map_console.clear();
    let player = &objects[0];
    if let Some(fov) = &player.field_of_view {
        if let Some(ls) = &player.light_source {
            map.draw_fov_ls(map_console, fov, ls);
        } else {
            map.draw_fov(map_console, fov);
        }
        let fov_map = fov.map.lock().unwrap();
        for object in objects {
            if let Some(position) = object.position {
                if fov_map.is_in_fov(position.x, position.y) {
                    object.draw(map_console);
                }
            }
        }
    } else {
        for object in objects {
            object.draw(map_console);
        }
    }
    blit(
        map_console,
        (0, 0),
        (map.width, map.height),
        root_console,
        (0, 0),
        1.0,
        1.0,
    );
    root_console.flush();
}
