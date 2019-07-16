use crate::effect;
use effect::Effect;
use crate::entity;
use entity::Entity;
use entity::get_player;
use crate::map;
use map::Map;
use map::get_map;
use crate::math;
use math::geometry::cell::Cell;
use math::geometry::cell::Cellular;
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
use crate::world;
use world::World;

/// The game object.
#[derive(Debug)]
pub struct Game {
    /// The input domain.
    pub input_domain: InputDomain,
    /// The game map.
    pub map: Map,
    /// The game world.
    pub world: World,
    /// The player entity ID.
    pub player_id: usize,
    /// The game settings.
    pub settings: Settings,
    /// The current seed.
    pub seed: SeedType,
    /// The random number generator.
    pub rng: RngType,
    /// Which turn of the game we're on.
    pub turns: usize,
    /// Whether or not we should advance the clock.
    pub should_advance: bool,
    /// Whether or not we should continue.
    pub should_continue: bool,
}

impl Game {

    /// Get the entity with the specified ID.
    pub fn get_entity(&self, id: usize) -> &Entity {
        &self.world.entity_list.vector[id]
    }

    /// Get the entity mutably with the specified ID.
    pub fn get_entity_mut(&mut self, id: usize) -> &mut Entity {
        &mut self.world.entity_list.vector[id]
    }

    /// Get the entities at the specified location.
    pub fn get_entities(&self, x: i32, y: i32) -> Vec<&Entity> {
        self.map
            .entity_map
            .get_entity_ids_cell(&Cell::new(x as usize, y as usize))
            .unwrap_or(std::collections::HashSet::new())
            .iter()
            .map(|&id| self.get_entity(id) )
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
    let rng = get_rng(seed);
    let settings = get_settings();
    let mut ui = Ui::new(&settings);
    ui.open();
    let scheduler = Scheduler::new();
    let width = ui.settings.map.width;
    let height = ui.settings.map.height;
    let mut world = World::new();
    let map = get_map(seed, width, height);
    let player = get_player(&map);
    let player_position = player.position.unwrap();
    let start_cell = map.tile_map.start;
    println!("{}", start_cell);
    let player_id = world.entity_list.insert_entity(player);
    let mut game = Game {
        input_domain: InputDomain::Explore,
        map: map,
        world: world,
        player_id: player_id,
        settings: get_settings(),
        seed: seed,
        rng: rng,
        turns: 0,
        should_advance: false,
        should_continue: true,
    };
    Effect::MoveEntity(player_position.as_cell(), start_cell)
        .execute(player_id, &mut game);
    scheduler.feed(&mut game.world.entity_list.vector);
    ui.refresh();
    while !ui.is_closed() {
        if let Some(next_id) = scheduler.next(&game.world.entity_list.vector) {
            if next_id == player_id {
                ui.render(player_id, &game);
                debug!("Player ID = Next ID.");
                game.should_advance = false;
                game.should_continue = true;
                ui.handle_input(player_id, &mut game);
                if !game.should_continue {
                    ui.close();
                    return;
                } else if game.should_advance {
                    debug!("Feeding.");
                    scheduler.feed(&mut game.world.entity_list.vector);
                }
            } else {
                debug!("Cueing.");
                scheduler.cue(next_id, &mut game);
            }
        } else {
            debug!("Feeding.");
            scheduler.feed(&mut game.world.entity_list.vector);
        }
    }
}
