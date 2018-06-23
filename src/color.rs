use palette::Srgba;

pub struct Color {
    pub(crate) inner: Srgba<f64>,
}

impl From<u32> for Color {
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
