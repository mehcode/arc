use super::{
    events,
    os::{self, Node as OsNode},
    Align, Color, Context, Edge, Event, Font, Gravity, Node, NodeId, PositionType,
};
use yoga;

pub struct Text {
    pub(crate) id: NodeId,
    pub(crate) inner: os::Text,
}

impl Text {
    pub fn new(context: &Context) -> Self {
        Self {
            id: context.next_id(),
            inner: os::Text::new(),
        }
    }

    #[inline]
    pub fn set_text(&mut self, text: impl AsRef<str>) {
        self.inner.set_text(text.as_ref());
    }

    //
    // Layout
    //

    #[inline]
    pub fn set_gravity(&mut self, gravity: Gravity) {
        self.inner.set_gravity(gravity);
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
    // Style: Text
    //

    #[inline]
    pub fn set_text_color(&mut self, color: impl Into<Color>) {
        self.inner.set_text_color(color.into());
    }

    #[inline]
    pub fn set_text_size(&mut self, size: f32) {
        self.inner.set_text_size(size);
    }

    //
    // Style: Font
    //

    #[inline]
    pub fn set_font(&mut self, font: &Font) {
        self.inner.set_font(&font.inner);
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

impl Node for Text {
    #[inline]
    fn id(&self) -> NodeId {
        self.id
    }
}

impl os::Node for Text {
    #[inline]
    fn handle(&self) -> os::NodeHandle {
        self.inner.handle()
    }
}

//
// Layout
//

impl_layout!(Text);
