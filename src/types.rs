use crate::nodeId::NodeId;
use std::fmt::{Debug, Display};
use std::hash::Hash;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ck {
    pub sck: i64,
    pub dck: i64,
}

impl Ck {
    pub fn new(sck: i64, dck: i64) -> Self {
        Self { sck, dck }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TagItem<E: Eq + Clone + Hash + Debug + Display> {
    pub sck: i64,
    pub n: i64,
    pub elem: E,
}

impl<E: Eq + Clone + Hash + Debug + Display> TagItem<E> {
    pub fn new(sck: i64, n: i64, elem: E) -> Self {
        Self { sck, n, elem }
    }

    pub fn to_dot(&self, id: &NodeId) -> Dot {
        Dot::new(id.clone(), self.sck, self.n)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Dot {
    pub id: NodeId,
    pub sck: i64,
    pub n: i64,
}

impl Dot {
    pub fn new(id: NodeId, sck: i64, n: i64) -> Self {
        Self { id, sck, n }
    }

    pub fn to_tag<E: Eq + Clone + Hash + Debug + Display>(&self, elem: &E) -> TagItem<E> {
        TagItem::new(self.sck, self.n, elem.clone())
    }
}
impl Display for Ck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.sck, self.dck)
    }
}

impl Debug for Ck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl<E: Eq + Clone + Hash + Debug + Display> Display for TagItem<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.sck, self.n, self.elem)
    }
}

impl<E: Eq + Clone + Hash + Debug + Display> Debug for TagItem<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Dot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.id, self.sck, self.n)
    }
}
