use crate::settings;
use settings::Settings;
use crate::tile;
use tile::Tile;

/// The map type.
pub type MapType = Vec<Vec<Tile>>;

/// The map object.
#[derive(Clone, Debug)]
pub struct Map {
    /// The actual inner map.
    map: MapType,
}

/// The map object.
impl Map {

    /// Constructor.
    pub fn new(settings: &Settings) -> Self {
        let map_settings = &settings.map;
        Map {
            map: vec![vec![Tile::new(); map_settings.height as usize]; map_settings.width as usize],
        }
    }

}
