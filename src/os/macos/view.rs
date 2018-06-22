use super::super::super::{Align, Color, Edge, FlexDirection, Justify};
use cocoa::base::{class, id, YES};
use cocoa::foundation::{NSPoint, NSRect, NSSize, NSUInteger};
use objc::{declare::ClassDecl,
           runtime::{Class, Object, Sel, BOOL}};
use std::os::raw::c_void;
use std::ptr::null_mut;
use yoga_sys::{YGDirection, YGNodeCalculateLayout, YGNodeFreeRecursive, YGNodeGetChildCount,
               YGNodeGetParent, YGNodeInsertChild, YGNodeLayoutGetHeight, YGNodeLayoutGetLeft,
               YGNodeLayoutGetTop, YGNodeLayoutGetWidth, YGNodeNew, YGNodeRef,
               YGNodeStyleSetHeightPercent, YGNodeStyleSetWidthPercent,
               YGNodeStyleSetAlignItems, YGNodeStyleSetFlexDirection, YGNodeStyleSetFlexGrow,
               YGNodeStyleSetHeight, YGNodeStyleSetJustifyContent, YGNodeStyleSetMargin,
               YGNodeStyleSetPadding, YGNodeStyleSetWidth};

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

    pub(crate) fn add_child(&mut self, view: View) {
        unsafe {
            // Add the node as a subnode (in yoga)
            let parent_node = *(*self.0).get_ivar::<*mut c_void>("sqYGNode") as YGNodeRef;
            let parent_children_count = YGNodeGetChildCount(parent_node);
            let child_node = *(*view.0).get_ivar::<*mut c_void>("sqYGNode") as YGNodeRef;

            YGNodeInsertChild(parent_node, child_node, parent_children_count);

            // Add the view as a subview (in ui)
            msg_send![self.0, addSubview:view.0];
        }
    }

    pub(crate) fn set_width(&mut self, width: f32) {
        unsafe {
            let node = *(*self.0).get_ivar::<*mut c_void>("sqYGNode") as YGNodeRef;

            YGNodeStyleSetWidth(node, width);
        }
    }

    pub(crate) fn set_width_percent(&mut self, width: f32) {
        unsafe {
            let node = *(*self.0).get_ivar::<*mut c_void>("sqYGNode") as YGNodeRef;

            YGNodeStyleSetWidthPercent(node, width);
        }
    }

    pub(crate) fn set_height(&mut self, height: f32) {
        unsafe {
            let node = *(*self.0).get_ivar::<*mut c_void>("sqYGNode") as YGNodeRef;

            YGNodeStyleSetHeight(node, height);
        }
    }

    pub(crate) fn set_height_percent(&mut self, height: f32) {
        unsafe {
            let node = *(*self.0).get_ivar::<*mut c_void>("sqYGNode") as YGNodeRef;

            YGNodeStyleSetHeightPercent(node, height);
        }
    }

    pub(crate) fn set_align_items(&mut self, align: Align) {
        unsafe {
            let node = *(*self.0).get_ivar::<*mut c_void>("sqYGNode") as YGNodeRef;

            YGNodeStyleSetAlignItems(node, align.into_yg());
        }
    }

    pub(crate) fn set_justify_content(&mut self, justify: Justify) {
        unsafe {
            let node = *(*self.0).get_ivar::<*mut c_void>("sqYGNode") as YGNodeRef;

            YGNodeStyleSetJustifyContent(node, justify.into_yg());
        }
    }

    pub(crate) fn set_padding(&mut self, edge: Edge, padding: f32) {
        unsafe {
            let node = *(*self.0).get_ivar::<*mut c_void>("sqYGNode") as YGNodeRef;

            YGNodeStyleSetPadding(node, edge.into_yg(), padding);
        }
    }

    pub(crate) fn set_margin(&mut self, edge: Edge, margin: f32) {
        unsafe {
            let node = *(*self.0).get_ivar::<*mut c_void>("sqYGNode") as YGNodeRef;

            YGNodeStyleSetMargin(node, edge.into_yg(), margin);
        }
    }

    pub(crate) fn set_flex_grow(&mut self, flex_grow: f32) {
        unsafe {
            let node = *(*self.0).get_ivar::<*mut c_void>("sqYGNode") as YGNodeRef;

            YGNodeStyleSetFlexGrow(node, flex_grow);
        }
    }

    pub(crate) fn set_flex_direction(&mut self, flex_direction: FlexDirection) {
        unsafe {
            let node = *(*self.0).get_ivar::<*mut c_void>("sqYGNode") as YGNodeRef;

            YGNodeStyleSetFlexDirection(node, flex_direction.into_yg());
        }
    }

    pub(crate) fn set_background_color(&mut self, color: Color) {
        unsafe {
            let color: id = msg_send![class("NSColor"), 
                colorWithRed: color.inner.red
                       green: color.inner.green
                        blue: color.inner.blue
                       alpha: 1.0_f64];

            msg_send![color, retain];

            (*self.0).set_ivar::<id>("sqBackgroundColor", color);

            // NOTE: This is only needed at the _root_ of the hierarchy
            msg_send![self.0, setWantsLayer: YES];
            msg_send![self.0, setNeedsDisplay: YES];
        }
    }
}

impl Clone for View {
    fn clone(&self) -> Self {
        unsafe {
            msg_send![self.0, retain];
        }

        View(self.0)
    }
}

