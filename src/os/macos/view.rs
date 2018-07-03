use super::{
    super::super::{Color, Event, MouseButton, MouseDown, MouseEnter, MouseLeave, MouseUp, Point},
    node::yoga_from_handle,
    Node,
};
use cocoa::{
    appkit::NSEvent,
    base::{class, id, YES},
    foundation::{NSPoint, NSRect, NSSize, NSUInteger},
};
use core_graphics::geometry::CGRect;
use objc::{
    declare::ClassDecl,
    runtime::{Class, Object, Sel, BOOL},
};
use std::{os::raw::c_void, ptr};
use yoga;

// TODO: Investigate ways to make this file more "safe"

pub(crate) struct View(pub(crate) id);

// NOTE: In order to send references of this packed in Context to different threads.
//       It's very unsafe to touch these unless on the "main" thread but Context ensures
//       public access is only allowed on main thread.
unsafe impl Send for View {}

impl View {
    pub(crate) fn new() -> Self {
        // Allocate and initialize an empty, native view
        let view: id = unsafe { msg_send![*CLASS, new] };

        View(view)
    }

    pub(crate) fn add(&mut self, node: &impl Node) {
        let this_node = self.yoga();
        let this_node_children_count = this_node.child_count();

        let incoming_handle = node.handle();
        let incoming_node = yoga_from_handle(incoming_handle);

        // Add this node as a sub-node (in layout)
        this_node.insert_child(incoming_node, this_node_children_count);

        // Add the view as a sub-view (in ui)
        unsafe {
            msg_send![self.0, addSubview: incoming_handle];
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

    pub(crate) fn set_corner_radius(&mut self, radius: f32) {
        unsafe {
            (*self.0).set_ivar("sqCornerRadius", f64::from(radius));
        }
    }

    //
    // Events
    //

    pub(crate) fn mouse_down(&mut self) -> &mut Event<MouseDown> {
        unsafe { &mut *(*(*self.0).get_mut_ivar::<*mut c_void>("sqEVTMouseDown") as *mut _) }
    }

    pub(crate) fn mouse_up(&mut self) -> &mut Event<MouseUp> {
        unsafe { &mut *(*(*self.0).get_mut_ivar::<*mut c_void>("sqEVTMouseUp") as *mut _) }
    }

    pub(crate) fn mouse_enter(&mut self) -> &mut Event<MouseEnter> {
        unsafe { &mut *(*(*self.0).get_mut_ivar::<*mut c_void>("sqEVTMouseEnter") as *mut _) }
    }

    pub(crate) fn mouse_leave(&mut self) -> &mut Event<MouseLeave> {
        unsafe { &mut *(*(*self.0).get_mut_ivar::<*mut c_void>("sqEVTMouseLeave") as *mut _) }
    }
}

impl Drop for View {
    fn drop(&mut self) {
        unsafe {
            msg_send![self.0, release];
        }
    }
}

impl Node for View {
    fn handle(&self) -> id {
        self.0
    }
}

lazy_static! {
    pub(crate) static ref CLASS: &'static Class = declare();
}

fn declare() -> &'static Class {
    let super_cls = Class::get("NSView").unwrap();
    let mut cls_decl = ClassDecl::new("SQView", super_cls).unwrap();

    unsafe {
        // Yoga Node (ivar)
        cls_decl.add_ivar::<*mut c_void>("sqYGNode");

        // Background Color (ivar)
        cls_decl.add_ivar::<id>("sqBackgroundColor");

        // Corner Radius (ivar)
        cls_decl.add_ivar::<f64>("sqCornerRadius");

        // Events (ivar)

        cls_decl.add_ivar::<*mut c_void>("sqEVTMouseDown");
        cls_decl.add_ivar::<*mut c_void>("sqEVTMouseUp");
        cls_decl.add_ivar::<*mut c_void>("sqEVTMouseEnter");
        cls_decl.add_ivar::<*mut c_void>("sqEVTMouseLeave");

        // Methods

        cls_decl.add_method(sel!(init), init as extern "C" fn(_: &Object, _: Sel) -> id);
        cls_decl.add_method(sel!(dealloc), dealloc as extern "C" fn(_: &Object, _: Sel));

        cls_decl.add_method(
            sel!(isFlipped),
            is_flipped as extern "C" fn(_: &Object, _: Sel) -> BOOL,
        );

        cls_decl.add_method(
            sel!(layout),
            layout as extern "C" fn(_: &mut Object, _: Sel),
        );

        cls_decl.add_method(
            sel!(drawRect:),
            draw_rect as extern "C" fn(_: &Object, _: Sel, _: NSRect),
        );

        // Events (method)

        cls_decl.add_method(
            sel!(mouseDown:),
            mouse_down as extern "C" fn(_: &Object, _: Sel, _: id),
        );

        cls_decl.add_method(
            sel!(rightMouseDown:),
            mouse_down as extern "C" fn(_: &Object, _: Sel, _: id),
        );

        cls_decl.add_method(
            sel!(otherMouseDown:),
            mouse_down as extern "C" fn(_: &Object, _: Sel, _: id),
        );

        cls_decl.add_method(
            sel!(mouseUp:),
            mouse_up as extern "C" fn(_: &Object, _: Sel, _: id),
        );

        cls_decl.add_method(
            sel!(rightMouseUp:),
            mouse_up as extern "C" fn(_: &Object, _: Sel, _: id),
        );

        cls_decl.add_method(
            sel!(otherMouseUp:),
            mouse_up as extern "C" fn(_: &Object, _: Sel, _: id),
        );

        cls_decl.add_method(
            sel!(mouseEnter:),
            mouse_enter as extern "C" fn(_: &Object, _: Sel, _: id),
        );

        cls_decl.add_method(
            sel!(mouseLeave:),
            mouse_leave as extern "C" fn(_: &Object, _: Sel, _: id),
        );
    }

    cls_decl.register()
}

extern "C" fn init(this: &Object, _: Sel) -> id {
    unsafe {
        // Superclass

        let super_cls = Class::get("NSView").unwrap();
        let this: id = msg_send![super(this, super_cls), init];

        // Yoga node (layout)

        (*this).set_ivar(
            "sqYGNode",
            Box::into_raw(Box::new(yoga::Node::new())) as *mut c_void,
        );

        // Events
        // TODO: Init these on demand

        (*this).set_ivar(
            "sqEVTMouseDown",
            Box::into_raw(Box::new(Event::<MouseDown>::new())) as *mut c_void,
        );

        (*this).set_ivar(
            "sqEVTMouseUp",
            Box::into_raw(Box::new(Event::<MouseUp>::new())) as *mut c_void,
        );

        (*this).set_ivar(
            "sqEVTMouseEnter",
            Box::into_raw(Box::new(Event::<MouseEnter>::new())) as *mut c_void,
        );

        (*this).set_ivar(
            "sqEVTMouseLeave",
            Box::into_raw(Box::new(Event::<MouseLeave>::new())) as *mut c_void,
        );

        this
    }
}

extern "C" fn dealloc(this: &Object, _: Sel) {
    unsafe {
        let subviews: id = msg_send![this, subviews];
        let subviews_count: NSUInteger = msg_send![subviews, count];
        for _ in 0..subviews_count {
            let subview: id = msg_send![subviews, objectAtIndex: 0];

            // Remove and release subview
            msg_send![subview, removeFromSuperview];
            msg_send![subview, release];
        }

        //
        // Events
        //

        let _ = Box::from_raw(
            *(*this).get_ivar::<*mut c_void>("sqEVTMouseDown") as *mut Event<MouseDown>
        );

        let _ =
            Box::from_raw(*(*this).get_ivar::<*mut c_void>("sqEVTMouseUp") as *mut Event<MouseUp>);

        let _ = Box::from_raw(
            *(*this).get_ivar::<*mut c_void>("sqEVTMouseEnter") as *mut Event<MouseEnter>
        );

        let _ = Box::from_raw(
            *(*this).get_ivar::<*mut c_void>("sqEVTMouseLeave") as *mut Event<MouseLeave>
        );

        // Release the stored NSColor for background color

        let background_color: &id = (*this).get_ivar::<id>("sqBackgroundColor");
        if !background_color.is_null() {
            msg_send![*background_color, release];
        }

        // Free the yoga node (layout)

        let _ = Box::from_raw(*(*this).get_ivar::<*mut c_void>("sqYGNode") as *mut yoga::Node);
    }
}

fn yoga_apply_layout_to_view_hierarchy(view: id) {
    let node = yoga_from_handle(view);

    let x = f64::from(node.get_layout_left());
    let y = f64::from(node.get_layout_top());

    let width = f64::from(node.get_layout_width());
    let height = f64::from(node.get_layout_height());

    let frame = NSRect::new(NSPoint::new(x, y), NSSize::new(width, height));

    unsafe {
        msg_send![view, setFrame: frame];

        let subviews: id = msg_send![view, subviews];
        let subviews_count: NSUInteger = msg_send![subviews, count];

        for i in 0..subviews_count {
            let subview: id = msg_send![subviews, objectAtIndex: i];

            yoga_apply_layout_to_view_hierarchy(subview);
        }
    }
}

extern "C" fn layout(this: &mut Object, _: Sel) {
    unsafe {
        let window: id = msg_send![this, window];
        let window_content_view: id = msg_send![window, contentView];
        let is_root = ptr::eq(window_content_view, this);

        if is_root {
            let bounds: NSRect = msg_send![this, frame];
            let node = yoga_from_handle(&mut *this);

            // Calculate layout for tree (if at root)
            node.calculate_layout(
                bounds.size.width as f32,
                bounds.size.height as f32,
                yoga::Direction::LTR,
            );

            // Apply layout to view hierarchy
            yoga_apply_layout_to_view_hierarchy(this);
        } else {
            return;
        }
    }
}

extern "C" fn is_flipped(_: &Object, _: Sel) -> BOOL {
    YES
}

//extern "C" fn wants_update_layer(_: &Object, _: Sel) -> BOOL {
//    YES
//}
//
//extern "C" fn update_layer(this: &Object, _: Sel) {
//    unsafe {
//        let layer: id = msg_send![this, layer];
//
//        let background_color: &id = (*this).get_ivar::<id>("sqBackgroundColor");
//        if !background_color.is_null() {
//            let background_color: id = msg_send![*background_color, CGColor];
//
//            println!("SET THE BG COLOR");
//            msg_send![layer, setBackgroundColor: background_color];
//        }
//
//        let corner_radius: f64 = *(*this).get_ivar("sqCornerRadius");
//        msg_send![layer, setCornerRadius: corner_radius];
//        msg_send![layer, setMasksToBounds: YES];
//    }
//}

extern "C" {
    fn CGContextSetFillColorWithColor(context: id, color: id);
    fn CGContextFillRect(context: id, rect: CGRect);
}

extern "C" fn draw_rect(this: &Object, _: Sel, dirty_rect: NSRect) {
    unsafe {
        // Draw background (if present)
        let background_color: &id = (*this).get_ivar("sqBackgroundColor");
        if !background_color.is_null() {
            // TODO: Make helper function here
            let context: id = msg_send![class("NSGraphicsContext"), currentContext];
            let context: id = msg_send![context, CGContext];

            let background_color: id = msg_send![*background_color, CGColor];
            CGContextSetFillColorWithColor(context, background_color);

            CGContextFillRect(context, *dirty_rect.as_CGRect());
        }

        // Draw subviews (on top of background)
        msg_send![super(this, &*class("NSView")), drawRect: dirty_rect];
    }
}

extern "C" fn mouse_down(this: &Object, _: Sel, native_event: id) {
    unsafe {
        let event =
            &mut *(*(*this).get_ivar::<*mut c_void>("sqEVTMouseDown") as *mut Event<MouseDown>);

        event.dispatch(MouseDown {
            location: location_in_window(this, native_event),
            button: mouse_button(native_event),
        });
    }
}

extern "C" fn mouse_up(this: &Object, _: Sel, native_event: id) {
    unsafe {
        let event = &mut *(*(*this).get_ivar::<*mut c_void>("sqEVTMouseUp") as *mut Event<MouseUp>);

        event.dispatch(MouseUp {
            location: location_in_window(this, native_event),
            button: mouse_button(native_event),
        });
    }
}

extern "C" fn mouse_enter(this: &Object, _: Sel, native_event: id) {
    unsafe {
        let event =
            &mut *(*(*this).get_ivar::<*mut c_void>("sqEVTMouseEnter") as *mut Event<MouseEnter>);

        event.dispatch(MouseEnter {
            location: location_in_window(this, native_event),
        });
    }
}

extern "C" fn mouse_leave(this: &Object, _: Sel, native_event: id) {
    unsafe {
        let event =
            &mut *(*(*this).get_ivar::<*mut c_void>("sqEVTMouseLeave") as *mut Event<MouseLeave>);

        event.dispatch(MouseLeave {
            location: location_in_window(this, native_event),
        });
    }
}

fn mouse_button(event: id) -> MouseButton {
    match unsafe { event.buttonNumber() } {
        0 => MouseButton::Left,
        1 => MouseButton::Right,
        2 => MouseButton::Middle,
        other => MouseButton::Other(other as u8),
    }
}

fn location_in_window(this: &Object, event: id) -> Point {
    // Even though the NSView is declared to have flipped coordinates the NSEvent
    // reports the location-in-window as (0,0) in bottom-left (we want top-left)
    let window: id = unsafe { msg_send![this, window] };
    let window_frame: NSRect = unsafe { msg_send![window, contentLayoutRect] };
    let mut location = unsafe { event.locationInWindow() };
    location.y = window_frame.size.height - location.y;

    Point::new(location.x as f32, location.y as f32)
}
