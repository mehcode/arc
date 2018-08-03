#![allow(non_upper_case_globals)]

use cocoa::{base::id, foundation::NSRect};
use core_foundation::{
    attributed_string::CFMutableAttributedString,
    base::{CFRange, FromMutVoid, FromVoid, TCFType, ToVoid},
    dictionary::CFDictionaryRef,
    string::CFStringRef,
};
use core_foundation_sys::attributed_string::CFAttributedStringSetAttribute;
use core_graphics::{
    base::CGFloat,
    geometry::{CGRect, CGSize, CG_AFFINE_TRANSFORM_IDENTITY},
    path::CGPath,
};
use core_text::{
    font::{new_from_descriptor, new_from_name},
    font_descriptor::CTFontDescriptor,
    framesetter::{CTFramesetter, CTFramesetterRef},
};
use crate::{
    os::{
        macos::{
            sys::{current_context, view},
            yoga_from_handle,
        },
        Font,
    },
    Color, Gravity,
};
use lazy_static::*;
use objc::{
    declare::ClassDecl,
    msg_send,
    runtime::{Class, Object, Sel},
    sel, sel_impl,
};
use std::{
    mem,
    os::raw::c_void,
    ptr::{self, null_mut},
};
use yoga::types::Direction;

lazy_static! {
    pub(crate) static ref CLASS: &'static Class = {
        let mut decl = ClassDecl::new("SQText", *view::CLASS).unwrap();

        unsafe {
            decl.add_ivar::<*const c_void>("sqText");
            decl.add_ivar::<*const c_void>("sqFontDescriptor");
            decl.add_ivar::<CGFloat>("sqFontSize");
            decl.add_ivar::<u16>("sqFontWeight");
            decl.add_ivar::<u16>("sqGravity");
            // decl.add_ivar::<*const c_void>("sqFont");
            // decl.add_ivar::<*const c_void>("sqFramesetter");

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

        // Memory to be released in dealloc
        mem::forget(text);
    }

    set_text_size(this, 14.);

    this
}

