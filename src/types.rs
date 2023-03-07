use crate::nodeId::NodeId;
use std::fmt::{Debug, Display};
use std::hash::Hash;



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ck{
    pub sck: i64,
    pub dck: i64
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TagElement<E: Eq + Clone + Hash + Debug + Display>{
    pub sck: i64,
    pub n: i64,
    pub elem: E
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Dot {
    pub id: NodeId,
    pub sck: i64,
    pub n: i64
}