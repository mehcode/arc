use cocoa::appkit::NSRectFill;
use cocoa::base::{class, id};
use cocoa::foundation::NSRect;
use objc::declare::ClassDecl;
use objc::runtime::BOOL;
use objc::runtime::YES;
use objc::runtime::{Class, Object, Sel};
use std::mem;
use std::os::raw::c_void;
use super::ObjCObject;
use super::super::color::Color;

pub(crate) enum Size {
    Unspecified,
    Absolute(f64),
    // 0.0 .. 1.0 (%)
    // Relative(f64),
}

impl Default for Size {
    fn default() -> Self {
        Size::Unspecified
    }
}

lazy_static! {
    pub(crate) static ref CLS: &'static Class = declare();
}

// TODO: Store natively instead of using this struct
#[derive(Default)]
pub(crate) struct ViewParams {
    pub(crate) width: Size,
    pub(crate) height: Size,
    background_color: Color,
}

fn declare() -> &'static Class {
    let super_cls = Class::get("NSView").unwrap();
    let mut decl = ClassDecl::new("ArcView", super_cls).unwrap();

    unsafe {
        decl.add_ivar::<*mut c_void>("__arc_view_params");

        decl.add_method(sel!(init), init as extern "C" fn(&Object, Sel) -> id);

        decl.add_method(
            sel!(drawRect:),
            draw_rect as extern "C" fn(&Object, Sel, NSRect),
        );

        decl.add_method(
            sel!(isFlipped),
            is_flipped as extern "C" fn(&Object, Sel) -> BOOL,
        );
    }

    decl.register()
}

extern "C" fn init(ptr: &Object, _: Sel) -> id {
    let super_cls = Class::get("NSView").unwrap();
    let ptr: id = unsafe { msg_send![super(ptr, super_cls), init] };

    unsafe {
        let params = Box::<ViewParams>::default();

        (*ptr).set_ivar::<*mut c_void>("__arc_view_params", Box::into_raw(params) as *mut c_void);
    }

    ptr
}

extern "C" fn is_flipped(_: &Object, _: Sel) -> BOOL {
    YES
}

extern "C" fn draw_rect(ptr: &Object, _: Sel, dirty_rect: NSRect) {
    unsafe {
        // TODO: Getting parameters should be easier
        let params = (*ptr).get_ivar::<*mut c_void>("__arc_view_params");
        let params: &Box<ViewParams> = mem::transmute(params);

        let Color { r, g, b, a } = params.background_color;
        let color: id = msg_send![class("NSColor"), colorWithSRGBRed:r green:g blue:b alpha:a];
        msg_send![color, setFill];

        NSRectFill(dirty_rect);
    }
}

pub trait View: ObjCObject {
    fn set_height(&mut self, h: f64) {
        unsafe {
            // TODO: Getting parameters should be easier
            let ptr = self.handle();
            let params = (*ptr).get_mut_ivar::<*mut c_void>("__arc_view_params");
            let params: &mut Box<ViewParams> = mem::transmute(params);

            params.height = Size::Absolute(h);
        }
    }

    fn set_width(&mut self, w: f64) {
        unsafe {
            // TODO: Getting parameters should be easier
            let ptr = self.handle();
            let params = (*ptr).get_mut_ivar::<*mut c_void>("__arc_view_params");
            let params: &mut Box<ViewParams> = mem::transmute(params);

            params.width = Size::Absolute(w);
        }
    }

    fn set_background_color<C: Into<Color>>(&mut self, color: C) {
        unsafe {
            // TODO: Getting parameters should be easier
            let ptr = self.handle();
            let params = (*ptr).get_mut_ivar::<*mut c_void>("__arc_view_params");
            let params: &mut Box<ViewParams> = mem::transmute(params);

            params.background_color = color.into();
        }
    }
}
