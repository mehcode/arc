use super::{os, Color, View};

pub struct Window {
    pub(crate) inner: os::Window,
}

impl Window {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            inner: os::Window::new(width, height),
        }
    }

    /// Sets whether the window should be resizable.
    pub fn set_resizable(&mut self, resizable: bool) {
        self.inner.set_resizable(resizable);
    }

    /// Sets the title of the window.
    pub fn set_title(&mut self, title: impl AsRef<str>) {
        self.inner.set_title(title.as_ref());
    }

    /// Sets the background color of the window.
    pub fn set_background_color(&mut self, color: impl Into<Color>) {
        self.inner.set_background_color(color.into());
    }

    /// Set the root view of the window.
    pub fn set_view(&mut self, view: View) {
        self.inner.set_view(view.inner);
    }
}
