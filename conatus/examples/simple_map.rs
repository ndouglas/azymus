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
    let screen_height = 100;
    let screen_width = 160;
    let map_width = 160;
    let map_height = screen_height - 5;
    let mut root_console = conatus::console::get_root_console(screen_width, screen_height);
    let mut map_console = conatus::console::get_map_console(map_width, map_height);
    let mut world = World::new();
    world.register::<Occupant>();
    world.register::<Opaque>();
    world.register::<Position>();
    world.register::<Renderable>();
    world.add_resource(MapConsoleResource(Arc::new(Mutex::new(map_console))));
    world.add_resource(RootConsoleResource(Arc::new(Mutex::new(root_console))));
    world.add_resource(ContinueFlagResource::default());
    let mut position = Position {
        x: map_width / 2,
        y: map_height / 2,
    };
    for y in 0..map_height {
        for x in 0..map_width {
            let is_wall = y == map_height / 2 && ( x == map_width / 3 || x == map_width * 2 / 3 );
            let color = if is_wall {
                DARK_BLUE
            } else {
                LIGHT_BLUE
            };
            if is_wall {
                world.create_entity()
                    .with(Occupant)
                    .with(Position {
                        x: x,
                        y: y,
                    })
                    .with(Renderable {
                        char: None,
                        foreground_color: None,
                        background_color: Some(color),
                    })
                    .build();
            } else {
                world.create_entity()
                    .with(Position {
                        x: x,
                        y: y,
                    })
                    .with(Renderable {
                        char: None,
                        foreground_color: None,
                        background_color: Some(color),
                    })
                    .build();
            }
        }
    }
    let player = world.create_entity()
        .with(position)
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
