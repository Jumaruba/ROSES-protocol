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
    pub elems: HashMap<NodeId, HashSet<(i64, i64, E)>>, // (id, sck, tag, element).
    cc: DotContext<NodeId>,
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
    // Functions that modifies the structures.
    // --------------------------

    /// Returns the set of a node.
    pub fn get_set(&self, id: &NodeId) -> Option<&HashSet<(i64, i64, E)>> {
        self.elems.get(id)
    }

    /// Removes all the entries related to the id.
    /// Cleans both elements and dot context.
    /// TODO: To test
    pub fn clean_id(&mut self, id: &NodeId) {
        self.elems.remove(id);
        self.cc.clean_id(id);
    }

    pub fn get_last_tag(&self, sck: i64) -> i64 {
        todo!()
    }

    /// Gets the value in the causal context.
    pub fn get_self_cc_n(&self, sck: &i64) -> i64 {
        self.cc.get_cc_n(&self.id, sck)
    }
    
    pub fn get_cc(&self) -> HashSet<(NodeId, i64, i64)> {
        self.cc.get_cc(&self.id)
    }
    // --------------------------
    // OPERATIONS
    // CRDT's core operations. 
    // --------------------------

    /// Gets elements of the kernel.
    /// TODO: to test
    pub fn elements(&self) -> HashSet<E> {
        let mut res: HashSet<E> = HashSet::new();
        for (_, hash) in self.elems.iter() {
            hash.iter().for_each(|(_, _, e)| {
                res.insert(e.clone());
            });
        }
        res
    }

    /// Adds an element with key equals to self.id and return the added entry.
    /// TODO: to test
    pub fn add(&mut self, element: E, sck: i64) -> (i64, i64, E) {
        let (_, _, n) = self.cc.makedot(&self.id, sck);
        let entry: (i64, i64, E) = (sck, n, element);

        self.elems
            .entry(self.id.clone())
            .and_modify(|set| {
                set.insert(entry.clone());
            })
            .or_insert(HashSet::from([entry.clone()]));

        entry
    }

    /// Remove an element from the set of elements. 
    /// TODO: to test
    pub fn rm(&mut self, elem: &E) {
        self.elems.iter_mut().for_each(|(_, set)| {
            *set = set
                .drain()
                .filter(|(_, _, s_elem)| {
                    return *elem == *s_elem;
                })
                .collect();
        });
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
