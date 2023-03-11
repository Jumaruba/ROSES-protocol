use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::hash::Hash;

use crate::kernel::Kernel;
use crate::nodeId::NodeId;
use crate::types::{Ck, Dot, TagItem};

#[derive(Debug, Clone)]
pub struct Handoff<E: Eq + Clone + Hash + Debug + Display> {
    id: NodeId,
    kernel: Kernel<E>,
    pub ck: Ck,
    pub slots: HashMap<NodeId, Ck>,
    tokens: HashMap<(NodeId, NodeId), (Ck, i64, HashSet<TagItem<E>>)>,
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

    pub fn fetch(&self) -> HashSet<E> {
        let mut kernel_elems = self.kernel.elements();
        kernel_elems.extend(self.get_token_elements());
        kernel_elems
    }

    /// Adds an element to the node.
    pub fn add(&mut self, elem: E) -> TagItem<E> {
        self.kernel.add(elem, self.ck.sck)
    }

    /// Removes an element
    pub fn rm(&mut self, elem: E) {
        self.rm_token_elem(&elem);
        self.kernel.rm(&elem);
    }

    pub fn merge(&mut self, other: &Self) {
        self.fill_slots(other); // Adds the new entries.
        self.discard_slot(other);
        self.create_slot(other);
        self.merge_vectors(other);
        self.discard_transl(other);
        self.translate(other);
        self.discard_tokens(other);
        self.create_token(other);

        //self.cache_tokens(other);
    }

    pub fn create_slot(&mut self, other: &Self) {
        if self.tier < other.tier && other.has_updates() && !self.slots.contains_key(&other.id) {
            self.slots
                .insert(other.id.clone(), Ck::new(other.ck.sck, other.ck.dck));
            self.ck.dck += 1;
        }
    }

    /// Creates a token in case there is a match slot in the other node.
    pub fn create_token(&mut self, other: &Self) {
        if other.slots.contains_key(&self.id) && other.slots[&self.id].sck == self.ck.sck {
            let slot_ck = other.slots[&self.id];
            let n = self.kernel.cc.get_cc(&self.id, self.ck.sck);
            let set = self.kernel.get_ti(&self.id);
            self.tokens
                .insert((self.id.clone(), other.id.clone()), (slot_ck, n, set));
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
        for ((_, dst), (ck, n, elems)) in other.tokens.iter() {
            if *dst == self.id
                && self.slots.contains_key(&other.id)
                && self.slots[&other.id].sck == ck.sck
            {
                self.insert_dot_elem(elems);
                let target_dot = self.kernel.cc.inc_cc(&self.id, self.ck.sck, *n);
                let source_dot = Dot::new(self.id.clone(), ck.sck, *n);
                self.transl.insert((source_dot, target_dot));
                self.slots.remove(&other.id);
            }
        }
    }


    fn insert_dot_elem(&mut self, elems: &HashSet<TagItem<E>>) {
        elems.iter().for_each(|tag_e| {
            self.kernel
                .insert_dot_elem(tag_e.to_dot(&self.id), &tag_e.elem)
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

    /// Applies translatiosn that came from the other node.
    /// TODO: check this. Supposed to get things on token and translate, only.
    pub fn translate(&mut self, other: &Self) {
        let mut res: Handoff<E> = Handoff::new(other.id.clone(), other.tier);
        // translate tokens
        for (orig, transl) in other.transl.iter() {
            for ((src, dst), (ck, n, elems)) in self.tokens.iter() {
                if orig.id == *src && ck.sck == orig.sck && orig.n == *n {
                    res.kernel.add_cc(transl);
                }
                elems.iter().for_each(|tag_element| {
                    res.kernel.ti.entry(dst.clone()).and_modify(|hash| {
                        hash.insert(TagItem::new(
                            transl.sck,
                            tag_element.n,
                            tag_element.elem.clone(),
                        ));
                    });
                });
            }
        }
        // create kernel from tokens
        // merge new kernel with self
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

    /// Gets all the elements from the token
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

    fn has_updates(&self) -> bool {
        self.kernel.has_seen(&self.id, self.ck.sck)
    }
}

impl<E: Eq + Clone + Hash + Debug + Display> Display for Handoff<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {}, ck: {:?}\ntier: {:?}\nelems: {:?}\ncc: {:?}\ndc: {:?}\nslots: {:?}\ntokens: {:?}\ntransl: {:?}\n",
            self.id, self.ck, self.tier, self.kernel.ti, self.kernel.cc.cc, self.kernel.cc.dc, self.slots, self.tokens, self.transl
        )
    }
}
