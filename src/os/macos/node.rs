use cocoa::base::{id, YES};
use std::os::raw::c_void;
use yoga;

pub(crate) type NodeHandle = id;

// Accessor for a View's yoga node
pub(crate) fn yoga_from_handle(handle: id) -> &'static mut yoga::Node {
    unsafe { &mut *(*(*handle).get_mut_ivar::<*mut c_void>("sqYGNode") as *mut _) }
}

#[doc(hidden)]
pub trait Node {
    fn handle(&self) -> NodeHandle;

    fn yoga(&mut self) -> &mut yoga::Node {
        yoga_from_handle(self.handle())
    }

    fn set_needs_layout(&mut self) {
        unsafe {
            // NOTE: Currently there is no way to re-layout just 1 area in a window.

            let window: id = msg_send![self.handle(), window];
            let window_content_view: id = msg_send![window, contentView];

            msg_send![window_content_view, setNeedsLayout: YES];
        }
    }
}
