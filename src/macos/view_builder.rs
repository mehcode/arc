use super::view::Size;
use super::super::color::Color;

#[derive(Debug, Default)]
pub struct ViewBuilder {
    pub(crate) background_color: Color,
    pub(crate) width: Size,
    pub(crate) height: Size,
}

impl ViewBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn width(self, width: f64) -> Self {
        Self {
            width: Size::Absolute(width),
            ..self
        }
    }

    pub fn height(self, height: f64) -> Self {
        Self {
            height: Size::Absolute(height),
            ..self
        }
    }

    pub fn background_color<T: Into<Color>>(self, color: T) -> Self {
        Self {
            background_color: color.into(),
            ..self
        }
    }
}
