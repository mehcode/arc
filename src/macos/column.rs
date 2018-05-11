use cocoa::base::id;
use super::view_box::ORIENTATION_VERTICAL;
use super::{ObjCObject, View, ViewBox, ViewGroup};

pub struct Column(ViewBox);

impl Column {
    pub fn new() -> Self {
        Column(ViewBox::new(ORIENTATION_VERTICAL))
    }
}

impl ObjCObject for Column {
    fn handle(&self) -> id {
        self.0.handle()
    }
}

impl View for Column {}
impl ViewGroup for Column {}
