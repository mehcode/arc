use super::Node;
use cocoa::{
    appkit::NSEvent,
    base::{class, id, YES},
    foundation::{NSPoint, NSRect, NSSize, NSUInteger},
};
use objc::{
    declare::ClassDecl,
    runtime::{Class, Object, Sel, BOOL},
};
use super::view::CLASS as VIEW_CLASS;

pub(crate) struct Text(pub(crate) id);

// NOTE: In order to send references of this packed in Context to different threads.
//       It's very unsafe to touch these unless on the "main" thread but Context ensures
//       public access is only allowed on main thread.
unsafe impl Send for Text {}

impl Text {
    pub(crate) fn new() -> Self {
        Text(unsafe { msg_send![*CLASS, new] })
    }
}

impl Node for Text {
    fn handle(&self) -> id {
        self.0
    }
}

lazy_static! {
    pub(crate) static ref CLASS: &'static Class = declare();
}

fn declare() -> &'static Class {
    let mut cls_decl = ClassDecl::new("SQText", *VIEW_CLASS).unwrap();

    unsafe {
        cls_decl.add_method(
            sel!(drawRect:),
            draw_rect as extern "C" fn(_: &Object, _: Sel, _: NSRect),
        );
    }

    cls_decl.register()
}

extern "C" fn draw_rect(_: &Object, _: Sel, _: NSRect) {
    println!("DRAW IT BRUH");
}
