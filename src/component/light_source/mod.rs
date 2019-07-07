use std::cmp;
use bear_lib_terminal::Color;

/// Something that gives off light.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct LightSource {
    /// The color of light produced.
    pub color: Color,
    /// Radius of lighted area.
    pub radius: i32,
    /// The intensity of the light produced.
    pub intensity: i32,
}

/// Something that gives off light.
impl LightSource {

    /// Constructor.
    pub fn new(color: Color, radius: i32, intensity: i32) -> LightSource {
        trace!("Entering LightSource::new().");
        LightSource {
            color: color,
            radius: radius,
            intensity: intensity,
        }
    }

    /// Compute intensity of light at a distance.
    pub fn intensity_at(&self, x: i32, y: i32, x2: i32, y2: i32) -> i32 {
        trace!("Entering LightSource::intensity_at().");
        let dx = (x2 - x).abs();
        let dy = (y2 - y).abs();
        if dx > self.radius || dy > self.radius {
            0
        } else {
            let distance = ((dx as f64).powi(2) + (dy as f64).powi(2)).sqrt();
            let coefficient = - (self.intensity as f64) / (self.radius as f64);
            let result: i32 = (self.intensity as f64 + distance * coefficient) as i32;
            let result = cmp::max(result, 0);
            result
        }
    }

    /// Transform a color at a specified distance.
    pub fn transform_color_at(&self, color: Color, x: i32, y: i32, x2: i32, y2: i32) -> Color {
        trace!("Entering LightSource::transform_color_at().");
        let intensity = self.intensity_at(x, y, x2, y2);
        let multiplier = intensity as f64 / 512 as f64;
        let red = color.red;
        let green = color.green;
        let blue = color.blue;
        let r_diff = (self.color.red as i32 - red as i32).abs();
        let g_diff = (self.color.green as i32 - green as i32).abs();
        let b_diff = (self.color.blue as i32 - blue as i32).abs();
        let new_r = (red as f64 + (r_diff as f64 * multiplier)) as u8;
        let new_g = (green as f64 + (g_diff as f64 * multiplier)) as u8;
        let new_b = (blue as f64 + (b_diff as f64 * multiplier)) as u8;
        trace!("Exiting LightSource::transform_color_at().");
        Color::from_rgb(new_r, new_g, new_b)
    }

}

/// A factory.
#[derive(Clone, Copy, Debug)]
pub enum Factory {
    /// A candle provides very little light.
    Candle,
    /// A torch provides more and stronger light.
    Torch,
}

/// A factory.
impl Factory {

    /// Creates a light source.
    pub fn create(self) -> LightSource {
        use Factory::*;
        match self {
            Candle => LightSource::new(Color::from_rgb(255, 127, 255), 6, 64),
            Torch => LightSource::new(Color::from_rgb(255, 127, 0), 10, 96),
        }
    }

}