impl Drop for View {
    fn drop(&mut self) {
        unsafe {
            msg_send![self.0, release];
        }
    }
}

lazy_static! {
    pub(crate) static ref CLASS: &'static Class = declare();
}

fn declare() -> &'static Class {
    let super_cls = Class::get("NSView").unwrap();
    let mut cls_decl = ClassDecl::new("SQView", super_cls).unwrap();

    unsafe {
        cls_decl.add_ivar::<*mut c_void>("sqYGNode");
        cls_decl.add_ivar::<id>("sqBackgroundColor");

        cls_decl.add_method(sel!(init), init as extern "C" fn(_: &Object, _: Sel) -> id);
        cls_decl.add_method(sel!(dealloc), dealloc as extern "C" fn(_: &Object, _: Sel));

        cls_decl.add_method(
            sel!(isFlipped),
            is_flipped as extern "C" fn(_: &Object, _: Sel) -> BOOL,
        );

        cls_decl.add_method(sel!(layout), layout as extern "C" fn(_: &Object, _: Sel));

        cls_decl.add_method(
            sel!(resizeSubviewsWithOldSize:),
            resize_subviews_with_old_size as extern "C" fn(_: &Object, _: Sel, _: NSSize),
        );

        cls_decl.add_method(
            sel!(wantsUpdateLayer),
            wants_update_layer as extern "C" fn(_: &Object, _: Sel) -> BOOL,
        );

        cls_decl.add_method(
            sel!(updateLayer),
            update_layer as extern "C" fn(_: &Object, _: Sel),
        );
    }

    cls_decl.register()
}

extern "C" fn init(this: &Object, _: Sel) -> id {
    unsafe {
        // Run our superclass initializer
        let super_cls = Class::get("NSView").unwrap();
        let this: id = msg_send![super(this, super_cls), init];

        // Initialize a new yoga node (for layout)
        (*this).set_ivar("sqYGNode", YGNodeNew() as *mut c_void);

        this
    }
}

extern "C" fn dealloc(this: &Object, _: Sel) {
    unsafe {
        let subviews: id = msg_send![this, subviews];
        let subviews_count: NSUInteger = msg_send![subviews, count];
        for _ in 0..subviews_count {
            let subview: id = msg_send![subviews, objectAtIndex: 0];

            // Unset yoga node (we release all yoga nodes at once)
            (*subview).set_ivar::<*mut c_void>("sqYGNode", null_mut());

            // Remove and release subview
            msg_send![subview, removeFromSuperview];
            msg_send![subview, release];
        }

        // Release the stored NSColor for background color
        let background_color: &id = (*this).get_ivar::<id>("sqBackgroundColor");
        if !background_color.is_null() {
            msg_send![*background_color, release];
        }

        // Free the yoga tree IF we are the root node
        let node = *(*this).get_ivar::<*mut c_void>("sqYGNode") as YGNodeRef;
        if !node.is_null() {
            let parent_node = YGNodeGetParent(node);
            if parent_node.is_null() {
                YGNodeFreeRecursive(node);
            }
        }
    }
}

extern "C" fn layout(this: &Object, _: Sel) {
    unsafe {
        // Run our superclass layout
        let super_cls = Class::get("NSView").unwrap();
        msg_send![super(this, super_cls), layout];

        // Calculate layout for yoga tree if at root
        let node = *(*this).get_ivar::<*mut c_void>("sqYGNode") as YGNodeRef;
        let parent_node = YGNodeGetParent(node);

        if parent_node.is_null() {
            let frame: NSRect = msg_send![this, frame];
            let frame_width = frame.size.width as f32;
            let frame_height = frame.size.height as f32;

            YGNodeCalculateLayout(node, frame_width, frame_height, YGDirection::YGDirectionLTR);
        } else {
            // Apply layout
            let left = YGNodeLayoutGetLeft(node) as f64;
            let top = YGNodeLayoutGetTop(node) as f64;
            // let right = YGNodeLayoutGetRight(node);
            // let bottom = YGNodeLayoutGetBottom(node);
            let width = YGNodeLayoutGetWidth(node) as f64;
            let height = YGNodeLayoutGetHeight(node) as f64;

            let frame = NSRect::new(NSPoint::new(left, top), NSSize::new(width, height));

            msg_send![this, setFrame: frame];
        }
    }
}

extern "C" fn is_flipped(_: &Object, _: Sel) -> BOOL {
    YES
}

extern "C" fn wants_update_layer(_: &Object, _: Sel) -> BOOL {
    YES
}

extern "C" fn resize_subviews_with_old_size(this: &Object, _: Sel, _: NSSize) {
    unsafe {
        let subviews: id = msg_send![this, subviews];
        let subviews_count: NSUInteger = msg_send![subviews, count];
        for i in 0..subviews_count {
            let subview: id = msg_send![subviews, objectAtIndex: i];

            msg_send![subview, setNeedsLayout: YES];
        }
    }
}

extern "C" fn update_layer(this: &Object, _: Sel) {
    unsafe {
        let background_color: &id = (*this).get_ivar::<id>("sqBackgroundColor");
        if !background_color.is_null() {
            let background_color: id = msg_send![*background_color, CGColor];

            let layer: id = msg_send![this, layer];
            msg_send![layer, setBackgroundColor: background_color];
        }
    }
}
