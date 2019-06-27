use std::sync::{Arc,Mutex};

/// An experimental roguelike (library), written in Rust.
extern crate azymus;
use azymus::component::occupant::Occupant;
use azymus::component::opaque::Opaque;
use azymus::component::position::Position;
use azymus::component::renderable::*;
use azymus::resource::continue_flag::ContinueFlagResource;
use azymus::resource::map_console::MapConsoleResource;
use azymus::resource::root_console::RootConsoleResource;
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
use tcod::colors::*;
use tcod::console::*;

fn main() {
    pretty_env_logger::init();
    trace!("Entering main().");
    let root_console = conatus::console::get_root_console(160, 100);
    let map_console = conatus::console::get_map_console(160, 95);
    let mut world = World::new();
    world.register::<Occupant>();
    world.register::<Opaque>();
    world.register::<Position>();
    world.register::<Renderable>();
    world.add_resource(MapConsoleResource(Arc::new(Mutex::new(map_console))));
    world.add_resource(RootConsoleResource(Arc::new(Mutex::new(root_console))));
    world.add_resource(ContinueFlagResource::default());
    let position = Position {
        x: 80,
        y: 50,
    };
    let player = world.create_entity()
        .with(Position {
            x: 80,
            y: 50,
        })
        .with(Renderable {
            char: Some('@'),
            foreground_color: Some(WHITE),
            background_color: None,
        })
        .build();
    while world.should_continue() {
        (world.read_resource::<MapConsoleResource>().0)
            .lock().unwrap()
            .clear();
        for (position, renderable) in (&world.read_storage::<Position>(), &world.read_storage::<Renderable>()).join() {
            (world.read_resource::<MapConsoleResource>().0)
                .lock().unwrap()
                .render_renderable(position.x, position.y, renderable);
        }
        world.blit_map_console();
        (world.read_resource::<RootConsoleResource>().0)
            .lock().unwrap()
            .flush();
        let exit = conatus::console::handle_keys(player, &mut world);
        if exit {
            break
        }
    }
    trace!("Exiting main().");
}
