use cocoa::base::{id, class};
use objc::declare::ClassDecl;
use objc::runtime::Class;
use cocoa::foundation::{NSString};
use cocoa::base::{nil, NO};
use super::{ObjCObject, View};
use std::os::raw::c_void;
use objc::runtime::Object;
use objc::runtime::Sel;
use super::view::ViewParams;
use super::super::color::Color;

lazy_static! {
    pub(crate) static ref CLS: &'static Class = declare();
}

pub struct Text(id);

fn declare() -> &'static Class {
    let super_cls = Class::get("NSTextView").unwrap();
    let mut decl = ClassDecl::new("SquareText", super_cls).unwrap();

    unsafe {
        // FIXME: This is duplicated from `View::declare`
        decl.add_ivar::<*mut c_void>("__square_view_params");
        decl.add_method(sel!(init), init as extern "C" fn(&Object, Sel) -> id);
    }

    decl.register()
}

// FIXME: This is duplicated from `View::declare`
extern "C" fn init(ptr: &Object, _: Sel) -> id {
    let super_cls = Class::get("NSView").unwrap();
    let ptr: id = unsafe { msg_send![super(ptr, super_cls), init] };

    unsafe {
        let params = Box::<ViewParams>::default();

        (*ptr).set_ivar::<*mut c_void>("__square_view_params", Box::into_raw(params) as *mut c_void);
    }

    unsafe {
        msg_send![ptr, setEditable: NO];
        msg_send![ptr, setSelectable: NO];
        msg_send![ptr, setDrawsBackground: NO];

        // FIXME: Allow customization here
        let font: id = msg_send![class("NSFont"), userFontOfSize: 18.0];
        msg_send![ptr, setFont: font];
    }

    ptr
}

impl ObjCObject for Text {
    fn handle(&self) -> id {
        self.0
    }
}

impl View for Text { }

impl Text {
    pub fn new() -> Self {
        let ptr: id = unsafe { msg_send![*CLS, new] };

        Text(ptr)
    }

    pub fn set_text(&mut self, text: &str) {
        unsafe {
            let text = NSString::alloc(nil).init_str(text);

            msg_send![self.0, setString: text];
        }
    }

    pub fn set_text_color<T: Into<Color>>(&mut self, color: T) {
        let Color { r, g, b, a } = color.into();

        unsafe {
            let color: id = msg_send![class("NSColor"), colorWithSRGBRed:r green:g blue:b alpha:a];
            msg_send![self.0, setTextColor: color];
        }
    }
}
