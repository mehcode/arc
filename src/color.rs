use failure::err_msg;
use palette;
use palette::Srgb;
use palette::rgb::{Rgb, RgbStandard};

#[derive(Default)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl<'a> From<&'a str> for Color {
    fn from(name: &'a str) -> Self {
        // FIXME: This should be TryFrom and use ?
        let color = palette::named::from_str(name)
            .ok_or_else(|| err_msg(format!("unknown color: {:?}", name)))
            .unwrap();

        Srgb::from_pixel(&color).into_linear().into()
    }
}

impl From<u32> for Color {
    fn from(rgb: u32) -> Self {
        let r = (rgb >> 16) as u8;
        let g = (rgb >> 8) as u8;
        let b = rgb as u8;

        Srgb::new_u8(r, g, b).into()
    }
}

impl<S: RgbStandard> From<Rgb<S, f32>> for Color {
    fn from(color: Rgb<S, f32>) -> Self {
        Self {
            r: color.red as f64,
            g: color.green as f64,
            b: color.blue as f64,
            a: 1.0,
        }
    }
}
