use super::{
    events, os, Align, Color, Context, Edge, Event, Font, Gravity, Node, NodeId, PositionType,
};
use std::mem::transmute;
use yoga;

#[derive(Copy, Clone)]
pub struct Text {
    pub(crate) inner: os::NodeId,
}

impl Text {
    pub fn new() -> Self {
        Self {
            inner: os::Nodes::emplace(os::Text::new()),
        }
    }

    #[inline]
    pub fn set_text(&mut self, text: impl AsRef<str> + Send) {
        os::Nodes::with(self.inner, move |node: &mut os::Text| {
            node.set_text(text.as_ref());
        })
    }

    //
    // Layout
    //

    #[inline]
    pub fn set_gravity(&mut self, gravity: Gravity) {
        os::Nodes::with(self.inner, move |node: &mut os::Text| {
            node.set_gravity(gravity);
        })
    }

    //
    // Style
    //

    /// Sets the background color for this node.
    ///
    /// Default: `transparent` (`0x00_00_00_00`)
    #[inline]
    pub fn set_background_color(&mut self, color: impl Into<Color>) {
        let color = color.into();

        os::Nodes::with(self.inner, move |node: &mut os::Text| {
            node.set_background_color(color);
        });
    }

    /// Sets the corner radius for this node.
    ///
    /// Default: `0`
    #[inline]
    pub fn set_corner_radius(&mut self, radius: f32) {
        os::Nodes::with(self.inner, move |node: &mut os::Text| {
            node.set_corner_radius(radius);
        });
    }

    //
    // Style: Text
    //

    #[inline]
    pub fn set_text_color(&mut self, color: impl Into<Color>) {
        let color = color.into();

        os::Nodes::with(self.inner, move |node: &mut os::Text| {
            node.set_text_color(color);
        });
    }

    #[inline]
    pub fn set_text_size(&mut self, size: f32) {
        os::Nodes::with(self.inner, move |node: &mut os::Text| {
            node.set_text_size(size);
        });
    }

    //
    // Style: Font
    //

    #[inline]
    pub fn set_font(&mut self, font: &Font) {
        // FIXME: Find a better way to do this. Most likely removing this method so
        //        don't think on it too long.
        let font: usize = unsafe { transmute(font) };
        os::Nodes::with(self.inner, move |node: &mut os::Text| {
            let font: &Font = unsafe { transmute(font) };
            node.set_font(&font.inner);
        });
    }

    //
    // Events
    // TODO: Documentation
    //

    #[inline]
    pub fn mouse_down(&mut self) -> Event<events::MouseDown> {
        os::Nodes::with(self.inner, move |node: &mut os::Text| {
            node.mouse_down().clone()
        })
    }

    #[inline]
    pub fn mouse_up(&mut self) -> Event<events::MouseUp> {
        os::Nodes::with(self.inner, move |node: &mut os::Text| {
            node.mouse_up().clone()
        })
    }

    #[inline]
    pub fn mouse_enter(&mut self) -> Event<events::MouseEnter> {
        os::Nodes::with(self.inner, move |node: &mut os::Text| {
            node.mouse_enter().clone()
        })
    }

    #[inline]
    pub fn mouse_leave(&mut self) -> Event<events::MouseLeave> {
        os::Nodes::with(self.inner, move |node: &mut os::Text| {
            node.mouse_leave().clone()
        })
    }
}

//
// Node
//

impl Node for Text {
    #[inline]
    fn id(&self) -> NodeId {
        self.inner
    }
}

//
// Layout
//

impl_layout!(Text);
