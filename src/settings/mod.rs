use std::io::{Read, Write};
use std::fs::File;
use std::error::Error;
use std::fmt;

/// Display Settings.
pub mod display;
use display::Display;

/// Map Settings.
pub mod map;
use map::Map;

/// Scheduler Settings.
pub mod scheduler;
use scheduler::Scheduler;

/// The filename.
pub const SETTINGS_FILENAME: &str = "settings.json";

/// Settings.
#[derive(Clone, Deserialize, Serialize)]
pub struct Settings {
    /// Display settings.
    pub display: Display,
    /// Map settings.
    pub map: Map,
    /// Scheduler settings.
    pub scheduler: Scheduler,
}

/// Settings.
impl Settings {

    /// Constructor.
    pub fn new() -> Self {
        Settings {
            display: Display::new(),
            map: Map::new(),
            scheduler: Scheduler::new(),
        }
    }

    /// Save the settings.
    pub fn save(&self) -> Result<(), Box<Error>> {
        let data = serde_json::to_string(self)?;
        let mut file = File::create(SETTINGS_FILENAME.to_string())?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

}

/// Load the settings.
pub fn load() -> Result<Settings, Box<Error>> {
    let mut data = String::new();
    let mut file = File::open(SETTINGS_FILENAME.to_string())?;
    file.read_to_string(&mut data)?;
    let result = serde_json::from_str::<Settings>(&data)?;
    Ok(result)
}

/// Load the settings.
pub fn get_settings() -> Settings {
    let settings = match load() {
        Ok(settings) => settings,
        Err(_) => {
            let settings = Settings::new();
            match settings.save() {
                Ok(_) => settings,
                Err(error) => panic!("Error: {}", error),
            }
        },
    };
    settings
}


/// Allows us to show this object in tests, etc.
impl fmt::Debug for Settings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Settings")
    }
}

#[cfg(test)]
mod tests {

}
