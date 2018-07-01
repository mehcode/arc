use cocoa::base::id;

pub(crate) type NodeHandle = id;

#[doc(hidden)]
pub trait Node {
    fn handle(&self) -> NodeHandle;
}
