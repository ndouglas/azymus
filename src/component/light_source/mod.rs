use std::fmt;
use tcod::colors::*;

/// Something that gives off light.
#[derive(Clone)]
pub struct LightSource {
    /// The color of light produced.
    pub color: Color,
    /// Radius of lighted area.
    pub radius: i32,
    /// The intensity of the light produced.
    pub intensity: i32,
    /// The coefficient used to calculate effective intensity.
    pub coefficient: f64,
}

/// Something that gives off light.
impl LightSource {

    /// Constructor.
    pub fn new() -> LightSource {
        let color = RED;
        let radius = 10;
        let intensity = 96;
        LightSource {
            color: color,
            radius: radius,
            intensity: intensity,
            coefficient: - (intensity as f64) / (radius as f64),
        }
    }

    /// Compute intensity of light at a distance.
    pub fn intensity_at(&self, dx: i32, dy: i32) -> i32 {
        if dx > self.radius || dy > self.radius {
            0
        } else {
            let distance = ((dx as f64).powi(2) + (dy as f64).powi(2)).sqrt();
            let result: i32 = (self.intensity as f64 + distance * self.coefficient) as i32;
            println!("{}", result);
            result
        }
    }

}

/// Allows us to show this object in tests, etc.
impl fmt::Debug for LightSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LightSource")
    }
}
