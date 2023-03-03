use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::hash::Hash;

use crate::kernel::Kernel;
use crate::nodeId::NodeId;

#[derive(Debug)]
pub struct HandoffAworSet<E: Eq + Clone + Hash + Debug + Display> {
    id: NodeId,
    kernel: Kernel<E>, // Stores information received from lower tiers.
    sck: i64,          // Source clock.
    dck: i64,          // Destination clock.
    pub slots: HashMap<NodeId, (i64, i64)>, // Slots {id -> (sck, dck)}
    tokens: HashMap<(NodeId, NodeId), ((i64, i64), i64, HashSet<(i64,i64,E)>)>, // (sck, dck, tag, (sck, tag, E))
    pub transl: HashSet<(NodeId, i64, i64, NodeId, i64, i64)>, // (id_src, sck_src_clock, counter_src, id_dst, sck_dst_clock_ counter_dst)
    tier: i32,
}

impl<E: Eq + Clone + Hash + Debug + Display> HandoffAworSet<E> {
    pub fn new(id: NodeId, tier: i32) -> Self {
        Self {
            id: id.clone(),
            kernel: Kernel::new(&id, 0),
            sck: 0,
            dck: 0,
            slots: HashMap::new(),
            tokens: HashMap::new(),
            transl: HashSet::new(),
            tier,
        }
    }

    /// Returns all the elements known by the node.
    /// Must be the combination of the elements in the token and in the set.
    pub fn fetch(&self) -> HashSet<E> {
        todo!()
    }

    /// Gets all the elements from the token
    fn get_token_elements(&self) -> HashSet<E> {
        todo!()
    }

    /// Adds an element to the node.
    pub fn add(&mut self, element: E) -> (E, i64) {
        self.kernel.add(element)
    }

    /// Creates a slot.
    /// TODO: test
    pub fn create_slot(&mut self, other: &Self) {
        todo!()
    }

    /// Creates a token in case there is a match slot in the other node.
    /// TODO: Test
    pub fn create_token(&mut self, other: &Self) {
        todo!()
    }

    /// Set causal context and set associated to self.id to empty.
    fn empty_self(&mut self) {
        self.kernel = Kernel::new(&self.id, self.sck);
    }

    pub fn fill_slots(&mut self, other: &Self) {
    }

    /// Merges the tokens elements with the actual state.
    /// A correct kernel contains only elements created in the source node.
    fn add_tokens(&mut self, other: &Self) {
        if let Some(set) = other.kernel.set.get(&other.id) {
            for triple in set {
                let (_, tag_dst) = self.add(triple.2.clone());
                self.create_translation(&other.id, triple, tag_dst);
            }
        }
    }

    fn create_translation(&mut self, other_id: &NodeId, triple: &(i64, i64, E), tag_dst: i64) {
        self.transl.insert((
            other_id.clone(),
            triple.0,
            triple.1,
            self.id.clone(),
            self.sck,
            tag_dst,
        ));
    }

    /// Discards a slot that can never be filled, since sck is higher than the one marked in the slot.
    pub fn discard_slot(&mut self, other: &Self) {
        if let Some(&(src, _)) = self.slots.get(&other.id) {
            if other.sck > src {
                self.slots.remove(&other.id);
            }
        }
    }

    /// Discard tokens that were already used or are out of date.
    pub fn discard_tokens(&mut self, other: &Self) {
        todo!()
    }

    /// Updates the values in set and cc.
    pub fn aggregate(&mut self, other: &Self) {}

    /// Applies translatiosn that came from the other node.
    pub fn translate(&mut self, other: &Self) {
        if self.tier >= other.tier {}
    }
}
