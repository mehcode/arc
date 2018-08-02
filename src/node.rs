use super::os;
use downcast::{Any, *};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct NodeId(pub(crate) usize);

pub trait Node: os::Node + Send + Any {
    fn id(&self) -> NodeId;
}

downcast!(dyn Node);
