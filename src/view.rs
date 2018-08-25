use super::{context::emplace, events, os, Color, Event, Node, NodeId};
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
            id: emplace(os::View::new()),
        }
    }

    //
    // Container
    // TODO: Documentation
    //

    pub fn add(&mut self, node: impl Node) {
        let incoming_id = node.id();

        self.id.with::<os::View, _, _>(move |this| {
            incoming_id.with_any(move |incoming| {
                this.add(incoming);
            });
        });
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

        self.id.with_mut::<os::View, _, _>(move |node| {
            node.set_background_color(color);
        });
    }

    /// Sets the corner radius for this node.
    ///
    /// Default: `0`
    #[inline]
    pub fn set_corner_radius(&mut self, radius: f32) {
        self.id.with_mut::<os::View, _, _>(move |node| {
            node.set_corner_radius(radius);
        });
    }

    //
    // Events
    // TODO: Rethink API here
    //

    #[inline]
    pub fn mouse_down(&mut self) -> Event<events::MouseDown> {
        self.id
            .with_mut::<os::View, _, _>(move |node| node.mouse_down().clone())
    }

    #[inline]
    pub fn mouse_up(&mut self) -> Event<events::MouseUp> {
        self.id
            .with_mut::<os::View, _, _>(move |node| node.mouse_up().clone())
    }

    #[inline]
    pub fn mouse_enter(&mut self) -> Event<events::MouseEnter> {
        self.id
            .with_mut::<os::View, _, _>(move |node| node.mouse_enter().clone())
    }

    #[inline]
    pub fn mouse_leave(&mut self) -> Event<events::MouseLeave> {
        self.id
            .with_mut::<os::View, _, _>(move |node| node.mouse_leave().clone())
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
impl_layout_container!(View);
