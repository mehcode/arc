use cocoa::base::YES;
use super::{ObjCObject, View};
use builder::Builder;

pub trait ViewGroup: ObjCObject {
    fn add<U: View, T: Builder<U>>(&mut self, view: T) {
        let view = view.build();

        unsafe {
            msg_send![self.handle(), addSubview:view.handle()];
            msg_send![self.handle(), setNeedsLayout: YES];
        }
    }
}
