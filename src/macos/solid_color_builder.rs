use super::{SolidColor, View};
use super::super::Builder;
use super::super::color::Color;
use super::view::Size;
use super::view_builder::ViewBuilder;

#[derive(Debug)]
pub struct SolidColorBuilder {
    view_builder: ViewBuilder,
}

impl SolidColorBuilder {
    pub fn new<T: Into<Color>>(color: T) -> Self {
        Self {
            view_builder: ViewBuilder::new().background_color(color),
        }
    }

    // TODO: Macro (?) to add view builder methods

    pub fn width(self, width: f64) -> Self {
        Self {
            view_builder: self.view_builder.width(width),
        }
    }

    pub fn height(self, height: f64) -> Self {
        Self {
            view_builder: self.view_builder.height(height),
        }
    }
}

impl Builder<SolidColor> for SolidColorBuilder {
    fn build(self) -> SolidColor {
        let mut self_ = SolidColor::new(self.view_builder.background_color);

        // TODO: Macro (?) to apply view builder properties

        if let Size::Absolute(width) = self.view_builder.width {
            self_.set_width(width);
        }

        if let Size::Absolute(height) = self.view_builder.height {
            self_.set_height(height);
        }

        self_
    }
}
