use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::hash::Hash;

use crate::dotcontext::DotContext;
use crate::types::{Ck, Dot, NodeId, TagElem};

#[derive(Debug, Clone)]
pub struct Handoff<E: Eq + Clone + Hash + Debug + Display> {
    pub id: NodeId,
    pub tier: i32,
    pub ck: Ck,                                   // Clock
    pub cc: DotContext,                           // Causal Context
    pub te: HashMap<NodeId, HashSet<TagElem<E>>>, // Tagged Elements
    pub slots: HashMap<NodeId, Ck>,
    pub tokens: HashMap<(NodeId, NodeId), (Ck, i64, HashSet<TagElem<E>>)>,
    pub transl: HashSet<(Dot, Dot)>,
}

impl<E: Eq + Clone + Hash + Debug + Display> Handoff<E> {
    pub fn new(id: NodeId, tier: i32) -> Self {
        Self {
            id: id.clone(),
            tier,
            ck: Ck { sck: 1, dck: 1 },
            cc: DotContext::new(),
            te: HashMap::new(),
            slots: HashMap::new(),
            tokens: HashMap::new(),
            transl: HashSet::new(),
        }
    }

    pub fn fetch(&self) -> HashSet<E> {
        let mut kernel_elems = self.get_ti_elems();
        kernel_elems.extend(self.get_token_elems());
        kernel_elems
    }

    fn get_ti_elems(&self) -> HashSet<E> {
        let mut res: HashSet<E> = HashSet::new();
        for (_, set) in self.te.iter() {
            set.iter().for_each(|e| {
                res.insert(e.elem.clone());
            })
        }
        res
    }

    fn get_token_elems(&self) -> HashSet<E> {
        let mut res: HashSet<E> = HashSet::new();
        for (_, (_, _, elems)) in self.tokens.iter() {
            elems.iter().for_each(|tag_element| {
                res.insert(tag_element.elem.clone());
            });
        }
        res
    }

    /// Adds an element to the node.
    pub fn add_elem(&mut self, elem: E) -> TagElem<E> {
        let dot = self.cc.makedot(&self.id, self.ck.sck);
        let tag_elem = TagElem::new(dot.sck, dot.n, elem);
        self.te
            .entry(dot.id)
            .and_modify(|set| {
                set.insert(tag_elem.clone());
            })
            .or_insert(HashSet::from([tag_elem.clone()]));
        tag_elem
    }

    /// Removes an element
    pub fn rm_elem(&mut self, elem: E) {
        self.rm_te_elem(&elem);
        self.rm_token_elem(&elem);
    }

    fn rm_te_elem(&mut self, elem: &E) {
        self.te = self
            .te
            .drain()
            .map(|(id, mut set)| {
                set = set
                    .drain()
                    .filter(|tag_elem| *elem != tag_elem.elem)
                    .collect();
                (id, set)
            })
            .filter(|(_, set)| !set.is_empty())
            .collect();
    }

    fn rm_token_elem(&mut self, elem: &E) {
        self.tokens.iter_mut().for_each(|(_, (_, _, set))| {
            *set = set
                .drain()
                .filter(|tag_elem| {
                    return tag_elem.elem != *elem;
                })
                .collect();
        });
    }

    pub fn merge(&mut self, other: &Self) {
        self.fill_slots(other); // Adds the new entries.
        self.discard_slot(other);
        self.create_slot(other);
        self.discard_transl(other);
        self.translate(other);
        self.merge_vectors(other);
        self.discard_tokens(other);
        self.create_token(other);
        self.cache_tokens(other);
    }

    pub fn create_slot(&mut self, other: &Self) {
        if self.tier < other.tier && other.has_updates() && !self.slots.contains_key(&other.id) {
            self.slots
                .insert(other.id.clone(), Ck::new(other.ck.sck, self.ck.dck));
            self.ck.dck += 1;
        }
    }

    /// Creates a token in case there is a match slot in the other node.
    pub fn create_token(&mut self, other: &Self) {
        if other.slots.contains_key(&self.id) && other.slots[&self.id].sck == self.ck.sck {
            let ck = other.slots[&self.id];
            let n = self.cc.get_cc(&self.id, self.ck.sck);
            let set = self.te.get(&self.id).unwrap_or(&HashSet::new()).clone();
            self.tokens
                .insert((self.id.clone(), other.id.clone()), (ck, n, set));
            self.clear_local();
            self.ck.sck += 1;
        }
    }

    pub fn merge_vectors(&mut self, other: &Self) {
        // Do not merge entries with other.id as key.
        if self.tier <= other.tier {
            let mut prep_other: Self = other.clone();
            prep_other.clear_local();
            self.join(&prep_other);
        } else {
            self.join(&other);
        }
    }

