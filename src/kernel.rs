use crate::{DotContext, NodeId};
use core::hash::Hash;
use std::fmt::Debug;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

#[derive(Clone, Debug, PartialEq, Eq)]

/// The kernel is a structure that performs operations of a crdt.
pub struct Kernel<E>
where
    E: Eq + Display + Clone + Hash + Debug,
{
    pub id: NodeId,
    pub elems: HashMap<NodeId, HashMap<(i64, i64), E>>, // (id, sck, tag, element).
    pub cc: DotContext<NodeId>,
}

impl<E> Kernel<E>
where
    E: Eq + Display + Clone + Hash + Debug,
{
    pub fn new(id: &NodeId) -> Self {
        Self {
            id: id.clone(),
            elems: HashMap::new(),
            cc: DotContext::new(),
        }
    }
    // --------------------------
    // STANDARD FUNCTIONS
    // --------------------------

    /// Returns the set of a node.
    pub fn get_set(&self, id: &NodeId) -> Option<&HashSet<(i64, i64, E)>> {
        todo!()
    }

    /// Removes all the entries related to the id.
    pub fn remove_id(&mut self, id: &NodeId) {
        todo!()
    }

    pub fn get_last_tag(&self, sck: i64) -> i64 {
        todo!()
    }
    // --------------------------
    // OPERATIONS
    // --------------------------

    /// TODO: to support self and other sets;
    pub fn elements(&self) -> HashSet<E> {
        todo!()
    }

    /// Adds an element with key equals to self.id and return the added entry.
    /// TODO: to test
    pub fn add(&mut self, element: E, sck: i64) -> (i64, i64, E) {
        let (_,_,n) = self.cc.makedot(&self.id, sck); 
        let key = (sck, n);

        self.elems
            .entry(self.id.clone())
            .and_modify(|hash| {
                hash.insert(key.clone(), element.clone());
            })
            .or_insert(HashMap::from([(key, element.clone())]));

        (sck, n, element)
    }

    /// TODO: to support self_set
    pub fn rm(&mut self, element: E) {
        todo!()
    }

    pub fn join(&mut self, other: &mut Self) {
        todo!()
    }

    // --------------------------
    // UTILS
    // --------------------------

    /// Returns true if the node has ever received information about it, and false otherwise.
    pub fn has_seen(&self, id: &NodeId) -> bool {
        self.cc.has_seen(id)
    }

}
