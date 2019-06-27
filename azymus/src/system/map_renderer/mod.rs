use specs::*;
use crate::component;
use component::position::Position;
use component::renderable::Renderable;
use component::renderable::*;
use crate::resource;
use resource::map_console::MapConsoleResource;
use resource::root_console::RootConsoleResource;
use tcod::console::*;
use tcod::colors::*;
use std::ops::DerefMut;

/// Renderer.
#[derive(Clone, Copy, Debug)]
pub struct MapRendererSystem;

/// Renderer.
impl<'a> System<'a> for MapRendererSystem {

    type SystemData = (
        WriteExpect<'a, MapConsoleResource>,
        WriteExpect<'a, RootConsoleResource>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        trace!("Exiting MapRendererSystem::run().");
        let (
            mut map_console_resource,
            mut root_console_resource,
            position_storage,
            renderable_storage,
        ) = data;
        let map_console = &mut (map_console_resource.0).lock().unwrap();
        let width = map_console.width();
        let height = map_console.height();
        map_console.set_default_foreground(WHITE);
        map_console.clear();
        for (position, renderable) in (&position_storage, &renderable_storage).join() {
            trace!("Processing renderable at ({}, {})...", position.x, position.y);
            map_console.render_renderable(position.x, position.y, renderable);
        }
        let root_console = &mut (root_console_resource.0).lock().unwrap();
        blit(
            map_console.deref_mut(),
            (0, 0),
            (width, height),
            root_console.deref_mut(),
            (0, 0),
            1.0,
            1.0,
        );
        root_console.flush();
        tcod::system::set_fps(60);
        trace!("Exiting MapRendererSystem::run().");
    }

}
