use super::{
    context::WeakContext,
    events,
    os::{self, Node as OsNode},
    Align, Color, Context, Edge, Event, FlexDirection, Justify, Node, NodeId, PositionType, Wrap,
};
use yoga;

/// The fundamental component, `View` is a container that supports
/// layout with **Flexbox** powered by [Yoga](https://yogalayout.com/). View maps directly
/// to the native view equivalent of the platform (e.g. `NSView` in macOS).
///
/// `View` is designed to be ne sted inside other views and can have 0 to many children of
/// any type.
pub struct View {
    pub(crate) id: NodeId,
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

    //
    // Container
    // TODO: Documentation
    //

    pub fn add(&mut self, node: impl Node) {
        // TODO: It _should_ be impossible to have a view with a `WeakContext` that lives
        //       longer than a `Context` as view construction requires `&Context`. Should
        //       we panic instead of doing nothing here?

        if let Some(context) = self.context.upgrade() {
            self.inner.add(&node);
            context.emplace_node(node);
        }
    }

    //
    // Style
    //

    /// Sets the background color for this node.
    ///
    /// Default: `transparent` (`0x00_00_00_00`)
    #[inline]
    pub fn set_background_color(&mut self, color: impl Into<Color>) {
        self.inner.set_background_color(color.into());
    }

    /// Sets the corner radius for this node.
    ///
    /// Default: `0`
    #[inline]
    pub fn set_corner_radius(&mut self, radius: f32) {
        self.inner.set_corner_radius(radius);
    }

    //
    // Events
    // TODO: Documentation
    //

    #[inline]
    pub fn mouse_down(&mut self) -> &mut Event<events::MouseDown> {
        self.inner.mouse_down()
    }

    #[inline]
    pub fn mouse_up(&mut self) -> &mut Event<events::MouseUp> {
        self.inner.mouse_up()
    }

    #[inline]
    pub fn mouse_enter(&mut self) -> &mut Event<events::MouseEnter> {
        self.inner.mouse_enter()
    }

    #[inline]
    pub fn mouse_leave(&mut self) -> &mut Event<events::MouseLeave> {
        self.inner.mouse_leave()
    }
}

//
// Node
//

impl Node for View {
    fn id(&self) -> NodeId {
        self.id
    }
}

impl os::Node for View {
    fn handle(&self) -> os::NodeHandle {
        self.inner.handle()
    }
}

//
// Layout
//

impl_layout!(View);
impl_layout_container!(View);
