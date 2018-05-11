use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivateIgnoringOtherApps,
                    NSApplicationActivationPolicyRegular, NSMenu, NSMenuItem, NSRunningApplication};
use cocoa::base::{id, nil, selector};
use cocoa::foundation::{NSAutoreleasePool, NSProcessInfo, NSString};
use objc::declare::ClassDecl;
use objc::runtime::Class;
use objc::runtime::Object;
use objc::runtime::Sel;
use objc::runtime::BOOL;
use objc::runtime::YES;

pub struct Application {
    #[allow(unused)]
    pool: id,

    instance: id,
}

impl Application {
    pub fn new() -> Self {
        let pool = unsafe { NSAutoreleasePool::new(nil) };
        let instance = unsafe { NSApp() };

        unsafe {
            instance.setActivationPolicy_(NSApplicationActivationPolicyRegular);

            // Create Menubar
            let menubar = NSMenu::new(nil).autorelease();
            let app_menu_item = NSMenuItem::new(nil).autorelease();
            menubar.addItem_(app_menu_item);
            instance.setMainMenu_(menubar);

            // Create Application Menu
            let app_menu = NSMenu::new(nil).autorelease();
            let quit_prefix = NSString::alloc(nil).init_str("Quit ");
            let process_name = NSProcessInfo::processInfo(nil).processName();
            let quit_title = quit_prefix.stringByAppendingString_(process_name);
            let quit_action = selector("terminate:");
            let quit_key = NSString::alloc(nil).init_str("q");
            let quit_item = NSMenuItem::alloc(nil)
                .initWithTitle_action_keyEquivalent_(quit_title, quit_action, quit_key)
                .autorelease();

            app_menu.addItem_(quit_item);
            app_menu_item.setSubmenu_(app_menu);
        }

        let delegate_super_cls = Class::get("NSObject").unwrap();
        let mut decl = ClassDecl::new("AppDelegate", delegate_super_cls).unwrap();

        unsafe {
            decl.add_method(
                sel!(applicationShouldTerminateAfterLastWindowClosed:),
                application_should_terminate_after_last_window_closed
                    as extern "C" fn(&Object, Sel, id) -> BOOL,
            );

            decl.add_method(
                sel!(applicationWillTerminate:),
                application_will_terminate as extern "C" fn(&Object, Sel, id),
            );
        }

        let delegate_cls = decl.register();

        unsafe {
            let delegate: id = msg_send![delegate_cls, new];

            msg_send![instance, setDelegate: delegate];
        }

        Application { pool, instance }
    }

    pub fn activate(&self) {
        unsafe {
            NSRunningApplication::currentApplication(nil)
                .activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
        }
    }

    pub fn run(&self) {
        unsafe {
            self.instance.run();
        }
    }
}

extern "C" fn application_should_terminate_after_last_window_closed(
    _: &Object,
    _: Sel,
    _: id,
) -> BOOL {
    YES
}

extern "C" fn application_will_terminate(_: &Object, _: Sel, _: id) {}
