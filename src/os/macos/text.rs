use super::{super::super::Color, view::CLASS as VIEW_CLASS, Node};
use cocoa::{
    base::{class, id, nil, YES},
    foundation::{NSAutoreleasePool, NSRect, NSString},
};
use core_foundation::{
    array::CFIndex,
    base::TCFType,
    string::{CFString, CFStringRef},
};
use core_foundation_sys::base::CFRange;
use core_graphics::{
    base::CGFloat,
    display::CFRelease,
    geometry::{CGAffineTransform, CG_AFFINE_TRANSFORM_IDENTITY},
};
use objc::{
    declare::ClassDecl,
    runtime::{Class, Object, Sel},
};
use std::{os::raw::c_void, ptr};

pub(crate) struct Text(pub(crate) id);

// NOTE: In order to send references of this packed in Context to different threads.
//       It's very unsafe to touch these unless on the "main" thread but Context ensures
//       public access is only allowed on main thread.
unsafe impl Send for Text {}

impl Text {
    pub(crate) fn new() -> Self {
        Text(unsafe { msg_send![*CLASS, new] })
    }

    pub(crate) fn set_text(&mut self, text: &str) {
        unsafe {
            let text = NSString::alloc(nil).init_str(text).autorelease();
            let string: &id = (*self.0).get_ivar("sqText");

            CFAttributedStringReplaceString(
                *string,
                CFRange::init(0, CFAttributedStringGetLength(*string)),
                text,
            );
        }
    }

    pub(crate) fn set_text_color(&mut self, color: Color) {
        unsafe {
            let string: &id = (*self.0).get_ivar("sqText");

            let color: id = msg_send![class("NSColor"),
                    colorWithRed: color.red as f64
                           green: color.green as f64
                            blue: color.blue as f64
                           alpha: color.alpha as f64];

            let color: id = msg_send![color, CGColor];

            CFAttributedStringSetAttribute(
                *string,
                CFRange::init(0, CFAttributedStringGetLength(*string)),
                kCTForegroundColorAttributeName,
                color,
            );
        }
    }

    //
    // Font
    //

    pub(crate) fn set_font_size(&mut self, size: f32) {
        unsafe {
            let string: &id = (*self.0).get_ivar("sqText");
            let font: &id = (*self.0).get_ivar("sqFont");

            let new_font =
                CTFontCreateCopyWithAttributes(*font, size.into(), ptr::null(), ptr::null());

            CFAttributedStringSetAttribute(
                *string,
                CFRange::init(0, CFAttributedStringGetLength(*string)),
                kCTFontAttributeName,
                new_font,
            );

            CFRelease(*font as *mut _);

            (*self.0).set_ivar("sqFont", new_font);
        }
    }

    //
    // Style
    //

    pub(crate) fn set_background_color(&mut self, color: Color) {
        unsafe {
            let color: id = msg_send![class("NSColor"),
                    colorWithRed: color.red as f64
                           green: color.green as f64
                            blue: color.blue as f64
                           alpha: color.alpha as f64];

            msg_send![color, retain];

            (*self.0).set_ivar::<id>("sqBackgroundColor", color);

            msg_send![self.0, setNeedsDisplay: YES];
        }
    }
}

//
// Node
//

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
        cls_decl.add_ivar::<id>("sqText");
        cls_decl.add_ivar::<id>("sqFont");

        cls_decl.add_method(sel!(init), init as extern "C" fn(_: &Object, _: Sel) -> id);
        cls_decl.add_method(sel!(dealloc), dealloc as extern "C" fn(_: &Object, _: Sel));

        cls_decl.add_method(
            sel!(drawRect:),
            draw_rect as extern "C" fn(_: &Object, _: Sel, _: NSRect),
        );
    }

    cls_decl.register()
}

