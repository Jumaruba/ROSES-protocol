use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::hash::Hash;

use crate::kernel::Kernel;
use crate::nodeId::NodeId;

#[derive(Debug)]
pub struct Handoff<E: Eq + Clone + Hash + Debug + Display> {
    id: NodeId,
    kernel: Kernel<E>, 
    sck: i64,          
    pub dck: i64,      
    pub slots: HashMap<NodeId, (i64, i64)>, // Slots {id -> (sck, dck)}
    tokens: HashMap<(NodeId, NodeId), ((i64, i64), i64, HashSet<(i64, i64, E)>)>, // (sck, dck, tag, (sck, tag, E))
    pub transl: HashSet<(NodeId, i64, i64, NodeId, i64, i64)>, // (id_src, sck_src_clock, counter_src, id_dst, sck_dst_clock_ counter_dst)
    tier: i32,
}

impl<E: Eq + Clone + Hash + Debug + Display> Handoff<E> {
    pub fn new(id: NodeId, tier: i32) -> Self {
        Self {
            id: id.clone(),
            kernel: Kernel::new(&id),
            sck: 1,
            dck: 1,
            slots: HashMap::new(),
            tokens: HashMap::new(),
            transl: HashSet::new(),
            tier,
        }
    }

    // --------------------------
    // OPERATIONS
    // --------------------------

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
    pub fn add(&mut self, element: E) -> (i64, i64, E) {
        self.kernel.add(element, self.sck)
    }

    // --------------------------
    // MERGE FUNCTIONS 
    // --------------------------


    /// Creates a slot.
    /// # Example
    /// ```
    /// use std::collections::HashMap;
    /// use thesis_code::{handoff::Handoff, nodeId::NodeId};
    /// // Given
    /// let id_a = NodeId::new(1, "A".to_string());
    /// let id_b = NodeId::new(1, "B".to_string());
    /// let mut h_1: Handoff<i32> = Handoff::new(id_a.clone(), 1);
    /// h_1.add(2);
    /// let mut h_2: Handoff<i32> = Handoff::new(id_b, 0);
    /// // When
    /// h_2.create_slot(&h_1);
    /// // Then
    /// assert_eq!(h_2.dck, 2);
    /// assert_eq!(h_2.slots, HashMap::from([(id_a, (1,1))]));
    /// ```
    pub fn create_slot(&mut self, other: &Self) {
        // can be optimized to check only the other.kernel.set.
        if self.tier < other.tier
            && other.kernel.cc.cc.contains_key(&other.id)
            && !self.slots.contains_key(&other.id)
        {
            self.slots.insert(other.id.clone(), (other.sck, self.dck));
            self.dck += 1;
        }
    }

    /// Creates a token in case there is a match slot in the other node.
    /// To test
    pub fn create_token(&mut self, other: &Self) {
        todo!();
        /*
        if other.slots.contains_key(&self.id) && other.slots[&self.id].0 == self.sck {
            let slot_ck = other.slots[&self.id];
            let self_n = self.kernel.cc.get_n(&self.id, &self.sck);

            let set = self
                .kernel
                .elems
                .get(&self.id)
                .unwrap_or(&HashSet::new())
                .clone();

            self.tokens
                .insert((self.id.clone(), other.id.clone()), (slot_ck, self_n, set));
            self.kernel.remove_id(&self.id);
            self.sck += 1;
        }*/
    }

    pub fn fill_slots(&mut self, other: &Self) {
        todo!()
    }

    /// Merges the tokens elements with the actual state.
    /// A correct kernel contains only elements created in the source node.
    fn add_tokens(&mut self, other: &Self) {
        todo!()
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
        todo!()
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
