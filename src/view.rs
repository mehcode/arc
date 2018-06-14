use super::{os, Color, FlexDirection};

pub struct View {
    pub(crate) inner: os::View,
}

impl View {
    pub fn new() -> Self {
        Self {
            inner: os::View::new(),
        }
    }

    pub fn add_child(&mut self, child: View) {
        self.inner.add_child(child.inner);
    }

    pub fn set_width(&mut self, width: f32) {
        self.inner.set_width(width);
    }

    pub fn set_height(&mut self, height: f32) {
        self.inner.set_height(height);
    }

    pub fn set_flex_grow(&mut self, flex_grow: f32) {
        self.inner.set_flex_grow(flex_grow);
    }

    pub fn set_flex_direction(&mut self, flex_direction: FlexDirection) {
        self.inner.set_flex_direction(flex_direction);
    }

    pub fn set_background_color(&mut self, color: impl Into<Color>) {
        self.inner.set_background_color(color.into());
    }
}