macro_rules! cf_release {
    ($this:expr, $name:expr) => {
        #[cfg_attr(feature = "cargo-clippy", allow(unnecessary_mut_passed))]
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
    cf_release!(this, "sqFontDescriptor");
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
    let range = CFRange::init(0, text.char_len());
    let gravity = Gravity::from_bits_truncate(unsafe { *(*this).get_ivar("sqGravity") });
    let yg_node = yoga_from_handle(this);
    let dir = yg_node.get_layout_direction();

    // TODO: Only create (and set) CTParagraphStyle if needed
    // Apply horizontal gravity

    let align: u8 = if gravity.contains(Gravity::START) {
        if dir == Direction::LTR {
            kCTTextAlignmentLeft
        } else {
            kCTTextAlignmentRight
        }
    } else if gravity.contains(Gravity::END) {
        if dir == Direction::RTL {
            kCTTextAlignmentLeft
        } else {
            kCTTextAlignmentRight
        }
    } else if gravity.contains(Gravity::LEFT) {
        kCTTextAlignmentLeft
    } else if gravity.contains(Gravity::RIGHT) {
        kCTTextAlignmentRight
    } else if gravity.contains(Gravity::CENTER) {
        kCTTextAlignmentCenter
    } else {
        // Default is left align
        kCTTextAlignmentLeft
    };

    let settings = &[CTParagraphStyleSetting {
        spec: kCTParagraphStyleSpecifierAlignment,
        value: &align as *const u8 as *const _,
        value_size: mem::size_of::<u8>(),
    }];

    let paragraph_style = unsafe { CTParagraphStyleCreate(settings.as_ptr(), settings.len()) };

    unsafe {
        CFAttributedStringSetAttribute(
            text.as_concrete_TypeRef(),
            range,
            kCTParagraphStyleAttributeName,
            paragraph_style as *const _,
        );
    }

    // TODO: Only create (and set) CTFont if needed

    let font_size: CGFloat = unsafe { *this.get_ivar("sqFontSize") };
    let font_descriptor_ptr = unsafe { this.get_ivar::<*const c_void>("sqFontDescriptor") };
    let font = if font_descriptor_ptr.is_null() {
        // No set font descriptor; use default font
        // TODO: Cache a default font descriptor somewhere
        new_from_name(".SF NS Text", font_size).unwrap()
    } else {
        let font_descriptor = unsafe { CTFontDescriptor::from_void(*font_descriptor_ptr) };
        new_from_descriptor(&font_descriptor, font_size)
    };

    unsafe { text.set_attribute(range, kCTFontAttributeName, font) };

    // TODO: Only create CTFramesetter if needed

    let framesetter = CTFramesetter::new_with_attributed_string(text.as_concrete_TypeRef());

    // Apply padding (from Yoga) to path bounds

    let mut bounds: CGRect = unsafe { msg_send![this, bounds] };

    let padding_left = yg_node.get_layout_padding_left();
    let padding_right = yg_node.get_layout_padding_right();
    let padding_top = yg_node.get_layout_padding_top();
    let padding_bottom = yg_node.get_layout_padding_bottom();

    bounds.origin.x += f64::from(padding_left);
    bounds.origin.y -= f64::from(padding_top);
    bounds.size.width -= f64::from(padding_left + padding_right);
    bounds.size.height -= f64::from(padding_top + padding_bottom);

    // TODO: This can be cached

    let size = unsafe {
        CTFramesetterSuggestFrameSizeWithConstraints(
            framesetter.as_concrete_TypeRef(),
            CFRange::init(0, 0),
            null_mut(),
            CGSize::new(bounds.size.width, std::f64::MAX),
            null_mut(),
        )
    };

    // Apply vertical gravity
    if gravity.contains(Gravity::BOTTOM) {
        bounds.origin.y += -bounds.size.height + size.height;
    } else if gravity.contains(Gravity::TOP) {
        // Do nothing (is default)
    } else if gravity.contains(Gravity::CENTER_VERTICAL) {
        bounds.origin.y += -(bounds.size.height / 2.) + (size.height / 2.);
    }

    // Ensure that the text _overflows_ below the view
    if (bounds.size.height) < size.height {
        bounds.size.height = size.height;
    }

    let path = CGPath::new_with_rect(bounds);

    let frame = framesetter.create_frame(CFRange::init(0, 0), &path);

    // Draw the text to the view

    let context = current_context();

    context.save();

    context.set_text_matrix(&CG_AFFINE_TRANSFORM_IDENTITY);
    context.translate(0., bounds.size.height);
    context.scale(1.0, -1.0);

    frame.draw(context);

    context.restore();

    // Release unmanaged resources
    // TODO: Push safe wrappers to `core-foundation-rs`

    unsafe { CFRelease(paragraph_style as *const _) }
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

    let color = color.into_cgcolor();
    let range = CFRange::init(0, text.char_len());

    unsafe {
        text.set_attribute(range, kCTForegroundColorAttributeName, color);
    }

    // TODO: set_needs_display()
}

pub(crate) fn set_gravity(this: id, gravity: Gravity) {
    unsafe {
        (*this).set_ivar::<u16>("sqGravity", gravity.bits());
    }

    // TODO: set_needs_display()
}

pub(crate) fn set_font(this: id, font: &Font) {
    // Release existing font descriptor
    cf_release!(&mut *this, "sqFontDescriptor");

    // Clone the font object (which makes a new reference of the same descriptor in ObjC)
    let font = font.clone();

    unsafe {
        (*this).set_ivar(
            "sqFontDescriptor",
            font.0.as_concrete_TypeRef() as *const c_void,
        );
    }

    // Memory to be released in dealloc
    mem::forget(font);

    // TODO: set_needs_display()
}

pub(crate) fn set_text_size(this: id, size: f32) {
    unsafe {
        (*this).set_ivar("sqFontSize", CGFloat::from(size));
    }

    // TODO: set_needs_display()
}

const kCTTextAlignmentLeft: u8 = 0;
const kCTTextAlignmentRight: u8 = 1;
const kCTTextAlignmentCenter: u8 = 2;

const kCTParagraphStyleSpecifierAlignment: u32 = 0;

/*
    kCTParagraphStyleSpecifierFirstLineHeadIndent = 1,
    kCTParagraphStyleSpecifierHeadIndent = 2,
    kCTParagraphStyleSpecifierTailIndent = 3,
    kCTParagraphStyleSpecifierTabStops = 4,
    kCTParagraphStyleSpecifierDefaultTabInterval = 5,
    kCTParagraphStyleSpecifierLineBreakMode = 6,
    kCTParagraphStyleSpecifierLineHeightMultiple = 7,
    kCTParagraphStyleSpecifierMaximumLineHeight = 8,
    kCTParagraphStyleSpecifierMinimumLineHeight = 9,
    kCTParagraphStyleSpecifierLineSpacing = 10,			/* deprecated */
    kCTParagraphStyleSpecifierParagraphSpacing = 11,
    kCTParagraphStyleSpecifierParagraphSpacingBefore = 12,
    kCTParagraphStyleSpecifierBaseWritingDirection = 13,
    kCTParagraphStyleSpecifierMaximumLineSpacing = 14,
    kCTParagraphStyleSpecifierMinimumLineSpacing = 15,
    kCTParagraphStyleSpecifierLineSpacingAdjustment = 16,
    kCTParagraphStyleSpecifierLineBoundsOptions = 17,
*/

#[repr(C)]
struct CTParagraphStyleSetting {
    spec: u32,
    value_size: usize,
    value: *const c_void,
}

extern "C" {
    static kCTFontAttributeName: CFStringRef;
    static kCTForegroundColorAttributeName: CFStringRef;
    static kCTParagraphStyleAttributeName: CFStringRef;

    fn CTFramesetterSuggestFrameSizeWithConstraints(
        framesetter: CTFramesetterRef,
        string_range: CFRange,
        frame_attributes: CFDictionaryRef,
        constraints: CGSize,
        fit_range: *mut CFRange,
    ) -> CGSize;

    fn CTParagraphStyleCreate(
        settings: *const CTParagraphStyleSetting,
        count: usize,
    ) -> *mut c_void;

    fn CFRelease(ptr: *const c_void);
}
