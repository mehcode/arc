use cocoa::{
    appkit::{NSBackingStoreType, NSWindow, NSWindowStyleMask},
    base::{id, nil, NO, YES},
    foundation::{NSPoint, NSRect, NSSize, NSString},
};
use crate::{Color, Node};
use objc::{msg_send, sel, sel_impl};

pub(crate) struct Window(pub(crate) id);

impl Window {
    pub(crate) fn new(width: f32, height: f32) -> Self {
        let style = NSWindowStyleMask::NSTitledWindowMask
            | NSWindowStyleMask::NSClosableWindowMask
            | NSWindowStyleMask::NSResizableWindowMask
            | NSWindowStyleMask::NSMiniaturizableWindowMask;

        let window = unsafe {
            NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(
                NSRect::new(
                    NSPoint::new(0.0, 0.0),
                    NSSize::new(f64::from(width), f64::from(height)),
                ),
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

        Window(window)
    }

    crate fn show(&mut self) {
        unsafe {
            msg_send![self.0, makeKeyAndOrderFront: nil];
        }
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
            msg_send![self.0, setBackgroundColor: color.into_nscolor()];
        }
    }

    pub(crate) fn set_view(&mut self, node: &impl Node) {
        unsafe {
            msg_send![self.0, setContentView: node.handle()];
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            msg_send![self.0, release];
        }
    }
}
