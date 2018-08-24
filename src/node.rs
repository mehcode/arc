use crate::NodeId;

pub trait Node {
    fn id(&self) -> NodeId;
}
