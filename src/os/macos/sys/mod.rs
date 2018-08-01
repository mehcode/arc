mod color;
pub(crate) mod text;
pub(crate) mod view;

use cocoa::base::class;
use core_graphics::context::CGContextRef;
use foreign_types_shared::ForeignTypeRef;
use objc::runtime::Object;
use std::os::raw::c_void;
use Event;

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
