use crate::nodeId::NodeId;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Dot {
    pub id: NodeId,
    pub sck: i64,
    pub n: i64
}