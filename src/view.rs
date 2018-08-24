use super::{
    events,
    os::{self, Node as OsNode},
    Align, Color, Edge, Event, FlexDirection, Justify, Node, NodeId, PositionType, Wrap,
};
use yoga;

/// The fundamental component, `View` is a container that supports
/// layout with **Flexbox** powered by [Yoga](https://yogalayout.com/). View maps directly
/// to the native view equivalent of the platform (e.g. `NSView` in macOS).
///
/// `View` is designed to be ne sted inside other views and can have 0 to many children of
/// any type.
#[derive(Copy, Clone)]
pub struct View {
    pub(crate) id: NodeId,
}

impl View {
    pub fn new() -> Self {
        Self {
            id: os::Nodes::emplace(os::View::new()),
        }
    }

    //
    // Container
    // TODO: Documentation
    //

    pub fn add(&mut self, node: impl Node) {
        os::View::add(self.id(), node.id());
    }

    //
    // Style
    //

    /// Sets the background color for this node.
    ///
    /// Default: `transparent` (`0x00_00_00_00`)
    #[inline]
    pub fn set_background_color(&mut self, color: impl Into<Color>) {
        let color = color.into();

        os::Nodes::with(self.id, move |node: &mut os::View| {
            node.set_background_color(color);
        });
    }

    /// Sets the corner radius for this node.
    ///
    /// Default: `0`
    #[inline]
    pub fn set_corner_radius(&mut self, radius: f32) {
        os::Nodes::with(self.id, move |node: &mut os::View| {
            node.set_corner_radius(radius);
        });
    }

    //
    // Events
    // TODO: Documentation
    //

    #[inline]
    pub fn mouse_down(&mut self) -> Event<events::MouseDown> {
        os::Nodes::with(self.id, move |node: &mut os::View| {
            node.mouse_down().clone()
        })
    }

    #[inline]
    pub fn mouse_up(&mut self) -> Event<events::MouseUp> {
        os::Nodes::with(self.id, move |node: &mut os::View| node.mouse_up().clone())
    }

    #[inline]
    pub fn mouse_enter(&mut self) -> Event<events::MouseEnter> {
        os::Nodes::with(self.id, move |node: &mut os::View| {
            node.mouse_enter().clone()
        })
    }

    #[inline]
    pub fn mouse_leave(&mut self) -> Event<events::MouseLeave> {
        os::Nodes::with(self.id, move |node: &mut os::View| {
            node.mouse_leave().clone()
        })
    }
}

//
// Node
//

impl Node for View {
    #[inline]
    fn id(&self) -> NodeId {
        self.id
    }
}

//
// Layout
//

impl_layout!(View);
//impl_layout_container!(View);
