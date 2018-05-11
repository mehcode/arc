use cocoa::base::id;
use super::view_box::ORIENTATION_HORIZONTAL;
use super::{ObjCObject, View, ViewBox, ViewGroup};

pub struct Row(ViewBox);

impl Row {
    pub fn new() -> Self {
        Row(ViewBox::new(ORIENTATION_HORIZONTAL))
    }
}

impl ObjCObject for Row {
    fn handle(&self) -> id {
        self.0.handle()
    }
}

impl View for Row {}
impl ViewGroup for Row {}
