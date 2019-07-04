use tcod::console::*;
use crate::settings;
use settings::Settings;

/// Creates a map console with the specified settings.
pub fn get_map_console(settings: &Settings) -> Offscreen {
    let map_settings = &settings.map;
    let map_console = Offscreen::new(map_settings.width, map_settings.height);
    map_console
}
