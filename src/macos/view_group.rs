use cocoa::base::YES;
use super::{ObjCObject, View};

pub trait ViewGroup: ObjCObject {
    fn add<T: View>(&mut self, view: T) {
        unsafe {
            msg_send![self.handle(), addSubview:view.handle()];
            msg_send![self.handle(), setNeedsLayout: YES];
        }
    }
}
