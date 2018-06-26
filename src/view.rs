use super::{
    context::WeakContext, events, os, Align, Color, Context, Edge, Event, FlexDirection, Justify,
    PositionType, Wrap,
};
use yoga_sys::{
    YGNodeStyleSetAlignContent, YGNodeStyleSetAlignItems, YGNodeStyleSetAlignSelf,
    YGNodeStyleSetFlexBasis, YGNodeStyleSetFlexDirection, YGNodeStyleSetFlexGrow,
    YGNodeStyleSetFlexShrink, YGNodeStyleSetFlexWrap, YGNodeStyleSetHeight,
    YGNodeStyleSetHeightPercent, YGNodeStyleSetJustifyContent, YGNodeStyleSetMargin,
    YGNodeStyleSetMaxHeight, YGNodeStyleSetMaxWidth, YGNodeStyleSetMinHeight,
    YGNodeStyleSetMinWidth, YGNodeStyleSetPadding, YGNodeStyleSetPosition,
    YGNodeStyleSetPositionType, YGNodeStyleSetWidth, YGNodeStyleSetWidthPercent,
};

/// The fundamental component, `View` is a container that supports
/// layout with **Flexbox** powered by [Yoga](https://yogalayout.com/). View maps directly
/// to the native view equivalent of the platform (e.g.. `NSView` for macOS).
///
/// `View` is designed to be nested inside other views and can have 0 to many children of
/// any type.
pub struct View {
    pub(crate) id: usize,
    pub(crate) inner: os::View,
    pub(crate) context: WeakContext,
}

impl View {
    pub fn new(context: &Context) -> Self {
        Self {
            id: context.next_id(),
            inner: os::View::new(),
            context: context.downgrade(),
        }
    }

    /// Returns the context identifier associated with this node.
    /// Can be used to access and/or mutate this at a later time.
    pub fn id(&self) -> usize {
        self.id
    }
}

//
// Container
//

impl View {
    pub fn add(&mut self, child: View) {
        if let Some(context) = self.context.upgrade() {
            let inner = child.inner.clone();
            context.emplace_node(child);

            self.inner.add(inner);
        }
    }
}

//
// Events
//

impl View {
    pub fn mouse_down(&mut self) -> &mut Event<events::MouseDown> {
        self.inner.mouse_down()
    }

    pub fn mouse_up(&mut self) -> &mut Event<events::MouseUp> {
        self.inner.mouse_up()
    }

    pub fn mouse_enter(&mut self) -> &mut Event<events::MouseEnter> {
        self.inner.mouse_enter()
    }

    pub fn mouse_leave(&mut self) -> &mut Event<events::MouseLeave> {
        self.inner.mouse_leave()
    }
}

//
// Style
//

impl View {
    /// Sets the background color for this view.
    ///
    /// Default: `transparent` (`0x00_00_00_00`)
    pub fn set_background_color(&mut self, color: impl Into<Color>) {
        self.inner.set_background_color(color.into());
    }

    /// Sets the corner radius for this view.
    ///
    /// Default: `0`
    pub fn set_corner_radius(&mut self, radius: f32) {
        self.inner.set_corner_radius(radius);
    }

    /// Sets the position type for this View which determines how it is positioned
    /// within its parent.
    ///
    /// See: https://yogalayout.com/docs/absolute-relative-layout
    pub fn set_position_type(&mut self, position_type: PositionType) {
        unsafe {
            YGNodeStyleSetPositionType(self.inner.yoga_node(), position_type.into_yg());
        }

        self.inner.set_needs_layout();
    }

    /// Sets the relative or absolute (depending on position type) offset from the specified
    /// edge for this view.
    pub fn set_position(&mut self, edge: Edge, offset: f32) {
        unsafe {
            YGNodeStyleSetPosition(self.inner.yoga_node(), edge.into_yg(), offset);
        }

        self.inner.set_needs_layout();
    }

    /// Sets the content alignment for this view.
    ///
    /// Content alignment defines the distribution of lines along the cross-axis.
    /// This only has effect when items are wrapped to multiple lines.
    pub fn set_align_content(&mut self, align: Align) {
        unsafe {
            YGNodeStyleSetAlignContent(self.inner.yoga_node(), align.into_yg());
        }

        self.inner.set_needs_layout();
    }

    /// Sets the item alignment for this view.
    pub fn set_align_items(&mut self, align: Align) {
        unsafe {
            YGNodeStyleSetAlignItems(self.inner.yoga_node(), align.into_yg());
        }

        self.inner.set_needs_layout();
    }

    /// Sets the self alignment for this view.
    ///
    /// Overrides the item alignment on the parent of this view.
    pub fn set_align_self(&mut self, align: Align) {
        unsafe {
            YGNodeStyleSetAlignSelf(self.inner.yoga_node(), align.into_yg());
        }

        self.inner.set_needs_layout();
    }

