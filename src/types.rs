use crate::nodeId::NodeId;
use std::fmt::{Debug, Display};
use std::hash::Hash;



#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ck{
    pub sck: i64,
    pub dck: i64
}

#[derive(Clone, PartialEq, Eq, Hash)]
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

impl Display for Ck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.sck, self.dck)
    }
}

impl Debug for Ck{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}


impl<E: Eq + Clone + Hash + Debug + Display> Display for TagElement<E>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.sck, self.n, self.elem)
    }
}


impl<E: Eq + Clone + Hash + Debug + Display> Debug for TagElement<E>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Dot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.id, self.sck, self.n)
    }
}