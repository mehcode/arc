use palette::Srgba;

pub struct Color {
    pub(crate) inner: Srgba<f32>,
}

impl<'a> From<u32> for Color {
    fn from(argb: u32) -> Color {
        let a = (argb >> 24) as u8;
        let r = (argb >> 16) as u8;
        let g = (argb >> 8) as u8;
        let b = argb as u8;

        Color {
            inner: Srgba::new(r, g, b, a).into_format(),
        }
    }
}

impl<'a> From<&'a [f32; 3]> for Color {
    fn from(rgb: &'a [f32; 3]) -> Color {
        Color {
            inner: Srgba::new(rgb[0], rgb[1], rgb[2], 1.).into_format(),
        }
    }
}

impl<'a> From<&'a [f32; 4]> for Color {
    fn from(argb: &'a [f32; 4]) -> Color {
        Color {
            inner: Srgba::new(argb[1], argb[2], argb[3], argb[0]).into_format(),
        }
    }
}
