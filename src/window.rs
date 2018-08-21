use crate::{os, Color, Context, Node};

pub struct Window {
    pub(crate) inner: os::Window,
}

impl Window {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            inner: os::Window::new(width, height),
        }
    }

    /// Moves the window to the front and shows the window.
    #[inline]
    pub fn show(&mut self) {
        self.inner.show();
    }

    /// Sets whether the window should be resizable.
    ///
    /// Default: `true`.
    #[inline]
    pub fn set_resizable(&mut self, resizable: bool) {
        self.inner.set_resizable(resizable);
    }

    /// Sets the title of the window.
    ///
    /// Default: `""`
    #[inline]
    pub fn set_title(&mut self, title: impl AsRef<str>) {
        self.inner.set_title(title.as_ref());
    }

    /// Sets the background color of the window.
    ///
    /// Default: OS (and window manager) specific. For example, in macOS it would be `0xff_e6e6e6`.
    #[inline]
    pub fn set_background_color(&mut self, color: impl Into<Color>) {
        self.inner.set_background_color(color.into());
    }

    /// Set the root view of the window.
    #[inline]
    pub fn set_view(&mut self, node: impl Node + 'static) {
        self.inner.set_view(node.id());
    }
}
