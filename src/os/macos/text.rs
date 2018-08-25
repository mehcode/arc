use crate::{
    events,
    os::{
        macos::{node::Node, sys},
        Font,
    },
    Color, Event, Gravity,
};
use objc::{msg_send, runtime::Object, sel, sel_impl};

pub(crate) struct Text(pub(crate) *mut Object);

// Nodes are safe to send between threads as long as they are only accessed on the
// UI thread (which the public API should ensure).
unsafe impl Send for Text {}
unsafe impl Sync for Text {}

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

    //
    // Events
    //

    #[inline]
    pub(crate) fn mouse_down(&mut self) -> &mut Event<events::MouseDown> {
        sys::event(self.0, "sqEVTMouseDown")
    }

    #[inline]
    pub(crate) fn mouse_up(&mut self) -> &mut Event<events::MouseUp> {
        sys::event(self.0, "sqEVTMouseUp")
    }

    #[inline]
    pub(crate) fn mouse_enter(&mut self) -> &mut Event<events::MouseEnter> {
        sys::event(self.0, "sqEVTMouseEnter")
    }

    #[inline]
    pub(crate) fn mouse_leave(&mut self) -> &mut Event<events::MouseLeave> {
        sys::event(self.0, "sqEVTMouseLeave")
    }
}

impl Drop for Text {
    fn drop(&mut self) {
        unsafe {
            msg_send![self.0, release];
        }
    }
}

impl Node for Text {
    #[inline]
    fn handle(&self) -> *mut Object {
        self.0
    }
}
