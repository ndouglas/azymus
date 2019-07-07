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
use ui::Ui;
use ui::input::Domain as InputDomain;

/// The game object.
#[derive(Debug)]
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
    let mut ui = Ui::new(&settings);
    let scheduler = Scheduler::new();
    let width = ui.settings.map.width;
    let height = ui.settings.map.height;
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
    while !ui.is_closed() {
        if let Some(next_id) = scheduler.next(&game.entities) {
            if next_id == player_id {
                ui.render(player_id, &game);
                debug!("Player ID = Next ID.");
                let exit = ui.handle_input(player_id, &mut game);
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
