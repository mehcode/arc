use super::Window;
use cocoa::{
    appkit::{
        NSApp, NSApplication, NSApplicationActivateIgnoringOtherApps,
        NSApplicationActivationPolicy, NSApplicationActivationPolicyProhibited,
        NSApplicationActivationPolicyRegular, NSMenu, NSMenuItem, NSRunningApplication,
    },
    base::{class, id, nil, YES}, foundation::{NSAutoreleasePool, NSProcessInfo, NSString},
};
use dispatch;
use objc::{
    declare::ClassDecl, runtime::{Class, Object, Sel, BOOL},
};
use std::cell::Cell;

pub(crate) struct Context {
    pool: Cell<Option<id>>,
}

impl Context {
    pub(crate) fn new() -> Self {
        let pool = unsafe { NSAutoreleasePool::new(nil) };
        unsafe {
            // Initialize the shared application instance
            let app: id = NSApp();

            // Set the application delegate
            let delegate: id = msg_send![*DELEGATE_CLASS, new];
            msg_send![app, setDelegate: delegate];

            // Declare a new menu bar for our application
            let menubar = NSMenu::new(nil).autorelease();
            app.setMainMenu_(menubar);

            // Declare the 0th menu item (the app menu) in the menu bar
            let app_menubar_item = NSMenuItem::new(nil).autorelease();
            menubar.addItem_(app_menubar_item);

            // Declare the app menu (inside the app menubar item)
            let app_menu = NSMenu::new(nil).autorelease();
            app_menubar_item.setSubmenu_(app_menu);

            // Add `Quit %s` (quit the application)
            let app_name = NSProcessInfo::processInfo(nil).processName();
            let quit_title = NSString::alloc(nil)
                .init_str("Quit ")
                .stringByAppendingString_(app_name);
            let quit_key = NSString::alloc(nil).init_str("q");
            let quit_menu_item = NSMenuItem::alloc(nil)
                .initWithTitle_action_keyEquivalent_(quit_title, sel!(terminate:), quit_key)
                .autorelease();

            app_menu.addItem_(quit_menu_item);
        }

        Self {
            pool: Cell::new(Some(pool)),
        }
    }

    pub(crate) fn add_window(&self, window: Window) {
        unsafe {
            msg_send![window.0, makeKeyAndOrderFront: nil];
        }
    }

    pub(crate) fn run(&self) {
        let app = unsafe { NSApp() };

        unsafe {
            // If we are running outside of a bundle (e.g.. `cargo run`) then
            // force activate our application
            let policy: NSApplicationActivationPolicy = msg_send![app, activationPolicy];
            if policy == NSApplicationActivationPolicyProhibited {
                app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

                NSRunningApplication::currentApplication(nil)
                    .activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
            }

            // Activate the application (unhighlights the dock icon).
            msg_send![app, finishLaunching];
        }

        if let Some(pool) = self.pool.take() {
            unsafe {
                // Release the setup NSAutoReleasePool.
                msg_send![pool, release];
            }
        }

        unsafe {
            msg_send![app, run];
        }
    }

    pub(crate) fn execute_on_main_thread(&self, callback: impl FnOnce() -> () + Send) {
        dispatch::Queue::main().sync(callback);
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        if let Some(pool) = self.pool.take() {
            unsafe {
                msg_send![pool, release];
            }
        }
    }
}

lazy_static! {
    static ref DELEGATE_CLASS: &'static Class = declare_delegate();
}

fn declare_delegate() -> &'static Class {
    let delegate_super_cls = Class::get("NSObject").unwrap();
    let mut delegate_cls_decl = ClassDecl::new("SQAppDelegate", delegate_super_cls).unwrap();

    unsafe {
        delegate_cls_decl.add_method(
            sel!(applicationShouldTerminateAfterLastWindowClosed:),
            should_terminate_after_last_window_closed
                as extern "C" fn(_: &Object, _: Sel, _: id) -> BOOL,
        );

        delegate_cls_decl.add_method(
            sel!(applicationShouldTerminate:),
            should_terminate as extern "C" fn(_: &Object, _: Sel, _: id) -> i32,
        );
    }

    delegate_cls_decl.register()
}

extern "C" fn should_terminate_after_last_window_closed(_: &Object, _: Sel, _: id) -> BOOL {
    YES
}

// Translate :terminate with :stop
// This allows us to cleanly exit the runloop on the rust side
extern "C" fn should_terminate(_: &Object, _: Sel, _: id) -> i32 {
    unsafe {
        let app: id = msg_send![class("NSApplication"), sharedApplication];
        msg_send![app, stop: nil];
    }

    // NSTerminateCancel
    0
}
