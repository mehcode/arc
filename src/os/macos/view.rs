use super::{node::yoga_from_handle, sys, Node};
use crate::{Color, Event, MouseDown, MouseEnter, MouseLeave, MouseUp};
use objc::{msg_send, runtime::Object, sel, sel_impl};

pub(crate) struct View(pub(crate) *mut Object);

// NOTE: In order to send references of this packed in Context to different threads.
//       It's very unsafe to touch these unless on the "main" thread but Context ensures
//       public access is only allowed on main thread.
unsafe impl Send for View {}

impl View {
    pub(crate) fn new() -> Self {
        View(unsafe { msg_send![*sys::view::CLASS, new] })
    }

    //
    // Container
    //

    pub(crate) fn add(&mut self, node: &impl Node) {
        let this_node = self.yoga();
        let this_node_children_count = this_node.child_count();

        let incoming_handle = node.handle();
        let incoming_node = yoga_from_handle(incoming_handle);

        // Add this node as a sub-node (in layout)
        this_node.insert_child(incoming_node, this_node_children_count);

        // Add the view as a sub-view (in ui)
        unsafe {
            msg_send![self.0, addSubview: incoming_handle];
        }
    }

    //
    // Style
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
    pub(crate) fn mouse_down(&mut self) -> &mut Event<MouseDown> {
        sys::event(self.0, "sqEVTMouseDown")
    }

    #[inline]
    pub(crate) fn mouse_up(&mut self) -> &mut Event<MouseUp> {
        sys::event(self.0, "sqEVTMouseUp")
    }

    #[inline]
    pub(crate) fn mouse_enter(&mut self) -> &mut Event<MouseEnter> {
        sys::event(self.0, "sqEVTMouseEnter")
    }

    #[inline]
    pub(crate) fn mouse_leave(&mut self) -> &mut Event<MouseLeave> {
        sys::event(self.0, "sqEVTMouseLeave")
    }
}

impl Drop for View {
    fn drop(&mut self) {
        unsafe {
            msg_send![self.0, release];
        }
    }
}

impl Node for View {
    fn handle(&self) -> *mut Object {
        self.0
    }
}
