use cocoa::base::id;
use cocoa::foundation::{NSPoint, NSRect, NSSize};
use objc::declare::ClassDecl;
use objc::runtime::Class;
use objc::runtime::Object;
use objc::runtime::Sel;
use std::mem;
use std::os::raw::c_void;
use super::view;
use super::view::{Size, View, ViewParams};
use super::ObjCObject;

pub const ORIENTATION_HORIZONTAL: u8 = 0;
pub const ORIENTATION_VERTICAL: u8 = 1;

lazy_static! {
    pub(crate) static ref CLS: &'static Class = declare();
}

pub struct ViewBox(id);

fn declare() -> &'static Class {
    let super_cls = &*view::CLS;
    let mut decl = ClassDecl::new("ArcViewBox", super_cls).unwrap();

    unsafe {
        decl.add_ivar::<u8>("__arc_view_box_orientation");
        decl.add_method(sel!(layout), layout as extern "C" fn(&Object, Sel));
    }

    decl.register()
}

extern "C" fn layout(ptr: &Object, _: Sel) {
    unsafe {
        let subviews: id = msg_send![ptr, subviews];
        let count: usize = msg_send![subviews, count];
        let orientation: u8 = *(*ptr).get_ivar("__arc_view_box_orientation");

        let frame_size: NSSize = msg_send![ptr, frameSize];

        let mut number_of_unspec = 0.0;
        let mut total_accounted_for_size: f64 = 0.0;

        for i in 0..count {
            let subview: id = msg_send![subviews, objectAtIndex: i];

            // TODO: Getting parameters should be easier
            let params = (*subview).get_ivar::<*mut c_void>("__arc_view_params");
            let params: &Box<ViewParams> = mem::transmute(params);

            if orientation == ORIENTATION_VERTICAL {
                match params.height {
                    Size::Unspecified => {
                        number_of_unspec += 1.0;
                    }

                    Size::Absolute(v) => {
                        total_accounted_for_size += v;
                    }
                }
            } else if orientation == ORIENTATION_HORIZONTAL {
                match params.width {
                    Size::Unspecified => {
                        number_of_unspec += 1.0;
                    }

                    Size::Absolute(v) => {
                        total_accounted_for_size += v;
                    }
                }
            }
        }

        let mut y = 0.0;

        for i in 0..count {
            let subview: id = msg_send![subviews, objectAtIndex: i];

            // TODO: Getting parameters should be easier
            let params = (*subview).get_ivar::<*mut c_void>("__arc_view_params");
            let params: &Box<ViewParams> = mem::transmute(params);

            // TODO: Use a balanced formula to ensure we cover the parent view completely

            let sv_width = match params.width {
                Size::Absolute(w) => w,
                Size::Unspecified => {
                    if orientation == ORIENTATION_HORIZONTAL {
                        (frame_size.width - total_accounted_for_size) / number_of_unspec
                    } else {
                        frame_size.width
                    }
                }
            };

            let sv_height = match params.height {
                Size::Absolute(h) => h,
                Size::Unspecified => {
                    if orientation == ORIENTATION_VERTICAL {
                        (frame_size.height - total_accounted_for_size) / number_of_unspec
                    } else {
                        frame_size.height
                    }
                }
            };

            let origin = NSPoint::new(0.0, y);
            let frame = NSRect::new(origin, NSSize::new(sv_width, sv_height));

            msg_send![subview, setFrame: frame];

            y += frame.size.height;
        }
    }
}

impl ViewBox {
    pub fn new(orientation: u8) -> Self {
        let ptr: id = unsafe { msg_send![*CLS, new] };

        unsafe {
            (*ptr).set_ivar("__arc_view_box_orientation", orientation);
        }

        ViewBox(ptr)
    }
}

impl ObjCObject for ViewBox {
    fn handle(&self) -> id {
        self.0
    }
}

impl View for ViewBox {}
