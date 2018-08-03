use super::super::super::super::Color;
use core_graphics::{base::CGFloat, color::CGColor};
use objc::{
    msg_send,
    runtime::Object,
    class,
    sel, sel_impl};

impl Color {
    #[inline]
    pub(crate) fn into_nscolor(self) -> *mut Object {
        unsafe {
            msg_send![class!(NSColor),
                colorWithRed: f64::from(self.red)
                       green: f64::from(self.green)
                        blue: f64::from(self.blue)
                       alpha: f64::from(self.alpha)]
        }
    }

    #[inline]
    pub(crate) fn into_cgcolor(self) -> CGColor {
        CGColor::rgb(
            CGFloat::from(self.red),
            CGFloat::from(self.green),
            CGFloat::from(self.blue),
            CGFloat::from(self.alpha),
        )
    }
}
