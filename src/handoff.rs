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
    tokens: HashMap<
        (NodeId, NodeId),
        (
            (i64, i64),
            HashSet<(NodeId, i64, i64)>,
            HashSet<(i64, i64, E)>,
        ),
    >, // (sck, dck, tag, (sck, tag, E))
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
    // Core operations of the Handoff
    // --------------------------

    /// Returns all the elements known by the node.
    /// Must be the combination of the elements in the token and set.
    /// TODO: to test
    pub fn fetch(&self) -> HashSet<E> {
        let mut kernel_elems = self.kernel.elements();
        kernel_elems.extend(self.get_token_elements());
        kernel_elems
    }

    /// Adds an element to the node.
    /// TODO: to test
    pub fn add(&mut self, element: E) -> (i64, i64, E) {
        self.kernel.add(element, self.sck)
    }

    /// Removes an element
    /// TODO: To test
    pub fn rm(&mut self, elem: E) {
        self.rm_token_elem(&elem);
        self.kernel.rm(&elem);
    }

    pub fn merge(&mut self) {
        todo!()
    }

    // --------------------------
    // MERGE FUNCTIONS
    // Functions that composes the merge.
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
            && other.kernel.has_seen(&other.id)
            && !self.slots.contains_key(&other.id)
        {
            self.slots.insert(other.id.clone(), (other.sck, self.dck));
            self.dck += 1;
        }
    }

    /// Creates a token in case there is a match slot in the other node.
    /// TODO: to test
    pub fn create_token(&mut self, other: &Self) {
        if other.slots.contains_key(&self.id) && other.slots[&self.id].0 == self.sck {
            let slot_ck = other.slots[&self.id];
            let cc = self.kernel.get_cc();

            let set = self
                .kernel
                .elems
                .get(&self.id)
                .unwrap_or(&HashSet::new())
                .clone();

            self.tokens
                .insert((self.id.clone(), other.id.clone()), (slot_ck, cc, set));
            self.kernel.clean_id(&self.id);
            self.sck += 1;
        }
    }

    pub fn fill_slots(&mut self, other: &mut Self) {
        other.tokens.iter_mut().for_each(|((_, dst), (ck, _, elems))| {
            if *dst == self.id {
                if let Some(slot_val) = self.slots.get(&other.id) {
                    if slot_val == ck {
                        self.add_tokens(&other.id, elems);
                        self.kernel.join(&mut other.kernel);
                        self.slots.remove(&other.id);
                    }
                }
            }
        });
    }

    /// Merges the tokens elements with the actual state.
    /// A correct kernel contains only elements created in the source node.
    fn add_tokens(&mut self, other_id: &NodeId, other_elems: &HashSet<(i64, i64, E)>) {
        other_elems
            .iter()
            .for_each(|(other_sck, other_n, other_elem)| {
                let (self_sck, self_n, _) = self.kernel.add(other_elem.clone(), self.sck);
                self.transl.insert((
                    other_id.clone(),
                    other_sck.clone(),
                    other_n.clone(),
                    self.id.clone(),
                    self_sck,
                    self_n,
                ));
            });
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

    // --------------------------
    // UTILS FUNCTIONS
    // --------------------------

    /// Gets all the elements from the token
    /// TODO: to test
    fn get_token_elements(&self) -> HashSet<E> {
        let mut res: HashSet<E> = HashSet::new();
        for (_, (_, _, elems)) in self.tokens.iter() {
            elems.iter().for_each(|(_, _, e)| {
                res.insert(e.clone());
            });
        }
        res
    }

    /// Removes an element from the token.
    /// TODO: To test
    fn rm_token_elem(&mut self, elem: &E) {
        self.tokens.iter_mut().for_each(|(_, (_, _, set))| {
            *set = set
                .drain()
                .filter(|(_, _, s_elem)| {
                    return s_elem == elem;
                })
                .collect();
        });
    }
}
