use yoga_sys::{YGAlign, YGEdge, YGFlexDirection, YGJustify, YGPositionType, YGWrap};

/// Position type describes how to position within the parent.
///
/// Default: `Relative`
#[derive(Debug)]
pub enum PositionType {
    Relative,
    Absolute,
}

impl PositionType {
    pub(crate) fn into_yg(self) -> YGPositionType {
        use self::YGPositionType::*;

        match self {
            PositionType::Relative => YGPositionTypeRelative,
            PositionType::Absolute => YGPositionTypeAbsolute,
        }
    }
}

/// Align describes how to align children along the cross axis of their container.
#[derive(Debug)]
pub enum Align {
    Auto,

    /// Align children of a container to the start of the container's cross axis.
    Start,

    /// Align children of a container to the end of the container's cross axis.
    End,

    /// Stretch children of a container to match the height of the container's cross axis (default).
    Stretch,

    /// Align children of a container in the center of the container's cross axis.
    Center,

    Baseline,

    /// Evenly space of children across the container's cross axis, distributing remaining space
    /// between the children.
    SpaceBetween,

    /// Evenly space of children across the container's cross axis, distributing remaining space
    /// around the children.
    SpaceAround,
}

impl Align {
    pub(crate) fn into_yg(self) -> YGAlign {
        use self::YGAlign::*;

        match self {
            Align::Auto => YGAlignAuto,
            Align::Start => YGAlignFlexStart,
            Align::End => YGAlignFlexEnd,
            Align::Center => YGAlignCenter,
            Align::SpaceAround => YGAlignSpaceAround,
            Align::SpaceBetween => YGAlignSpaceBetween,
            Align::Stretch => YGAlignStretch,
            Align::Baseline => YGAlignBaseline,
        }
    }
}

/// Justify content describes how to align children within the main axis of their container.
#[derive(Debug)]
pub enum Justify {
    /// Align children of a container to the start of the container's main axis. (default).
    Start,

    /// Align children of a container to the end of the container's main axis.
    End,

    /// Align children of a container in the center of the container's main axis.
    Center,

    /// Evenly space of children across the container's main axis, distributing remaining space
    /// between the children.
    SpaceBetween,

    /// Evenly space of children across the container's main axis, distributing remaining space
    /// around the children.
    SpaceAround,

    SpaceEvenly,
}

impl Justify {
    pub(crate) fn into_yg(self) -> YGJustify {
        use self::YGJustify::*;

        match self {
            Justify::Start => YGJustifyFlexStart,
            Justify::End => YGJustifyFlexEnd,
            Justify::Center => YGJustifyCenter,
            Justify::SpaceAround => YGJustifySpaceAround,
            Justify::SpaceBetween => YGJustifySpaceBetween,
            Justify::SpaceEvenly => YGJustifySpaceEvenly,
        }
    }
}

/// Wrap controls what happens when children overflow
/// the size of the container along the main axis. By default children are forced
/// into a single line (which can shrink elements).
#[derive(Debug)]
pub enum Wrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

impl Wrap {
    pub(crate) fn into_yg(self) -> YGWrap {
        use self::YGWrap::*;

        match self {
            Wrap::Wrap => YGWrapWrap,
            Wrap::WrapReverse => YGWrapWrapReverse,
            Wrap::NoWrap => YGWrapNoWrap,
        }
    }
}

/// Flex direction controls the direction in which children of a node are laid out.
/// This is also referred to as the main axis.
#[derive(Debug)]
pub enum FlexDirection {
    /// Align children from top to bottom.
    Column,

    /// Align children from bottom to top.
    ColumnReverse,

    /// Align children from start to end.
    Row,

    /// Align children from end to start.
    RowReverse,
}

impl FlexDirection {
    pub(crate) fn into_yg(self) -> YGFlexDirection {
        use self::YGFlexDirection::*;

        match self {
            FlexDirection::Row => YGFlexDirectionRow,
            FlexDirection::RowReverse => YGFlexDirectionRowReverse,
            FlexDirection::Column => YGFlexDirectionColumn,
            FlexDirection::ColumnReverse => YGFlexDirectionColumnReverse,
        }
    }
}

#[derive(Debug)]
pub enum Edge {
    Left,
    Top,
    Right,
    Bottom,
    Start,
    End,
    Horizontal,
    Vertical,
    All,
}

impl Edge {
    pub(crate) fn into_yg(self) -> YGEdge {
        use self::YGEdge::*;

        match self {
            Edge::Left => YGEdgeLeft,
            Edge::Top => YGEdgeTop,
            Edge::Right => YGEdgeRight,
            Edge::Bottom => YGEdgeBottom,
            Edge::Start => YGEdgeStart,
            Edge::End => YGEdgeEnd,
            Edge::Horizontal => YGEdgeHorizontal,
            Edge::Vertical => YGEdgeVertical,
            Edge::All => YGEdgeAll,
        }
    }
}
