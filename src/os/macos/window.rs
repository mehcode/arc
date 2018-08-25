use cocoa::{
    appkit::{NSBackingStoreType, NSWindow, NSWindowStyleMask},
    base::{nil, NO, YES},
    foundation::{NSPoint, NSRect, NSSize, NSString},
};
use crate::{Color, NodeId};
use objc::{msg_send, runtime::Object, sel, sel_impl};

pub(crate) struct Window(pub(crate) *mut Object);

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

    pub(crate) fn set_root(&mut self, node_id: NodeId) {
        // NOTE: `Nodes::with_` is guaranteed to execute on the main thread which will
        //       ensure we don't actually switch threads.
        let this = self.0 as usize;
        node_id.with_any(move |node| unsafe {
            msg_send![this as *mut Object, setContentView: node.handle()];
        });
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            msg_send![self.0, release];
        }
    }
}
