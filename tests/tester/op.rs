use std::hash::Hash;
use std::fmt::{Debug, Display};

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Op<E: Eq + Clone + Hash + Debug + Display> {
    RM(E),
    ADD(E),
}