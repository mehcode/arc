mod color;
pub(crate) mod text;
pub(crate) mod view;

use cocoa::base::{class, YES};
use core_graphics::context::CGContextRef;
use crate::Event;
use foreign_types_shared::ForeignTypeRef;
use objc::{msg_send, runtime::Object, sel, sel_impl};
use std::os::raw::c_void;

#[inline]
pub(crate) fn set_needs_display(this: *mut Object) {
    unsafe {
        msg_send![this, setNeedsDisplay: YES];
    }
}

#[inline]
pub(crate) fn event<'a, T>(this: *mut Object, name: &'static str) -> &'a mut Event<T> {
    unsafe { &mut *(*(*this).get_mut_ivar::<*mut c_void>(name) as *mut _) }
}

#[inline]
pub(crate) fn current_context() -> &'static CGContextRef {
    unsafe {
        let context: *mut Object = msg_send![class("NSGraphicsContext"), currentContext];
        let context: *mut c_void = msg_send![context, CGContext];

        CGContextRef::from_ptr(context as *mut _)
    }
}
