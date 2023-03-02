use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::hash::Hash;

use crate::kernel::Kernel;
use crate::nodeId::NodeId;

#[derive(Debug)]
pub struct HandoffAworSet<E: Eq + Clone + Hash + Debug + Display> {
    id: NodeId,
    local_state: Kernel<E>,  // Stores information that was added locally.
    global_state: Kernel<E>, // Stores information received from lower tiers.
    sck: i64,                // Source clock.
    dck: i64,                // Destination clock.
    pub slots: HashMap<NodeId, (i64, i64)>, // Slots {id -> (sck, dck)}
    tokens: HashMap<(NodeId, NodeId), ((i64, i64), Kernel<E>)>, // The kernel is tipically the local_state.
    pub transl: HashSet<(NodeId, i64, i64, NodeId, i64, i64)>, // (id_src, sck_src_clock, counter_src, id_dst, sck_dst_clock_ counter_dst)
    tier: i32,
}

impl<E: Eq + Clone + Hash + Debug + Display> HandoffAworSet<E> {
    pub fn new(id: NodeId, tier: i32) -> Self {
        Self {
            id: id.clone(),
            local_state: Kernel::new(&id),
            global_state: Kernel::new(&id),
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
        let mut local_elements = self.local_state.elements();
        local_elements.extend(self.global_state.elements());
        local_elements
    }

    /// Gets all the elements from the token
    fn get_token_elements(&self) -> HashSet<E> {
        todo!()
    }

    /// Adds an element to the node.
    pub fn add(&mut self, element: E) {
        self.local_state.add(element, self.sck);
    }

    /// Creates a slot.
    /// TODO: test
    pub fn create_slot(&mut self, other: &Self) {
        // Later this can be optimized. Do not look to cc, but to the set of the local_state.
        if self.tier < other.tier
            && self.local_state.cc.contains_id(&other.id)
            && !self.slots.contains_key(&other.id)
        {
            self.slots.insert(other.id.clone(), (other.sck, self.dck));
            self.dck += 1;
        }
    }

    /// Creates a token in case there is a match slot in the other node.
    /// TODO: Test
    pub fn create_token(&mut self, other: &Self) {
        if let Some(&(sck, dck)) = other.slots.get(&self.id) {
            if sck == self.sck {
                let src_dst = (self.id.clone(), other.id.clone());
                self.tokens.insert(src_dst, ((sck, dck), self.local_state.clone()));
                self.sck += 1;
            }
        }
    }

    /// Set causal context and set associated to self.id to empty.
    fn empty_self(&mut self) {
        self.local_state = Kernel::new(&self.id);    
    }

    pub fn fill_slots(&mut self, other: &Self) {
        todo!()
    }

    fn translate_token_set(&mut self, set: &HashSet<(NodeId, E, i64)>, target_id: &NodeId) {
        todo!()
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
