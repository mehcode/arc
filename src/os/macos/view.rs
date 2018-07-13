use super::{
    super::super::{Color, Event, MouseDown, MouseEnter, MouseLeave, MouseUp},
    node::yoga_from_handle,
    sys::view,
    Node,
};
use cocoa::base::id;
use std::os::raw::c_void;

// TODO: Investigate ways to make this file more "safe"

pub(crate) struct View(pub(crate) id);

// NOTE: In order to send references of this packed in Context to different threads.
//       It's very unsafe to touch these unless on the "main" thread but Context ensures
//       public access is only allowed on main thread.
unsafe impl Send for View {}

impl View {
    pub(crate) fn new() -> Self {
        // Allocate and initialize an empty, native view
        let view: id = unsafe { msg_send![*view::CLASS, new] };

        View(view)
    }

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

    pub(crate) fn set_background_color(&mut self, color: Color) {
        view::set_background_color(self.0, color);
    }

    pub(crate) fn set_corner_radius(&mut self, radius: f32) {
        unsafe {
            (*self.0).set_ivar("sqCornerRadius", f64::from(radius));
        }
    }

    //
    // Events
    //

    pub(crate) fn mouse_down(&mut self) -> &mut Event<MouseDown> {
        unsafe { &mut *(*(*self.0).get_mut_ivar::<*mut c_void>("sqEVTMouseDown") as *mut _) }
    }

    pub(crate) fn mouse_up(&mut self) -> &mut Event<MouseUp> {
        unsafe { &mut *(*(*self.0).get_mut_ivar::<*mut c_void>("sqEVTMouseUp") as *mut _) }
    }

    pub(crate) fn mouse_enter(&mut self) -> &mut Event<MouseEnter> {
        unsafe { &mut *(*(*self.0).get_mut_ivar::<*mut c_void>("sqEVTMouseEnter") as *mut _) }
    }

    pub(crate) fn mouse_leave(&mut self) -> &mut Event<MouseLeave> {
        unsafe { &mut *(*(*self.0).get_mut_ivar::<*mut c_void>("sqEVTMouseLeave") as *mut _) }
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
    fn handle(&self) -> id {
        self.0
    }
}
