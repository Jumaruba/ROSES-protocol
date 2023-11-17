use std::fmt::{Display, Debug};
use std::hash::Hash;
use std::mem::size_of;


use super::{NodeId, Dot};

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct TDot {
    pub id: NodeId, 
    pub sck: i64,
    pub n: i64
}

impl TDot {
    pub fn new(id: NodeId, sck: i64, n: i64) -> Self {
        Self {id, sck, n}
    }

    pub fn to_dot(&self) -> Dot {
        return Dot::new(self.id.clone(), self.n)
    }

    pub fn get_num_bytes(&self) -> usize{
        return self.id.get_num_bytes() + size_of::<i64>() + size_of::<i64>(); 
    }
}

impl Display for TDot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{}, {})", self.id, self.sck, self.n)
    }
}

impl Debug for TDot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}