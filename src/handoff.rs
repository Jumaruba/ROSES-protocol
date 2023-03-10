use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::hash::Hash;

use crate::kernel::Kernel;
use crate::nodeId::NodeId;
use crate::types::{Ck, Dot, TagElement};

#[derive(Debug, Clone)]
pub struct Handoff<E: Eq + Clone + Hash + Debug + Display> {
    id: NodeId,
    kernel: Kernel<E>,
    pub ck: Ck,
    pub slots: HashMap<NodeId, Ck>,
    tokens: HashMap<(NodeId, NodeId), (Ck, i64, HashSet<TagElement<E>>)>,
    pub transl: HashSet<(Dot, Dot)>, // (id_src, sck_src_clock, counter_src, id_dst, sck_dst_clock_ counter_dst)  // TODO: create a type for this.
    tier: i32,
}

impl<E: Eq + Clone + Hash + Debug + Display> Handoff<E> {
    pub fn new(id: NodeId, tier: i32) -> Self {
        Self {
            id: id.clone(),
            kernel: Kernel::new(&id),
            ck: Ck { sck: 1, dck: 1 },
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
    pub fn add(&mut self, elem: E) -> TagElement<E> {
        self.kernel.add(elem, self.ck.sck)
    }

    /// Removes an element
    /// TODO: To test
    pub fn rm(&mut self, elem: E) {
        self.rm_token_elem(&elem);
        self.kernel.rm(&elem);
    }

    pub fn merge(&mut self, other: &Self) {
        self.fill_slots(other);     // Adds the new entries. 
        self.discard_slot(other);   
        self.create_slot(other);
        self.merge_vectors(other);
        self.discard_transl(other);
        //self.translate(other);
        self.discard_tokens(other);
        self.create_token(other);

        //self.cache_tokens(other);
    }

    // --------------------------
    // MERGE FUNCTIONS
    // Functions that composes the merge.
    // --------------------------

    pub fn create_slot(&mut self, other: &Self) {
        if self.tier < other.tier && other.has_updates() && !self.slots.contains_key(&other.id) {
            self.slots.insert(other.id.clone(), Ck::new(other.ck.sck, other.ck.dck));
            self.ck.dck += 1;
        }
    }

    /// Creates a token in case there is a match slot in the other node.
    pub fn create_token(&mut self, other: &Self) {
        if other.slots.contains_key(&self.id) && other.slots[&self.id].sck == self.ck.sck {
            let slot_ck = other.slots[&self.id];
            let n = self.kernel.cc.cc.get(&(self.id.clone(), self.ck.sck)).unwrap_or(&0).clone();   
            let set = self.kernel.elems.get(&self.id).unwrap_or(&HashSet::new()).clone();
            self.tokens.insert((self.id.clone(), other.id.clone()), (slot_ck, n, set));
            self.kernel.clean_id(&self.id, self.ck.sck);
            self.ck.sck += 1;
        }
    }

    pub fn merge_vectors(&mut self, other: &Self) {
        // Do not merge entries with other.id as key.
        if self.tier <= other.tier {
            let mut prep_other: Self = other.clone();
            prep_other.kernel.clean_id(&other.id, other.ck.sck);
            self.kernel.join(&prep_other.kernel);
        } else {
            self.kernel.join(&other.kernel);
        }
    }

    /// TODO: to test
    pub fn fill_slots(&mut self, other: &Self) {
        other.tokens.iter().for_each(|((_, dst), (ck, _, elems))| {
            if *dst == self.id {
                if let Some(slot_val) = self.slots.get(&other.id) {
                    if slot_val == ck {
                        self.add_tokens(&other.id, elems);
                        self.slots.remove(&other.id);
                    }
                }
            }
        });
    }

    /// Merges the tokens elements with the actual state.
    /// A correct kernel contains only elements created in the source node.
    fn add_tokens(&mut self, other_id: &NodeId, other_elems: &HashSet<TagElement<E>>) {
        other_elems.iter().for_each(|o_tag_element| {
            let s_tag_element = self.kernel.add(o_tag_element.elem.clone(), self.ck.sck);
            self.transl.insert((
                Dot {
                    id: other_id.clone(),
                    sck: o_tag_element.sck.clone(),
                    n: o_tag_element.n.clone(),
                },
                Dot {
                    id: self.id.clone(),
                    sck: s_tag_element.sck,
                    n: s_tag_element.n,
                },
            ));
        });
    }

    /// Discards a slot that can never be filled, since sck is higher than the one marked in the slot.
    pub fn discard_slot(&mut self, other: &Self) {
        if let Some(&ck) = self.slots.get(&other.id) {
            if other.ck.sck > ck.sck {
                self.slots.remove(&other.id);
            }
        }
    }

    /// Discard tokens that were already used or are out of date.
    /// TODO: to test
    pub fn discard_tokens(&mut self, other: &Self) {
        self.tokens = self
            .tokens
            .drain()
            .filter(|((src, dst), (token_ck, _, _))| {
                !(*dst == other.id
                    && match other.slots.get(&src) {
                        Some(&slot_ck) => slot_ck.dck > token_ck.dck,
                        None => other.ck.dck > token_ck.dck,
                    })
            })
            .collect();
    }

    /// Updates the values in set and cc.
    pub fn aggregate(&mut self, other: &Self) {
        self.kernel.join(&other.kernel);
    }

    /// Applies translatiosn that came from the other node.
    /// TODO: check this. Supposed to get things on token and translate, only.
    pub fn translate(&mut self, other: &Self) {
        todo!()
    }

    /// TODO
    pub fn cache_tokens(&mut self, other: &Self) {
        todo!()
    }

    /// TODO
    /// Translation is discarded when the element was already translated.
    pub fn discard_transl(&mut self, other: &Self) {
        self.transl = self
            .transl
            .drain()
            .filter(|(_, dst_dot)| !other.kernel.cc.dot_in(&dst_dot))
            .collect();
    }

    // --------------------------
    // UTILS FUNCTIONS
    // --------------------------

    /// Gets all the elements from the token
    /// TODO: to test
    fn get_token_elements(&self) -> HashSet<E> {
        let mut res: HashSet<E> = HashSet::new();
        for (_, (_, _, elems)) in self.tokens.iter() {
            elems.iter().for_each(|tag_element| {
                res.insert(tag_element.elem.clone());
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
                .filter(|tag_element| {
                    return tag_element.elem == *elem;
                })
                .collect();
        });
    }

    fn has_updates(&self) -> bool{
        self.kernel.has_seen(&self.id, self.ck.sck)
    }
}

impl<E: Eq + Clone + Hash + Debug + Display> Display for Handoff<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {}, ck: {:?}\ntier: {:?}\nelems: {:?}\ncc: {:?}\ndc: {:?}\nslots: {:?}\ntokens: {:?}\ntransl: {:?}\n",
            self.id, self.ck, self.tier, self.kernel.elems, self.kernel.cc.cc, self.kernel.cc.dc, self.slots, self.tokens, self.transl
        )
    }
}
