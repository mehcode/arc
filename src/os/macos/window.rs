use super::super::super::Color;
use super::View;
use cocoa::{appkit::{NSBackingStoreType, NSWindow, NSWindowStyleMask},
            base::{class, id, nil, NO, YES},
            foundation::{NSPoint, NSRect, NSSize, NSString}};
use objc::runtime::Object;
use objc_id::Id;

pub(crate) struct Window(pub(crate) Id<Object>);

impl Window {
    pub(crate) fn new(width: f64, height: f64) -> Self {
        let style = NSWindowStyleMask::NSTitledWindowMask | NSWindowStyleMask::NSClosableWindowMask
            | NSWindowStyleMask::NSResizableWindowMask
            | NSWindowStyleMask::NSMiniaturizableWindowMask;

        let window = unsafe {
            NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(
                NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(width, height)),
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

    pub(crate) fn set_background_color(&mut self, color: Color) {
        unsafe {
            let color: id = msg_send![class("NSColor"), 
                colorWithRed: color.inner.red
                       green: color.inner.green
                        blue: color.inner.blue
                       alpha: 1.0_f64];

            msg_send![self.0, setBackgroundColor: color];
        }
    }

    pub(crate) fn set_view(&mut self, view: View) {
        unsafe {
            msg_send![self.0, setContentView: &*view.0];
        }
    }
}
