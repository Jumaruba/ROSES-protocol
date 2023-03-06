use std::fmt::{Debug, Display};
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TagElement<E: Eq + Clone + Hash + Debug + Display>{
    pub sck: i64,
    pub n: i64,
    pub elem: E
}