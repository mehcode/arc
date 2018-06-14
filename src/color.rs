use palette::Srgb;

pub struct Color {
    pub(crate) inner: Srgb<f64>,
}

impl From<u32> for Color {
    fn from(rgb: u32) -> Color {
        let r = (rgb >> 16) as u8;
        let g = (rgb >> 8) as u8;
        let b = rgb as u8;

        Color {
            inner: Srgb::new(r, g, b).into_format(),
        }
    }
}
