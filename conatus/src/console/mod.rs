use azymus::action::*;
use azymus::command::*;
use azymus::input::*;
use azymus::world::*;
use specs::*;
use tcod::console::*;

/// Create a root console with a specified height and width.
pub fn get_root_console(width: i32, height: i32) -> Root {
    let root = Root::initializer()
        .font("resources/tcod/fonts/arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(width, height)
        .title("Conatus")
        .init();
    root
}

/// Create a map console with a specified height and width.
pub fn get_map_console(width: i32, height: i32) -> Offscreen {
    let map = Offscreen::new(width, height);
    map
}

/// Handle input.
pub fn handle_keys(entity: Entity, world: &mut World) -> bool {
    let event = world.wait_for_keypress();
    if let Some(command) = get_event_command(Domain::Explore, event) {
        if let Some(action) = get_command_action(command, entity, world) {
            if let Some(action) = get_permitted_action(action, entity, world) {
                action.execute(entity, world);
            }
        }
    }
   false
}
