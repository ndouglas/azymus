use tcod::console::*;
use crate::settings;
use settings::Settings;

/// Creates a messages console with the specified settings.
pub fn get_messages_console(settings: &Settings) -> Offscreen {
    let display_settings = &settings.display;
    let map_settings = &settings.map;
    let messages_console = Offscreen::new(display_settings.width, display_settings.height - map_settings.height);
    messages_console
}
