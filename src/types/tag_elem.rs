use std::hash::Hash;
use std::fmt::{Debug, Display};
use std::mem::size_of;


use super::{Dot, NodeId};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TagElem<E: Eq + Clone + Hash + Debug + Display> {
    pub n: i64,
    pub elem: E,
}

impl<E: Eq + Clone + Hash + Debug + Display> TagElem<E> {
    pub fn new(n: i64, elem: E) -> Self {
        Self { n, elem }
    }
    
    /// Extracts the dot from the Taggesd element. 
    pub fn to_dot(&self, id: &NodeId) -> Dot {
        Dot::new(id.clone(), self.n)
    }

    pub fn get_num_bytes(&self) -> usize {
        return size_of::<i64>() + size_of::<i64>() + size_of::<E>();
    }
}

impl<E: Eq + Clone + Hash + Debug + Display> Display for TagElem<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.n, self.elem)
    }
}

impl<E: Eq + Clone + Hash + Debug + Display> Debug for TagElem<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}