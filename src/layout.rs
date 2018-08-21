use bitflags::*;

bitflags! {
    pub struct Gravity: u16 {
        /// Push content to the top of its container.
        const TOP = 0x00_30;

        /// Push content to the bottom of its container.
        const BOTTOM = 0x00_50;

        /// Push content to the start (left in LTR or right in RTL) of its container.
        const START = 0x08_03;

        /// Push content to the end (right in LTR or left in RTL) of its container.
        const END = 0x08_05;

        /// Push content to the left of its container.
        const LEFT = 0x00_03;

        /// Push content to the right of its container.
        const RIGHT = 0x00_05;

        /// Place content in the horizontal center of its container.
        const CENTER_HORIZONTAL = 0x00_01;

        /// Place content in the vertical center of its container.
        const CENTER_VERTICAL = 0x00_10;

        /// Place content in the horizontal and vertical center of its container.
        const CENTER = 0x00_01 | 0x00_10;
    }
}

macro_rules! impl_layout {
    ($ty:ident) => {
        impl $ty {
            /// Sets the position type for this View which determines how it is positioned
            /// within its parent.
            ///
            /// See: https://yogalayout.com/docs/absolute-relative-layout
            pub fn set_position_type(&mut self, position_type: PositionType) {
                crate::os::Nodes::with_untyped(self.inner, move |node| {
                    node.yoga().set_position_type(position_type);
                    node.set_needs_layout();
                })
            }

            /// Sets the relative or absolute (depending on position type) offset from the specified
            /// edge for this view.
            pub fn set_position(&mut self, edge: Edge, offset: f32) {
                crate::os::Nodes::with_untyped(self.inner, move |node| {
                    node.yoga()
                        .set_position(edge, yoga::StyleUnit::Point(offset.into()));

                    node.set_needs_layout();
                })
            }

            /// Sets the self alignment for this view.
            ///
            /// Overrides the item alignment on the parent of this view.
            pub fn set_align_self(&mut self, align: Align) {
                crate::os::Nodes::with_untyped(self.inner, move |node| {
                    node.yoga().set_align_self(align);
                    node.set_needs_layout();
                })
            }

            /// Sets the flex grow for this view.
            ///
            /// Describes how any space within a container should be distributed among
            /// its children along the main axis. After laying out its children, a container
            /// will distribute any remaining space according to the flex grow values
            /// specified by its children.
            pub fn set_flex_grow(&mut self, flex_grow: f32) {
                crate::os::Nodes::with_untyped(self.inner, move |node| {
                    node.yoga().set_flex_grow(flex_grow);
                    node.set_needs_layout();
                })
            }

            /// Sets the flex shrink for this view.
            ///
            /// Describes how to shrink children along the main axis in the case that the total size of
            /// the children overflow the size of the container on the main axis. flex shrink is very
            /// similar to flex grow and can be thought of in the same way if any overflowing size is
            /// considered to be negative remaining space. These two properties also work well
            /// together by allowing children to grow and shrink as needed.
            pub fn set_flex_shrink(&mut self, flex_shrink: f32) {
                crate::os::Nodes::with_untyped(self.inner, move |node| {
                    node.yoga().set_flex_shrink(flex_shrink);
                    node.set_needs_layout();
                })
            }

            /// Sets the flex basis for this view.
            ///
            /// An axis-independent way of providing the default size of an item along the main axis.
            pub fn set_flex_basis(&mut self, flex_basis: f32) {
                crate::os::Nodes::with_untyped(self.inner, move |node| {
                    node.yoga()
                        .set_flex_basis(yoga::StyleUnit::Point(flex_basis.into()));

                    node.set_needs_layout();
                })
            }

            /// Sets the margin of the specified edge(s) for this view.
            pub fn set_margin(&mut self, edge: Edge, margin: f32) {
                crate::os::Nodes::with_untyped(self.inner, move |node| {
                    node.yoga()
                        .set_margin(edge, yoga::StyleUnit::Point(margin.into()));

                    node.set_needs_layout();
                })
            }

            /// Sets the padding of the specified edge(s) for this view.
            pub fn set_padding(&mut self, edge: Edge, padding: f32) {
                crate::os::Nodes::with_untyped(self.inner, move |node| {
                    node.yoga()
                        .set_padding(edge, yoga::StyleUnit::Point(padding.into()));

                    node.set_needs_layout();
                })
            }

            /// Sets the minimum width (in pixels) for this view.
            pub fn set_min_width(&mut self, width: f32) {
                crate::os::Nodes::with_untyped(self.inner, move |node| {
                    node.yoga()
                        .set_min_width(yoga::StyleUnit::Point(width.into()));

                    node.set_needs_layout();
                })
            }

            /// Sets the maximum width (in pixels) for this view.
            pub fn set_max_width(&mut self, width: f32) {
                crate::os::Nodes::with_untyped(self.inner, move |node| {
                    node.yoga()
                        .set_max_width(yoga::StyleUnit::Point(width.into()));

                    node.set_needs_layout();
                })
            }

            /// Sets the minimum height (in pixels) for this view.
            pub fn set_min_height(&mut self, height: f32) {
                crate::os::Nodes::with_untyped(self.inner, move |node| {
                    node.yoga()
                        .set_min_height(yoga::StyleUnit::Point(height.into()));

                    node.set_needs_layout();
                })
            }

            /// Sets the maximum height (in pixels) for this view.
            pub fn set_max_height(&mut self, height: f32) {
                crate::os::Nodes::with_untyped(self.inner, move |node| {
                    node.yoga()
                        .set_max_height(yoga::StyleUnit::Point(height.into()));

                    node.set_needs_layout();
                })
            }

            /// Sets the width (in pixels) for this view.
            pub fn set_width(&mut self, width: f32) {
                crate::os::Nodes::with_untyped(self.inner, move |node| {
                    node.yoga().set_width(yoga::StyleUnit::Point(width.into()));

                    node.set_needs_layout();
                })
            }

            /// Sets the width (in % of the parent) for this view.
            pub fn set_width_percent(&mut self, width: f32) {
                crate::os::Nodes::with_untyped(self.inner, move |node| {
                    // TODO: Get feedback if this behavior makes sense
                    node.yoga()
                        .set_width(yoga::StyleUnit::Percent((width * 100.).into()));

                    node.set_needs_layout();
                })
            }

            /// Sets the height (in pixels) for this view.
            pub fn set_height(&mut self, height: f32) {
                crate::os::Nodes::with_untyped(self.inner, move |node| {
                    node.yoga()
                        .set_height(yoga::StyleUnit::Point(height.into()));

                    node.set_needs_layout();
                })
            }

            /// Sets the height (in % of the parent) for this view.
            pub fn set_height_percent(&mut self, height: f32) {
                crate::os::Nodes::with_untyped(self.inner, move |node| {
                    // TODO: Get feedback if this behavior makes sense
                    node.yoga()
                        .set_height(yoga::StyleUnit::Percent((height * 100.).into()));

                    node.set_needs_layout();
                })
            }
        }
    };
}

