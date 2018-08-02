use cocoa::base::id;
use crate::{
    os::{
        macos::{node::Node, sys},
        Font,
    },
    Color, Gravity,
};
use objc::{msg_send, sel, sel_impl};

pub(crate) struct Text(pub(crate) id);

// NOTE: In order to send references of this packed in Context to different threads.
//       It's very unsafe to touch these unless on the "main" thread but Context ensures
//       public access is only allowed on main thread.
unsafe impl Send for Text {}

impl Text {
    #[inline]
    pub(crate) fn new() -> Self {
        Text(unsafe { msg_send![*sys::text::CLASS, new] })
    }

    #[inline]
    pub(crate) fn set_text(&mut self, text: &str) {
        sys::text::set_text(self.0, text);
    }

    //
    // Layout
    //

    #[inline]
    pub(crate) fn set_gravity(&mut self, gravity: Gravity) {
        sys::text::set_gravity(self.0, gravity);
    }

    //
    // Style: Text
    //

    #[inline]
    pub(crate) fn set_text_size(&mut self, size: f32) {
        sys::text::set_text_size(self.0, size);
    }

    #[inline]
    pub(crate) fn set_text_color(&mut self, color: Color) {
        sys::text::set_text_color(self.0, color);
    }

    //
    // Style: Font
    //

    #[inline]
    pub(crate) fn set_font(&mut self, font: &Font) {
        sys::text::set_font(self.0, font);
    }

    //
    // Style: View
    //

    #[inline]
    pub(crate) fn set_background_color(&mut self, color: Color) {
        sys::view::set_background_color(self.0, color);
    }

    #[inline]
    pub(crate) fn set_corner_radius(&mut self, radius: f32) {
        sys::view::set_corner_radius(self.0, radius);
    }
}

impl Node for Text {
    #[inline]
    fn handle(&self) -> id {
        self.0
    }
}
