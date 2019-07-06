use tcod::colors::*;
use tcod::console::*;
use crate::effect;
use effect::Effect;
use crate::entity;
use entity::Entity;
use entity::get_player;
use crate::map;
use map::Map;
use map::get_map;
use crate::scheduler;
use scheduler::Scheduler;
use crate::settings;
use settings::Settings;
use settings::get_settings;
use crate::ui;
use ui::input::Domain as InputDomain;
use ui::input::handle_keys;
use ui::map_console::get_map_console;
use ui::root_console::get_root_console;

/// The game object.
#[derive(Clone, Debug)]
pub struct Game {
    /// The input domain.
    pub input_domain: InputDomain,
    /// The game map.
    pub map: Map,
    /// All entities in the game.
    pub entities: Vec<Entity>,
    /// The player entity ID.
    pub player_id: usize,
    /// The game settings.
    pub settings: Settings,
    /// The current seed.
    pub seed: i64,
}

impl Game {

    /// Get the entities at the specified location.
    pub fn get_entities(&self, x: i32, y: i32) -> Vec<&Entity> {
        self.map
            .get_entities(x as usize, y as usize)
            .iter()
            .map(|&x| &self.entities[x] )
            .collect()
    }

}

/// Setup and run the main game loop.
pub fn run() {
    let seed: i64 = 0;
    let settings = get_settings();
    let mut root_console = get_root_console(&settings);
    let mut map_console = get_map_console(&settings);
    let scheduler = Scheduler::new();
    let width = map_console.width();
    let height = map_console.height();
    let mut entities = Vec::new();
    let (map, position) = get_map(seed, width, height, 0, &mut entities);
    let player = get_player(&map);
    let player_position = player.position.unwrap();
    let next_id = entities.len();
    let mut game = Game {
        input_domain: InputDomain::Explore,
        map: map,
        entities: entities,
        player_id: next_id,
        settings: get_settings(),
        seed: 0,
    };
    let player_id = game.player_id;
    game.entities.push(player);
    Effect::MoveEntity(player_position, position)
        .execute(player_id, &mut game);
    scheduler.feed(&mut game.entities);
    while !root_console.window_closed() {
        if let Some(next_id) = scheduler.next(&game.entities) {
            if next_id == player_id {
                render_all(&mut root_console, &mut map_console, player_id, &game);
                debug!("Player ID = Next ID.");
                let exit = handle_keys(&mut root_console, player_id, &mut game);
                if exit {
                    return;
                } else {
                    debug!("Feeding.");
                    scheduler.feed(&mut game.entities);
                }
            } else {
                debug!("Cueing.");
                scheduler.cue(next_id, &mut game);
            }
        } else {
            debug!("Feeding.");
            scheduler.feed(&mut game.entities);
        }
    }
}

fn render_all(root_console: &mut Root, map_console: &mut Offscreen, player_id: usize, game: &Game) {
    map_console.set_default_foreground(WHITE);
    map_console.clear();
    let map = &game.map;
    let player = &game.entities[player_id];
    if let Some(fov) = &player.field_of_view {
        if let Some(ls) = &player.light_source {
            map.draw_fov_ls(map_console, fov, ls);
        } else {
            map.draw_fov(map_console, fov);
        }
        let fov_map = fov.map.lock().unwrap();
        for object in &game.entities {
            if let Some(position) = object.position {
                if fov_map.is_in_fov(position.x, position.y) {
                    object.draw(map_console);
                }
            }
        }
    } else {
        for object in &game.entities {
            object.draw(map_console);
        }
    }
    blit(
        map_console,
        (0, 0),
        (map.width as i32, map.height as i32),
        root_console,
        (0, 0),
        1.0,
        1.0,
    );
    if let Some(body) = &game.entities[player_id].body {
        root_console.print_ex(
            1,
            root_console.height() - 2,
            BackgroundFlag::None,
            TextAlignment::Left,
            format!("HP: {}/{} ", body.current_hit_points, body.total_hit_points),
        );
    }
    root_console.flush();
}
