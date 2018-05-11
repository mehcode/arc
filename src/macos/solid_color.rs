use cocoa::base::id;
use super::{view, ObjCObject, View};

pub struct SolidColor(id);

impl SolidColor {
    pub fn new() -> Self {
        let ptr: id = unsafe { msg_send![*view::CLS, new] };

        SolidColor(ptr)
    }
}

impl ObjCObject for SolidColor {
    #[inline]
    fn handle(&self) -> id {
        self.0
    }
}

impl View for SolidColor {}