extern "C" fn init(this: &Object, _: Sel) -> id {
    unsafe {
        let this: id = msg_send![super(this, &*VIEW_CLASS), init];

        let text = NSString::alloc(nil).init_str("").autorelease();

        let string = CFAttributedStringCreateMutable(nil, 0);
        CFAttributedStringReplaceString(string, CFRange::init(0, 0), text);

        let font_family: CFString = ".SF NS Text".parse().unwrap();
        let font = CTFontCreateWithName(font_family.as_concrete_TypeRef(), 14., ptr::null());

        let color: id = msg_send![class("NSColor"),
                colorWithRed: 0. as f64
                       green: 0. as f64
                        blue: 0. as f64
                       alpha: 1.0 as f64];

        let color = msg_send![color, CGColor];

        CFAttributedStringSetAttribute(
            string,
            CFRange::init(0, CFAttributedStringGetLength(string)),
            kCTFontAttributeName,
            font,
        );

        CFAttributedStringSetAttribute(
            string,
            CFRange::init(0, CFAttributedStringGetLength(string)),
            kCTForegroundColorAttributeName,
            color,
        );

        (*this).set_ivar("sqText", string);
        (*this).set_ivar("sqFont", font);

        this
    }
}

extern "C" fn dealloc(this: &Object, _: Sel) {
    unsafe {
        let text: &*const c_void = (*this).get_ivar::<*const c_void>("sqText");
        if !text.is_null() {
            CFRelease(*text);
        }

        let font: &*const c_void = (*this).get_ivar::<*const c_void>("sqFont");
        if !font.is_null() {
            CFRelease(*font);
        }

        let background_color: &id = (*this).get_ivar::<id>("sqBackgroundColor");
        if !background_color.is_null() {
            msg_send![*background_color, release];
        }
    }
}

extern "C" fn draw_rect(this: &Object, _: Sel, dirty_rect: NSRect) {
    unsafe {
        // Draw background
        msg_send![super(this, &*VIEW_CLASS), drawRect: dirty_rect];

        let string: id = *(*this).get_ivar("sqText");

        let framesetter = CTFramesetterCreateWithAttributedString(string);

        // TODO: Use CTFramesetterSuggestFrameSizeWithConstraints to try and
        //       make sure we don't clip out the text if our bounds are too small.

        let bounds: NSRect = msg_send![this, bounds];
        let path = CGPathCreateWithRect(bounds, nil);

        let frame = CTFramesetterCreateFrame(
            framesetter,
            CFRange::init(0, 0),
            path,
            nil,
        );

        let context: id = msg_send![class("NSGraphicsContext"), currentContext];
        let context: id = msg_send![context, CGContext];

        CGContextSetTextMatrix(context, CG_AFFINE_TRANSFORM_IDENTITY);
        CGContextTranslateCTM(context, 0., bounds.size.height);
        CGContextScaleCTM(context, 1.0, -1.0);

        CTFrameDraw(frame, context);

        CFRelease(frame as *mut _);
        CFRelease(framesetter as *mut _);
        CGPathRelease(path);
    }
}

// Foreign APIs used in this file (not exposed via Rust libs)
// TODO: Send PRs to core-foundation et all
// + Core Foundation
// + Core Graphics
// + Core Text

extern "C" {
    static kCTFontAttributeName: CFStringRef;
    static kCTForegroundColorAttributeName: CFStringRef;

    fn CTFontCreateWithName(
        name: CFStringRef,
        size: CGFloat,
        matrix: *const CGAffineTransform,
    ) -> id;

    fn CTFontCreateCopyWithAttributes(
        font: id,
        size: CGFloat,
        matrix: *const CGAffineTransform,
        attributes: *const c_void,
    ) -> id;

    fn CFAttributedStringCreateMutable(alloc: id, max_length: CFIndex) -> id;
    fn CFAttributedStringGetLength(str: id) -> CFIndex;
    fn CFAttributedStringReplaceString(str: id, range: CFRange, new: id);
    fn CFAttributedStringSetAttribute(str: id, range: CFRange, name: CFStringRef, value: id);

    fn CGPathRelease(path: id);

    fn CTFramesetterCreateWithAttributedString(string: id) -> id;
    fn CTFramesetterCreateFrame(
        framesetter: id,
        string_range: CFRange,
        path: id,
        attributes: id,
    ) -> id;

    fn CGPathCreateWithRect(rect: NSRect, transform: id) -> id;
    fn CTFrameDraw(frame: id, context: id);

    fn CGContextSetTextMatrix(context: id, transform: CGAffineTransform);
    fn CGContextTranslateCTM(context: id, tx: CGFloat, ty: CGFloat);
    fn CGContextScaleCTM(context: id, sx: CGFloat, sy: CGFloat);
}
