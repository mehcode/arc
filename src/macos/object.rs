use cocoa::base::id;

#[doc(hidden)]
pub trait ObjCObject {
    #[inline]
    fn handle(&self) -> id;
}