macro_rules! impl_layout_container {
    ($ty:ident) => {
        impl $ty {
            /// Sets the content alignment for this view.
            ///
            /// Content alignment defines the distribution of lines along the cross-axis.
            /// This only has effect when items are wrapped to multiple lines.
            pub fn set_align_content(&mut self, align: Align) {
                crate::os::Nodes::with_untyped(self.inner, move |node| {
                    node.yoga().set_align_content(align);
                    node.set_needs_layout();
                })
            }

            /// Sets the item alignment for this view.
            pub fn set_align_items(&mut self, align: Align) {
                crate::os::Nodes::with_untyped(self.inner, move |node| {
                    node.yoga().set_align_items(align);
                    node.set_needs_layout();
                })
            }

            /// Sets the flex direction for this view.
            pub fn set_flex_direction(&mut self, flex_direction: FlexDirection) {
                crate::os::Nodes::with_untyped(self.inner, move |node| {
                    node.yoga().set_flex_direction(flex_direction);
                    node.set_needs_layout();
                })
            }

            /// Sets the flex wrap for this view.
            pub fn set_flex_wrap(&mut self, wrap: Wrap) {
                crate::os::Nodes::with_untyped(self.inner, move |node| {
                    node.yoga().set_flex_wrap(wrap);
                    node.set_needs_layout();
                })
            }

            /// Sets the content justification for this view.
            pub fn set_justify_content(&mut self, justify: Justify) {
                crate::os::Nodes::with_untyped(self.inner, move |node| {
                    node.yoga().set_justify_content(justify);
                    node.set_needs_layout();
                })
            }
        }
    };
}
