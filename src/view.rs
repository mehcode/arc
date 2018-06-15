use super::{os, Align, Color, Edge, FlexDirection, Justify};

/// The fundamental component, `View` is a container that supports
/// layout with **Flexbox** powered by [Yoga](https://yogalayout.com/). View maps directly
/// to the native view equivalent of the platform (e.g.. `NSView` for macOS).
///
/// `View` is designed to be nested inside other views and can have 0 to many children of
/// any type.
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

    pub fn set_background_color(&mut self, color: impl Into<Color>) {
        self.inner.set_background_color(color.into());
    }

    pub fn set_width(&mut self, width: f32) {
        self.inner.set_width(width);
    }

    pub fn set_width_percent(&mut self, width: f32) {
        self.inner.set_width_percent(width * 100.);
    }

    pub fn set_height(&mut self, height: f32) {
        self.inner.set_height(height);
    }

    pub fn set_height_percent(&mut self, height: f32) {
        self.inner.set_height_percent(height * 100.);
    }

    pub fn set_flex_grow(&mut self, flex_grow: f32) {
        self.inner.set_flex_grow(flex_grow);
    }

    pub fn set_align_items(&mut self, align: Align) {
        self.inner.set_align_items(align);
    }

    pub fn set_justify_content(&mut self, justify: Justify) {
        self.inner.set_justify_content(justify);
    }

    pub fn set_flex_direction(&mut self, flex_direction: FlexDirection) {
        self.inner.set_flex_direction(flex_direction);
    }

    pub fn set_padding(&mut self, edge: Edge, padding: f32) {
        self.inner.set_padding(edge, padding);
    }

    pub fn set_margin(&mut self, edge: Edge, margin: f32) {
        self.inner.set_margin(edge, margin);
    }
}
