/// Stores color information and opacity (alpha value).
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Color {
    /// The red component value of the color.
    pub red: f32,

    /// The green component value of the color.
    pub green: f32,

    /// The blue component value of the color.
    pub blue: f32,

    /// The alpha (opacity) component value of the color.
    pub alpha: f32,
}

impl Default for Color {
    fn default() -> Self {
        Color::TRANSPARENT
    }
}

//
// Named Colors
//

impl Color {
    pub const BLACK: Color = Color {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 1.,
    };

    pub const WHITE: Color = Color {
        red: 1.,
        green: 1.,
        blue: 1.,
        alpha: 1.,
    };
    
    // TODO: Consider naming `CLEAR`
    pub const TRANSPARENT: Color = Color {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
    };
}

//
// Conversions
//

impl<'a> From<u32> for Color {
    fn from(argb: u32) -> Color {
        let a = (argb >> 24) as u8;
        let r = (argb >> 16) as u8;
        let g = (argb >> 8) as u8;
        let b = argb as u8;

        Color {
            red: f32::from(r) / 255.,
            blue: f32::from(b) / 255.,
            green: f32::from(g) / 255.,
            alpha: f32::from(a) / 255.,
        }
    }
}

impl<'a> From<&'a [f32; 3]> for Color {
    fn from(rgb: &'a [f32; 3]) -> Color {
        Color {
            red: rgb[0],
            green: rgb[1],
            blue: rgb[2],
            alpha: 1.,
        }
    }
}

impl<'a> From<&'a [f32; 4]> for Color {
    fn from(argb: &'a [f32; 4]) -> Color {
        Color {
            red: argb[1],
            green: argb[2],
            blue: argb[3],
            alpha: argb[0],
        }
    }
}