    pub fn join(&mut self, other: &Self) {
        // Intersection and elements not known by other.
        self.te = self
            .te
            .drain()
            .map(|(id, mut set)| {
                let new_set: HashSet<TagElem<E>> = set
                    .drain()
                    .filter(|tag| {
                        (other.te.contains_key(&id) && other.te[&id].contains(tag)) || !other.cc.dot_in(&tag.to_dot(&id))
                    })
                    .collect();
                (id, new_set)
            })
            .filter(|(_, hash)| !hash.is_empty())
            .collect();


        // Elements known by other but not by self
        for (id, hash) in other.te.iter() {
            for tag in hash.iter() {
                let dot = tag.to_dot(&id);
                if !self.cc.dot_in(&dot) {
                    self.te
                        .entry(id.clone())
                        .and_modify(|val| {
                            val.insert(tag.clone());
                        })
                        .or_insert(HashSet::from([tag.clone()]));
                }
            }
        }
        self.cc.join(&other.cc);
    }

    pub fn fill_slots(&mut self, other: &Self) {
        for ((src, dst), (ck, n, elems)) in other.tokens.iter() {
            if *dst == self.id
                && self.slots.contains_key(&other.id)
                && self.slots[&other.id].sck == ck.sck
            {
                self.insert_dot_elems(elems);
                let curr_n = self.cc.get_cc(&self.id, self.ck.sck);
                let target_dot = Dot::new(self.id.clone(), self.ck.sck, *n + curr_n);
                self.cc.insert_cc(&target_dot);
                let source_dot = Dot::new(src.clone(), ck.sck, *n);
                self.transl.insert((source_dot, target_dot)); // Creates translation.
                self.slots.remove(&other.id);
            }
        }
    }

    fn insert_dot_elems(&mut self, elems: &HashSet<TagElem<E>>) {
        let curr_n = self.cc.get_cc(&self.id, self.ck.sck);
        elems.iter().for_each(|source_tag_e| {
            let target_tag_e = TagElem::new(
                self.ck.sck,
                source_tag_e.n + curr_n,
                source_tag_e.elem.clone(),
            );
            self.te
                .entry(self.id.clone())
                .and_modify(|set| {
                    set.insert(target_tag_e.clone());
                })
                .or_insert(HashSet::from([target_tag_e]));
        });
    }

    pub fn discard_slot(&mut self, other: &Self) {
        if let Some(&ck) = self.slots.get(&other.id) {
            if other.ck.sck > ck.sck {
                self.slots.remove(&other.id);
            }
        }
    }

    /// Discard tokens that were already used or are out of date.
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
        if other.tier >= self.tier {
            return;
        }
        let mut res: Handoff<E> = Handoff::new(other.id.clone(), other.tier);
        // translate tokens
        for (src_t, trg_t) in other.transl.iter() {
            if let Some(t) = self.tokens.get(&(src_t.id.clone(), trg_t.id.clone())) {
                // Match translation and token
                if src_t.sck == t.0.sck {
                    let range = (trg_t.n-src_t.n+1)..(trg_t.n+1);
                    range.for_each(|n| {res.cc.dc.insert(Dot::new(trg_t.id.clone(), trg_t.sck, n));});
                    //res.cc.insert_cc(trg_t);
                    t.2.iter().for_each(|tag| {
                        let n = (trg_t.n - src_t.n) + tag.n;
                        let tag = TagElem::new(trg_t.sck, n, tag.elem.clone());
                        res.te
                            .entry(trg_t.id.clone())
                            .and_modify(|set| {
                                set.insert(tag.clone());
                            })
                            .or_insert(HashSet::from([(tag.clone())]));
                    });
                }
            }
        }
        self.join(&res);
    }

    pub fn cache_tokens(&mut self, other: &Self) {
        if self.tier < other.tier {
            for ((src, dst), v) in other.tokens.iter() {
                if *src == other.id && *dst != self.id {
                    let keys = &(src.clone(), dst.clone());
                    let val = self.tokens.entry(keys.clone()).or_insert(v.clone()); 
                    if val.0.sck <= v.0.sck {
                        *val = v.clone();
                    } 
                }
            }
        }
    }

    /// Translation is discarded when the element was already translated.
    pub fn discard_transl(&mut self, other: &Self) {
        self.transl = self
            .transl
            .drain()
            .filter(|(_, dst_dot)| !other.cc.dot_in(&dst_dot))
            .collect();
    }

    fn has_updates(&self) -> bool {
        self.cc.id_in(&self.id, self.ck.sck)
    }

    fn clear_local(&mut self) {
        self.te.remove(&self.id);
        self.cc.clean_id(&self.id, self.ck.sck);
    }
}

impl<E: Eq + Clone + Hash + Debug + Display> Display for Handoff<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {:?}\ntier: {:?}\nelems: {:?}\ncc: {:?}\ndc: {:?}\nslots: {:?}\ntokens: {:?}\ntransl: {:?}\n",
            self.id, self.ck, self.tier, self.te, self.cc.cc, self.cc.dc, self.slots, self.tokens, self.transl
        )
    }
}
