use super::{super::super::Color, View};
use cocoa::{
    appkit::{NSBackingStoreType, NSWindow, NSWindowStyleMask},
    base::{class, id, nil, NO, YES},
    foundation::{NSPoint, NSRect, NSSize, NSString},
};
use objc::runtime::Object;
use objc_id::Id;

pub(crate) struct Window(pub(crate) Id<Object>);

impl Window {
    pub(crate) fn new(width: f32, height: f32) -> Self {
        let style = NSWindowStyleMask::NSTitledWindowMask
            | NSWindowStyleMask::NSClosableWindowMask
            | NSWindowStyleMask::NSResizableWindowMask
            | NSWindowStyleMask::NSMiniaturizableWindowMask;

        let window = unsafe {
            NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(
                NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(f64::from(width), f64::from(height))),
                style,
                NSBackingStoreType::NSBackingStoreBuffered,
                YES,
            )
        };

        unsafe {
            msg_send![window, setReleasedWhenClosed: NO];
        }

        unsafe {
            window.center();
        }

        Window(unsafe { Id::from_retained_ptr(window) })
    }

    pub(crate) fn set_title(&mut self, title: &str) {
        unsafe {
            let title = NSString::alloc(nil).init_str(title);

            msg_send![self.0, setTitle: title];
        }
    }

    pub(crate) fn set_resizable(&mut self, resizable: bool) {
        unsafe {
            let mut style: NSWindowStyleMask = msg_send![self.0, styleMask];
            style.set(NSWindowStyleMask::NSResizableWindowMask, resizable);

            msg_send![self.0, setStyleMask: style];
        }
    }

    pub(crate) fn set_background_color(&mut self, color: Color) {
        unsafe {
            let color: id = msg_send![class("NSColor"), 
                colorWithRed: color.inner.red as f64
                       green: color.inner.green as f64
                        blue: color.inner.blue as f64
                       alpha: color.inner.alpha as f64];

            msg_send![self.0, setBackgroundColor: color];
        }
    }

    pub(crate) fn set_view(&mut self, view: View) {
        unsafe {
            msg_send![self.0, setContentView: &*view.0];
        }
    }
}
