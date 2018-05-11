use cocoa::base::YES;
use super::{ObjCObject, View};

pub trait ViewGroup: ObjCObject {
    fn add<V: View>(&mut self, view: V) {
        unsafe {
            msg_send![self.handle(), addSubview:view.handle()];
            msg_send![self.handle(), setNeedsLayout: YES];
        }
    }
}
