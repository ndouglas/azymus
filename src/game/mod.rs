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
use crate::seed;
use seed::SeedType;
use seed::RngType;
use seed::get_rng;
use crate::settings;
use settings::Settings;
use settings::get_settings;
use crate::ui;
use ui::Ui;
use ui::Domain as InputDomain;

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
    pub seed: SeedType,
    /// The random number generator.
    pub rng: RngType,
}

impl Game {

    /// Get the entities at the specified location.
    pub fn get_entities(&self, x: i32, y: i32) -> Vec<&Entity> {
        self.map
            .get_entities(x as usize, y as usize)
            .unwrap_or(std::collections::HashSet::new())
            .iter()
            .map(|&x| &self.entities[x] )
            .collect()
    }

}

/// Setup and run the main game loop.
pub fn run() {
    let seed: SeedType = [
        1, 2, 3, 4,
        5, 6, 7, 8,
        1, 2, 3, 4,
        5, 6, 7, 8,
        1, 2, 3, 4,
        5, 6, 7, 8,
        1, 2, 3, 4,
        5, 6, 7, 8,
    ];
    let mut rng = get_rng(seed);
    let settings = get_settings();
    let mut ui = Ui::new(&settings);
    ui.open();
    let scheduler = Scheduler::new();
    let width = ui.settings.map.width;
    let height = ui.settings.map.height;
    let mut entities = Vec::new();
    let (map, position) = get_map(seed, &mut rng, width, height, 0, &mut entities);
    let player = get_player(&map);
    let player_position = player.position.unwrap();
    let next_id = entities.len();
    let mut game = Game {
        input_domain: InputDomain::Explore,
        map: map,
        entities: entities,
        player_id: next_id,
        settings: get_settings(),
        seed: seed,
        rng: rng,
    };
    let player_id = game.player_id;
    game.entities.push(player);
    Effect::MoveEntity(player_position, position)
        .execute(player_id, &mut game);
    scheduler.feed(&mut game.entities);
    ui.refresh();
    while !ui.is_closed() {
        if let Some(next_id) = scheduler.next(&game.entities) {
            if next_id == player_id {
                ui.render(player_id, &game);
                debug!("Player ID = Next ID.");
                let exit = ui.handle_input(player_id, &mut game);
                if exit {
                    ui.close();
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
