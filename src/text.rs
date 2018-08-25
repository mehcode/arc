use super::{context::emplace, events, os, Color, Event, Font, Gravity, Node, NodeId};
use std::mem::transmute;

#[derive(Copy, Clone)]
pub struct Text {
    pub(crate) id: NodeId,
}

impl Text {
    pub fn new() -> Self {
        Self {
            id: emplace(os::Text::new()),
        }
    }

    #[inline]
    pub fn set_text(&mut self, text: impl AsRef<str> + Send) {
        self.id.with_mut::<os::Text, _, _>(move |node| {
            node.set_text(text.as_ref());
        })
    }

    //
    // Layout
    //

    #[inline]
    pub fn set_gravity(&mut self, gravity: Gravity) {
        self.id.with_mut::<os::Text, _, _>(move |node| {
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

        self.id.with_mut::<os::Text, _, _>(move |node| {
            node.set_background_color(color);
        });
    }

    /// Sets the corner radius for this node.
    ///
    /// Default: `0`
    #[inline]
    pub fn set_corner_radius(&mut self, radius: f32) {
        self.id.with_mut::<os::Text, _, _>(move |node| {
            node.set_corner_radius(radius);
        });
    }

    //
    // Style: Text
    //

    #[inline]
    pub fn set_text_color(&mut self, color: impl Into<Color>) {
        let color = color.into();

        self.id.with_mut::<os::Text, _, _>(move |node| {
            node.set_text_color(color);
        });
    }

    #[inline]
    pub fn set_text_size(&mut self, size: f32) {
        self.id.with_mut::<os::Text, _, _>(move |node| {
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
        self.id.with_mut::<os::Text, _, _>(move |node| {
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
        self.id
            .with_mut::<os::Text, _, _>(move |node| node.mouse_down().clone())
    }

    #[inline]
    pub fn mouse_up(&mut self) -> Event<events::MouseUp> {
        self.id
            .with_mut::<os::Text, _, _>(move |node| node.mouse_up().clone())
    }

    #[inline]
    pub fn mouse_enter(&mut self) -> Event<events::MouseEnter> {
        self.id
            .with_mut::<os::Text, _, _>(move |node| node.mouse_enter().clone())
    }

    #[inline]
    pub fn mouse_leave(&mut self) -> Event<events::MouseLeave> {
        self.id
            .with_mut::<os::Text, _, _>(move |node| node.mouse_leave().clone())
    }
}

//
// Node
//

impl Node for Text {
    #[inline]
    fn id(&self) -> NodeId {
        self.id
    }
}

//
// Layout
//

impl_layout!(Text);
