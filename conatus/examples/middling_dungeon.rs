use std::sync::{Arc,Mutex};

/// An experimental roguelike (library), written in Rust.
extern crate azymus;
use azymus::component::actor::Actor;
use azymus::component::agent::Agent;
use azymus::component::player_explored::PlayerExplored;
use azymus::component::field_of_view::FieldOfView;
use azymus::component::name::Name;
use azymus::component::occupant::Occupant;
use azymus::component::opaque::Opaque;
use azymus::component::player::Player;
use azymus::component::position::Position;
use azymus::component::renderable::*;
use azymus::component::tile::Tile;
use azymus::entity::player::get_player;
use azymus::map::generator::algorithm::Algorithm;
use azymus::resource::continue_flag::ContinueFlagResource;
use azymus::resource::map_console::MapConsoleResource;
use azymus::resource::root_console::RootConsoleResource;
use azymus::resource::seed::SeedResource;
use azymus::system::actor_feeder::ActorFeederSystem;
use azymus::system::field_of_view::FieldOfViewSystem;
use azymus::system::map_renderer::MapRendererSystem;
use azymus::system::player_explored_marker::PlayerExploredMarkerSystem;
use azymus::world::*;


/// Testing scaffolds for Azymus.
extern crate conatus;

/// A Rust library providing a lightweight logging facade.
#[macro_use]
extern crate log;

/// A pretty, easy-to-use logger for Rust.
extern crate pretty_env_logger;

/// Specs
extern crate specs;
use specs::*;

/// Bindings for the tcod library.
extern crate tcod;

fn main() {
    pretty_env_logger::init();
    trace!("Entering main().");
    let screen_height = 100;
    let screen_width = 160;
    let map_width = 160;
    let map_height = screen_height - 5;
    let root_console = conatus::console::get_root_console(screen_width, screen_height);
    let map_console = conatus::console::get_map_console(map_width, map_height);
    let mut world = World::new();
    world.register::<Actor>();
    world.register::<Agent>();
    world.register::<FieldOfView>();
    world.register::<Name>();
    world.register::<Occupant>();
    world.register::<Opaque>();
    world.register::<Player>();
    world.register::<PlayerExplored>();
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Tile>();
    world.add_resource(ContinueFlagResource::default());
    world.add_resource(MapConsoleResource(Arc::new(Mutex::new(map_console))));
    world.add_resource(RootConsoleResource(Arc::new(Mutex::new(root_console))));
    world.add_resource(SeedResource(0));
    let seed: i64 = 0;
    let starting_position = Algorithm::Simple.generate_map(&mut world, map_width, map_height, seed);
    let player = get_player(&mut world, starting_position.0, starting_position.1, seed);
    let mut dispatcher = DispatcherBuilder::new()
        .with(FieldOfViewSystem, "field_of_view", &[])
        .with(ActorFeederSystem, "actor_feeder", &[])
        .with(PlayerExploredMarkerSystem, "player_explored_marker", &[
            "field_of_view",
        ])
        .with(MapRendererSystem, "map_renderer", &[
            "player_explored_marker",
        ])
        .build();
    dispatcher.setup(&mut world.res);
    while world.should_continue() {
        dispatcher.dispatch(&mut world.res);
        world.tick();
        world.maintain();
        let exit = conatus::console::handle_keys(player, &mut world);
        if exit {
            break
        }
    }
    trace!("Exiting main().");
}
