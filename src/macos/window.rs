use super::View;
use cocoa::appkit::{NSBackingStoreBuffered, NSWindow, NSWindowStyleMask};
use cocoa::base::{id, nil, NO};
use cocoa::foundation::{NSAutoreleasePool, NSString};
use cocoa::foundation::{NSPoint, NSRect, NSSize};

pub struct Window {
    instance: id,
}

impl Window {
    pub fn new(w: f64, h: f64) -> Self {
        let instance = unsafe {
            NSWindow::alloc(nil)
                .initWithContentRect_styleMask_backing_defer_(
                    NSRect::new(NSPoint::new(0., 0.), NSSize::new(w, h)),
                    NSWindowStyleMask::NSTitledWindowMask | NSWindowStyleMask::NSResizableWindowMask
                        | NSWindowStyleMask::NSClosableWindowMask
                        | NSWindowStyleMask::NSMiniaturizableWindowMask,
                    NSBackingStoreBuffered,
                    NO,
                )
                .autorelease()
        };

        // TODO: Window positioning should be configurable
        unsafe {
            instance.center();
        };

        Window { instance }
    }

    pub fn set_title(&self, title: &str) {
        unsafe {
            self.instance
                .setTitle_(NSString::alloc(nil).init_str(title));
        }
    }

    pub fn set_content_view<V: View>(&self, view: V) {
        unsafe {
            self.instance.setContentView_(view.handle());
        }
    }

    pub fn activate(&self) {
        unsafe {
            self.instance.makeKeyAndOrderFront_(nil);
        }
    }
}