    /// Sets the flex direction for this view.
    pub fn set_flex_direction(&mut self, flex_direction: FlexDirection) {
        unsafe {
            YGNodeStyleSetFlexDirection(self.inner.yoga_node(), flex_direction.into_yg());
        }

        self.inner.set_needs_layout();
    }

    /// Sets the wrap for this view.
    pub fn set_wrap(&mut self, wrap: Wrap) {
        unsafe {
            YGNodeStyleSetFlexWrap(self.inner.yoga_node(), wrap.into_yg());
        }

        self.inner.set_needs_layout();
    }

    /// Sets the flex grow for this view.
    ///
    /// Describes how any space within a container should be distributed among
    /// its children along the main axis. After laying out its children, a container
    /// will distribute any remaining space according to the flex grow values
    /// specified by its children.
    pub fn set_flex_grow(&mut self, flex_grow: f32) {
        unsafe {
            YGNodeStyleSetFlexGrow(self.inner.yoga_node(), flex_grow);
        }

        self.inner.set_needs_layout();
    }

    /// Sets the flex shrink for this view.
    ///
    /// Describes how to shrink children along the main axis in the case that the total size of
    /// the children overflow the size of the container on the main axis. flex shrink is very
    /// similar to flex grow and can be thought of in the same way if any overflowing size is
    /// considered to be negative remaining space. These two properties also work well
    /// together by allowing children to grow and shrink as needed.
    pub fn set_flex_shrink(&mut self, flex_shrink: f32) {
        unsafe {
            YGNodeStyleSetFlexShrink(self.inner.yoga_node(), flex_shrink);
        }

        self.inner.set_needs_layout();
    }

    /// Sets the flex basis for this view.
    ///
    /// An axis-independent way of providing the default size of an item along the main axis.
    pub fn set_flex_basis(&mut self, flex_basis: f32) {
        unsafe {
            YGNodeStyleSetFlexBasis(self.inner.yoga_node(), flex_basis);
        }

        self.inner.set_needs_layout();
    }

    /// Sets the content justification for this view.
    pub fn set_justify_content(&mut self, justify: Justify) {
        unsafe {
            YGNodeStyleSetJustifyContent(self.inner.yoga_node(), justify.into_yg());
        }

        self.inner.set_needs_layout();
    }

    /// Sets the margin of the specified edge(s) for this view.
    pub fn set_margin(&mut self, edge: Edge, margin: f32) {
        unsafe {
            YGNodeStyleSetMargin(self.inner.yoga_node(), edge.into_yg(), margin);
        }

        self.inner.set_needs_layout();
    }

    /// Sets the padding of the specified edge(s) for this view.
    pub fn set_padding(&mut self, edge: Edge, padding: f32) {
        unsafe {
            YGNodeStyleSetPadding(self.inner.yoga_node(), edge.into_yg(), padding);
        }

        self.inner.set_needs_layout();
    }

    /// Sets the minimum width (in pixels) for this view.
    pub fn set_min_width(&mut self, width: f32) {
        unsafe {
            YGNodeStyleSetMinWidth(self.inner.yoga_node(), width);
        }

        self.inner.set_needs_layout();
    }

    /// Sets the maximum width (in pixels) for this view.
    pub fn set_max_width(&mut self, width: f32) {
        unsafe {
            YGNodeStyleSetMaxWidth(self.inner.yoga_node(), width);
        }

        self.inner.set_needs_layout();
    }

    /// Sets the minimum height (in pixels) for this view.
    pub fn set_min_height(&mut self, height: f32) {
        unsafe {
            YGNodeStyleSetMinHeight(self.inner.yoga_node(), height);
        }

        self.inner.set_needs_layout();
    }

    /// Sets the maximum height (in pixels) for this view.
    pub fn set_max_height(&mut self, height: f32) {
        unsafe {
            YGNodeStyleSetMaxHeight(self.inner.yoga_node(), height);
        }

        self.inner.set_needs_layout();
    }

    /// Sets the width (in pixels) for this view.
    pub fn set_width(&mut self, width: f32) {
        unsafe {
            YGNodeStyleSetWidth(self.inner.yoga_node(), width);
        }

        self.inner.set_needs_layout();
    }

    /// Sets the width (in % of the parent) for this view.
    pub fn set_width_percent(&mut self, width: f32) {
        unsafe {
            YGNodeStyleSetWidthPercent(self.inner.yoga_node(), width * 100.);
        }

        self.inner.set_needs_layout();
    }

    /// Sets the height (in pixels) for this view.
    pub fn set_height(&mut self, height: f32) {
        unsafe {
            YGNodeStyleSetHeight(self.inner.yoga_node(), height);
        }

        self.inner.set_needs_layout();
    }

    /// Sets the height (in % of the parent) for this view.
    pub fn set_height_percent(&mut self, height: f32) {
        unsafe {
            YGNodeStyleSetHeightPercent(self.inner.yoga_node(), height * 100.);
        }

        self.inner.set_needs_layout();
    }
}
