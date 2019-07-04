use tcod::console::*;
use tcod::system::set_fps;
use crate::settings;
use settings::Settings;

/// Creates a root console with the specified settings.
pub fn get_root_console(settings: &Settings) -> Root {
    let display_settings = &settings.display;
    let font_file_path = display_settings.get_font_file_path();
    let root_console = Root::initializer()
        .font(font_file_path, FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(display_settings.width, display_settings.height)
        .title(&display_settings.title)
        .init();
    set_fps(display_settings.fps_limit);
    root_console
}
