use std::fmt::{Display, Debug};
use std::hash::Hash;

use super::{TagElem, NodeId};

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Dot {
    pub id: NodeId,
    pub sck: i64,
    pub n: i64,
}

impl Dot {
    pub fn new(id: NodeId, sck: i64, n: i64) -> Self {
        Self { id, sck, n }
    }

    pub fn to_tag<E: Eq + Clone + Hash + Debug + Display>(&self, elem: &E) -> TagElem<E> {
        TagElem::new(self.sck, self.n, elem.clone())
    }
}


impl Display for Dot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.id, self.sck, self.n)
    }
}

impl Debug for Dot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

