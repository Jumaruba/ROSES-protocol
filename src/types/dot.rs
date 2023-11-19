use std::fmt::{Display, Debug};
use std::hash::Hash;
use std::mem::size_of;


use super::{Payload, NodeId};

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Dot {
    pub id: NodeId,
    pub n: i64,
}

impl Dot {
    pub fn new(id: NodeId, n: i64) -> Self {
        Self { id,  n }
    }

    pub fn to_tag<E: Eq + Clone + Hash + Debug + Display>(&self, elem: &E) -> Payload<E> {
        Payload::new(self.n, elem.clone())
    } 

    pub fn get_num_bytes(&self) -> usize{
        return self.id.get_num_bytes() + size_of::<i64>(); 
    }
}


impl Display for Dot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.id, self.n)
    }
}

impl Debug for Dot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

