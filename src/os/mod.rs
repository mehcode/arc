#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
pub(crate) use self::macos::*;

use objc::{class, msg_send, sel, sel_impl};

#[inline]
pub(crate) fn is_main_thread() -> bool {
    unsafe { msg_send![class!(NSThread), isMainThread] }
}

#[inline]
pub(crate) fn execute_on_main_thread<R: Send>(callback: impl FnOnce() -> R + Send) -> R {
    if is_main_thread() {
        callback()
    } else {
        dispatch::Queue::main().sync(callback)
    }
}
