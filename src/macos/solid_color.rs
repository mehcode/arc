use cocoa::base::id;
use super::{view, ObjCObject, View};
use super::super::color::Color;
use super::solid_color_builder::SolidColorBuilder;

pub struct SolidColor(id);

impl SolidColor {
    pub fn new<T: Into<Color>>(color: T) -> Self {
        let ptr: id = unsafe { msg_send![*view::CLS, new] };
        let mut self_ = SolidColor(ptr);
        self_.set_background_color(color);

        self_
    }

    pub fn builder<T: Into<Color>>(color: T) -> SolidColorBuilder {
        SolidColorBuilder::new(color)
    }
}

impl ObjCObject for SolidColor {
    #[inline]
    fn handle(&self) -> id {
        self.0
    }
}

impl View for SolidColor {}
