pub(crate) mod text;
pub(crate) mod view;

use objc::runtime::Object;
use std::os::raw::c_void;
use Event;

pub(crate) fn event<'a, T>(this: *mut Object, name: &'static str) -> &'a mut Event<T> {
    unsafe { &mut *(*(*this).get_mut_ivar::<*mut c_void>(name) as *mut _) }
}
