use std::sync::{Arc,Mutex};
use tcod::input::Event;
use specs::*;
use crate::component;
use component::actor::Actor;
use component::agent::Agent;
use component::command_queue::CommandQueue;
use component::player_explored::PlayerExplored;
use component::field_of_view::FieldOfView;
use component::name::Name;
use component::occupant::Occupant;
use component::opaque::Opaque;
use component::player::Player;
use component::position::Position;
use component::renderable::*;
use component::tile::Tile;
use crate::entity;
use entity::player::get_player;
use crate::map;
use map::generator::algorithm::Algorithm;
use crate::resource;
use resource::continue_flag::ContinueFlagResource;
use resource::input_domain::InputDomainResource;
use resource::map_console::MapConsoleResource;
use resource::player_input::PlayerInputResource;
use resource::root_console::RootConsoleResource;
use resource::seed::SeedResource;
use crate::system;
use system::actor_feeder::ActorFeederSystem;
use system::field_of_view::FieldOfViewSystem;
use system::map_renderer::MapRendererSystem;
use system::player_explored_marker::PlayerExploredMarkerSystem;
use system::player_input::PlayerInputSystem;
use crate::ui::*;


/// Extensions for the World.
pub trait WorldExtension {

    /// Should we continue in the main loop?
    fn should_continue(&self) -> bool;

    /// Handle input from the user.
    fn wait_for_keypress(&self);

}

/// Extensions for the World.
impl WorldExtension for World {

    /// Should we continue in the main loop?
    fn should_continue(&self) -> bool {
        trace!("Entering World::should_continue().");
        let continue_flag = self.read_resource::<ContinueFlagResource>().0;
        let window_closed = (self.read_resource::<RootConsoleResource>().0)
            .lock()
            .unwrap()
            .window_closed();
        trace!("Exiting World::should_continue().");
        return continue_flag && !window_closed;
    }

    /// Handle input from the user.
    fn wait_for_keypress(&self) {
        trace!("Entering World::wait_for_keypress().");
        let key = (self.read_resource::<RootConsoleResource>().0)
            .lock()
            .unwrap()
            .wait_for_keypress(true);
        trace!("Exiting World::wait_for_keypress().");
        let mut player_input = self.write_resource::<PlayerInputResource>();
        player_input.0 = Some(Event::Key(key));
    }

}

/// Run the game loop.
pub fn run_game_loop() {
    let mut world = World::new();
    let screen_height = 100;
    let screen_width = 160;
    let map_width = 160;
    let map_height = screen_height - 5;
    let root_console = get_root_console(screen_width, screen_height);
    let map_console = get_map_console(map_width, map_height);
    let seed: i64 = 0;
    world.register::<Actor>();
    world.register::<Agent>();
    world.register::<CommandQueue>();
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
    world.add_resource(InputDomainResource::default());
    world.add_resource(MapConsoleResource(Arc::new(Mutex::new(map_console))));
    world.add_resource(PlayerInputResource::default());
    world.add_resource(RootConsoleResource(Arc::new(Mutex::new(root_console))));
    world.add_resource(SeedResource(seed));
    let starting_position = Algorithm::Simple.generate_map(&mut world, map_width, map_height, seed);
    let _player = get_player(&mut world, starting_position.0, starting_position.1, seed);
    let mut frame_dispatcher = DispatcherBuilder::new()
        .with(PlayerInputSystem, "player_input", &[])
        .with(MapRendererSystem, "map_renderer", &[])
        .build();
    frame_dispatcher.setup(&mut world.res);
    let mut tick_dispatcher = DispatcherBuilder::new()
        .with(FieldOfViewSystem, "field_of_view", &[])
        .with(ActorFeederSystem, "actor_feeder", &[])
        .build();
    tick_dispatcher.setup(&mut world.res);
    let mut idle_dispatcher = DispatcherBuilder::new()
        .build();
    idle_dispatcher.setup(&mut world.res);
    let mut turn_dispatcher = DispatcherBuilder::new()
        .with(PlayerExploredMarkerSystem, "player_explored_marker", &[])
        .build();
    turn_dispatcher.setup(&mut world.res);
    while world.should_continue() {

        idle_dispatcher.dispatch(&mut world.res);   // After a loop in which anything or nothing happened.
        frame_dispatcher.dispatch(&mut world.res);  // After a loop in which anything happened.
        tick_dispatcher.dispatch(&mut world.res);   // After each tick in which anything acted.
        turn_dispatcher.dispatch(&mut world.res);   // After a loop in which the character acted.
        world.wait_for_keypress();
    }
}
