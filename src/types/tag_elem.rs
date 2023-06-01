use std::hash::Hash;
use std::fmt::{Debug, Display};
use std::mem::size_of;


use super::{Dot, NodeId};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TagElem<E: Eq + Clone + Hash + Debug + Display> {
    pub sck: i64,
    pub n: i64,
    pub elem: E,
}

impl<E: Eq + Clone + Hash + Debug + Display> TagElem<E> {
    pub fn new(sck: i64, n: i64, elem: E) -> Self {
        Self { sck, n, elem }
    }

    pub fn to_dot(&self, id: &NodeId) -> Dot {
        Dot::new(id.clone(), self.sck, self.n)
    }

    pub fn get_num_bytes(&self) -> usize {
        return size_of::<i64>() + size_of::<i64>() + size_of::<E>();
    }
}

impl<E: Eq + Clone + Hash + Debug + Display> Display for TagElem<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.sck, self.n, self.elem)
    }
}

impl<E: Eq + Clone + Hash + Debug + Display> Debug for TagElem<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}