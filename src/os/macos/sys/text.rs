use super::view;
use cocoa::{
    base::{class, id},
    foundation::NSRect,
};
use color::Color;
use core_foundation::{
    attributed_string::CFMutableAttributedString,
    base::{CFRange, FromMutVoid, FromVoid, TCFType, ToVoid},
    string::{CFString, CFStringRef},
};
use core_graphics::{
    base::CGFloat,
    color::CGColor,
    context::CGContext,
    geometry::{CGRect, CG_AFFINE_TRANSFORM_IDENTITY},
    path::CGPath,
};
use core_text::{font::CTFont, framesetter::CTFramesetter};
use foreign_types_shared::ForeignType;
use objc::{
    declare::ClassDecl,
    runtime::{Class, Object, Sel},
};
use std::{mem, os::raw::c_void, ptr};

lazy_static! {
    pub(crate) static ref CLASS: &'static Class = {
        let mut decl = ClassDecl::new("SQText", *view::CLASS).unwrap();

        unsafe {
            decl.add_ivar::<*const c_void>("sqText");
            decl.add_ivar::<*const c_void>("sqFontFamily");
            decl.add_ivar::<CGFloat>("sqFontSize");
            decl.add_ivar::<u16>("sqFontWeight");
            // decl.add_ivar::<*const c_void>("sqFont");
            decl.add_ivar::<*const c_void>("sqFramesetter");

            decl.add_method(sel!(init), init as extern "C" fn(_: &Object, _: Sel) -> id);
            decl.add_method(
                sel!(dealloc),
                dealloc as extern "C" fn(_: &mut Object, _: Sel),
            );

            decl.add_method(
                sel!(drawRect:),
                draw_rect as extern "C" fn(_: &mut Object, _: Sel, _: NSRect),
            );
        }

        decl.register()
    };
}

extern "C" fn init(this: &Object, _: Sel) -> id {
    let this: id = unsafe { msg_send![super(this, &*view::CLASS), init] };

    unsafe {
        let text = CFMutableAttributedString::new();

        (*this).set_ivar("sqText", text.to_void());

        mem::forget(text);
    }

    set_text(this, "");
    set_font_size(this, 14.);
    set_font_family(this, ".SF NS Text");

    this
}

macro_rules! cf_release {
    ($this:expr, $name:expr) => {
        let is_null = {
            let value: &*const c_void = unsafe { $this.get_ivar($name) };
            let is_null = value.is_null();

            if !is_null {
                unsafe {
                    CFRelease(*value);
                }
            }

            is_null
        };

        if !is_null {
            unsafe {
                $this.set_ivar::<*const c_void>($name, ptr::null());
            }
        }
    };
}

extern "C" fn dealloc(this: &mut Object, _: Sel) {
    cf_release!(this, "sqText");
    cf_release!(this, "sqFontFamily");
    // cf_release!(this, "sqFont");
    // cf_release!(this, "sqFramesetter");

    unsafe { msg_send![super(this, &*view::CLASS), dealloc] }
}

extern "C" fn draw_rect(this: &mut Object, _: Sel, dirty_rect: NSRect) {
    unsafe {
        // Draw background (before text)
        msg_send![super(this, &*view::CLASS), drawRect: dirty_rect];
    }

    let mut text =
        unsafe { CFMutableAttributedString::from_mut_void(*(*this).get_mut_ivar("sqText")) };

    // TODO: Only create (and set) CTFont if needed
    let font_family = unsafe { CFString::from_void(*this.get_ivar("sqFontFamily")) };
    let font_size: CGFloat = unsafe { *this.get_ivar("sqFontSize") };
    let font = CTFont::new_from_name(&font_family, font_size).unwrap();

    let range = CFRange::init(0, text.char_len());
    unsafe { text.set_attribute(range, kCTFontAttributeName, font) };

    // TODO: Only create CTFramesetter if needed
    let framesetter = CTFramesetter::new_with_attributed_string(text.as_concrete_TypeRef());

    let bounds: CGRect = unsafe { msg_send![this, bounds] };
    let path = CGPath::new_with_rect(bounds);

    let frame = framesetter.create_frame(CFRange::init(0, 0), &path);

    let context = unsafe {
        let context: id = msg_send![class("NSGraphicsContext"), currentContext];
        let context: id = msg_send![context, CGContext];

        CGContext::from_ptr(context as *mut _)
    };

    context.save();

    context.set_text_matrix(&CG_AFFINE_TRANSFORM_IDENTITY);
    context.translate(0., bounds.size.height);
    context.scale(1.0, -1.0);

    frame.draw(&context);

    context.restore();

    mem::forget(context);
}

pub(crate) fn set_text(this: id, string: &str) {
    let mut text =
        unsafe { CFMutableAttributedString::from_mut_void(*(*this).get_mut_ivar("sqText")) };

    let text_len = text.char_len();
    text.replace_str(&string.into(), CFRange::init(0, text_len));

    // TODO: set_needs_display()
}

pub(crate) fn set_text_color(this: id, color: Color) {
    let mut text =
        unsafe { CFMutableAttributedString::from_mut_void(*(*this).get_mut_ivar("sqText")) };

    let color = CGColor::rgb(
        CGFloat::from(color.red),
        CGFloat::from(color.green),
        CGFloat::from(color.blue),
        CGFloat::from(color.alpha),
    );

    let range = CFRange::init(0, text.char_len());

    unsafe {
        text.set_attribute(range, kCTForegroundColorAttributeName, color);
    }

    // TODO: set_needs_display()
}

pub(crate) fn set_font_family(this: id, family: &str) {
    // cf_release!(&mut *this, "sqFont");

    let family: CFString = family.into();

    unsafe {
        (*this).set_ivar(
            "sqFontFamily",
            family.as_concrete_TypeRef() as *const c_void,
        );
    }

    // TODO: set_needs_display()

    mem::forget(family);
}

pub(crate) fn set_font_size(this: id, size: f32) {
    // cf_release!(&mut *this, "sqFont");

    unsafe {
        (*this).set_ivar("sqFontSize", CGFloat::from(size));
    }

    // TODO: set_needs_display()
}

extern "C" {
    static kCTFontAttributeName: CFStringRef;
    static kCTForegroundColorAttributeName: CFStringRef;

    fn CFRelease(ptr: *const c_void);
}
