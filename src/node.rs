use crate::NodeId;

pub trait Node: Send {
    fn id(&self) -> NodeId;
}
