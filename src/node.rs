use super::os;
use downcast::{Any, *};

#[cfg_attr(feature = "cargo-clippy", allow(stutter))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct NodeId(pub(crate) usize);

pub trait Node: os::Node + Send + Any {
    fn id(&self) -> NodeId;
}

downcast!(dyn Node);
