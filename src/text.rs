use super::{
    os::{self, Node as OsNode},
    Align, Color, Context, Edge, Node, NodeId, PositionType,
};
use yoga;

pub struct Text {
    pub(crate) id: NodeId,
    pub(crate) inner: os::Text,
    // pub(crate) context: WeakContext,
}

impl Text {
    pub fn new(context: &Context) -> Self {
        Self {
            id: context.next_id(),
            inner: os::Text::new(),
            // context: context.downgrade(),
        }
    }

    //
    // Text
    //

    pub fn set_text(&mut self, text: &str) {
        self.inner.set_text(text);
    }

    pub fn set_text_color(&mut self, color: impl Into<Color>) {
        self.inner.set_text_color(color.into());
    }

    //
    // Font
    //

    pub fn set_font_size(&mut self, size: f32) {
        self.inner.set_font_size(size);
    }
}

//
// Node
//

impl Node for Text {
    fn id(&self) -> NodeId {
        self.id
    }
}

impl os::Node for Text {
    fn handle(&self) -> os::NodeHandle {
        self.inner.handle()
    }
}

impl From<Text> for NodeId {
    fn from(node: Text) -> NodeId {
        node.id()
    }
}

//
// Style
//

impl Text {
    /// Sets the background color for this node.
    ///
    /// Default: `transparent` (`0x00_00_00_00`)
    pub fn set_background_color(&mut self, color: impl Into<Color>) {
        self.inner.set_background_color(color.into());
    }

    // Sets the corner radius for this node.
    //
    // Default: `0`
    // pub fn set_corner_radius(&mut self, radius: f32) {
    //    self.inner.set_corner_radius(radius);
    // }
}

//
// Layout – Self
//

impl Text {
    /// Sets the position type for this View which determines how it is positioned
    /// within its parent.
    ///
    /// See: https://yogalayout.com/docs/absolute-relative-layout
    pub fn set_position_type(&mut self, position_type: PositionType) {
        self.inner.yoga().set_position_type(position_type);
        self.inner.set_needs_layout();
    }

    /// Sets the relative or absolute (depending on position type) offset from the specified
    /// edge for this view.
    pub fn set_position(&mut self, edge: Edge, offset: f32) {
        self.inner
            .yoga()
            .set_position(edge, yoga::StyleUnit::Point(offset.into()));

        self.inner.set_needs_layout();
    }

    /// Sets the self alignment for this view.
    ///
    /// Overrides the item alignment on the parent of this view.
    pub fn set_align_self(&mut self, align: Align) {
        self.inner.yoga().set_align_self(align);
        self.inner.set_needs_layout();
    }

    /// Sets the flex grow for this view.
    ///
    /// Describes how any space within a container should be distributed among
    /// its children along the main axis. After laying out its children, a container
    /// will distribute any remaining space according to the flex grow values
    /// specified by its children.
    pub fn set_flex_grow(&mut self, flex_grow: f32) {
        self.inner.yoga().set_flex_grow(flex_grow);
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
        self.inner.yoga().set_flex_shrink(flex_shrink);
        self.inner.set_needs_layout();
    }

    /// Sets the flex basis for this view.
    ///
    /// An axis-independent way of providing the default size of an item along the main axis.
    pub fn set_flex_basis(&mut self, flex_basis: f32) {
        self.inner
            .yoga()
            .set_flex_basis(yoga::StyleUnit::Point(flex_basis.into()));

        self.inner.set_needs_layout();
    }

    /// Sets the margin of the specified edge(s) for this view.
    pub fn set_margin(&mut self, edge: Edge, margin: f32) {
        self.inner
            .yoga()
            .set_margin(edge, yoga::StyleUnit::Point(margin.into()));

        self.inner.set_needs_layout();
    }

    /// Sets the padding of the specified edge(s) for this view.
    pub fn set_padding(&mut self, edge: Edge, padding: f32) {
        self.inner
            .yoga()
            .set_padding(edge, yoga::StyleUnit::Point(padding.into()));

        self.inner.set_needs_layout();
    }

    /// Sets the minimum width (in pixels) for this view.
    pub fn set_min_width(&mut self, width: f32) {
        self.inner
            .yoga()
            .set_min_width(yoga::StyleUnit::Point(width.into()));

        self.inner.set_needs_layout();
    }

    /// Sets the maximum width (in pixels) for this view.
    pub fn set_max_width(&mut self, width: f32) {
        self.inner
            .yoga()
            .set_max_width(yoga::StyleUnit::Point(width.into()));

        self.inner.set_needs_layout();
    }

    /// Sets the minimum height (in pixels) for this view.
    pub fn set_min_height(&mut self, height: f32) {
        self.inner
            .yoga()
            .set_min_height(yoga::StyleUnit::Point(height.into()));

        self.inner.set_needs_layout();
    }

    /// Sets the maximum height (in pixels) for this view.
    pub fn set_max_height(&mut self, height: f32) {
        self.inner
            .yoga()
            .set_max_height(yoga::StyleUnit::Point(height.into()));
        self.inner.set_needs_layout();
    }

    /// Sets the width (in pixels) for this view.
    pub fn set_width(&mut self, width: f32) {
        self.inner
            .yoga()
            .set_width(yoga::StyleUnit::Point(width.into()));

        self.inner.set_needs_layout();
    }

    /// Sets the width (in % of the parent) for this view.
    pub fn set_width_percent(&mut self, width: f32) {
        self.inner
            .yoga()
            .set_width(yoga::StyleUnit::Percent(width.into()));

        self.inner.set_needs_layout();
    }

    /// Sets the height (in pixels) for this view.
    pub fn set_height(&mut self, height: f32) {
        self.inner
            .yoga()
            .set_height(yoga::StyleUnit::Point(height.into()));

        self.inner.set_needs_layout();
    }

    /// Sets the height (in % of the parent) for this view.
    pub fn set_height_percent(&mut self, height: f32) {
        self.inner
            .yoga()
            .set_height(yoga::StyleUnit::Percent(height.into()));

        self.inner.set_needs_layout();
    }
}
