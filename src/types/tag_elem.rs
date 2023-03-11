use std::hash::Hash;
use std::fmt::{Debug, Display};


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