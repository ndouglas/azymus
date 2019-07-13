use tcod::colors::Color as TcodColor;
use bear_lib_terminal::Color as BltColor;

/// The color structure.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Color {
    /// Red.
    pub red: u8,
    /// Green.
    pub green: u8,
    /// Blue.
    pub blue: u8,
    /// Alpha.
    pub alpha: u8,
}

/// The color structure.
impl Color {

    /// RGB with full opacity.
    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Color {
            red: red,
            green: green,
            blue: blue,
            alpha: 255,
        }
    }

    /// RGB with specified capacity.
    pub fn from_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Color {
            red: red,
            green: green,
            blue: blue,
            alpha: alpha,
        }
    }

    /// From Tcod color.
    pub fn from_tcod(color: &TcodColor) -> Self {
        Color {
            red: color.r,
            green: color.g,
            blue: color.b,
            alpha: 255,
        }
    }

    /// From BearLibTerminal color.
    pub fn from_blt(color: &BltColor) -> Self {
        Color {
            red: color.red,
            green: color.green,
            blue: color.blue,
            alpha: color.alpha,
        }
    }

    /// To Tcod color.
    pub fn to_tcod(&self) -> TcodColor {
        TcodColor::new(self.red, self.green, self.blue)
    }

    /// To BearLibTerminal color.
    pub fn to_blt(&self) -> BltColor {
        BltColor::from_rgba(self.red, self.green, self.blue, self.alpha)
    }

}
