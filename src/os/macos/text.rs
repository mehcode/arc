use super::{node::Node, sys};
use cocoa::base::id;
use color::Color;

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
    // Style: Text
    //

    #[inline]
    pub(crate) fn set_text_size(&mut self, size: f32) {
        sys::text::set_font_size(self.0, size);
    }

    #[inline]
    pub(crate) fn set_text_color(&mut self, color: Color) {
        sys::text::set_text_color(self.0, color);
    }

    //
    // Style: Font
    //

    #[inline]
    pub(crate) fn set_font_family(&mut self, family: &str) {
        sys::text::set_font_family(self.0, family);
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
