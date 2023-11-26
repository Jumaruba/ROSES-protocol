use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::mem::size_of;

use crate::dotcontext::DotContext;
use crate::types::{Ck, Dot, NodeId, Payload, TDot};

#[derive(Debug, Clone)]
pub struct Handoff<E: Eq + Clone + Hash + Debug + Display> {
    pub id: NodeId,
    pub tier: i32,
    pub ck: Ck,                                         // Clock
    pub cc: DotContext,                                 // Causal Context
    pub payload: HashMap<NodeId, HashSet<Payload<E>>>,  // Tagged Elements
    pub slots: HashMap<NodeId, Ck>,
    pub tokens: HashMap<(NodeId, NodeId), (Ck, (i64, i64), HashSet<Payload<E>>)>,
    pub transl: HashMap<(NodeId, NodeId), (Ck, (i64, i64))>,
    pub last_send_n: i64
}

impl<E: Eq + Clone + Hash + Debug + Display> Handoff<E> {
    pub fn new(id: NodeId, tier: i32) -> Self {
        Self {
            id: id.clone(),
            tier,
            ck: Ck { sck: 1, dck: 1 },
            cc: DotContext::new(),
            payload: HashMap::new(),
            slots: HashMap::new(),
            tokens: HashMap::new(),
            transl: HashMap::new(),
            last_send_n: 0
        }
    }

    /// Gets the number of bytes in a Handoff.
    /// TODO: check this again in the end.
    pub fn get_num_bytes(&self) -> usize {
        let mut total_size: usize = 0;
        total_size += self.id.get_num_bytes();  // id 
        total_size += size_of::<i32>();         // tier 
        total_size += size_of::<i64>() * 2;     // sck, dck
        total_size += self.cc.get_num_bytes();  // cc 

        // Slots 
        for (key, _) in self.slots.iter(){
            total_size += key.get_num_bytes();  // key
            total_size += size_of::<i64>() * 2; // value
        }

        // Translation size 
        for ((nodeId1, nodeId2), _) in self.transl.iter(){
            total_size += nodeId1.get_num_bytes(); 
            total_size += nodeId2.get_num_bytes(); 
            total_size += size_of::<i64>() * 4; 
        } 

        // Tokens
        for (key, value) in self.tokens.iter(){
            total_size += key.0.get_num_bytes(); 
            total_size += key.1.get_num_bytes();  
            
            total_size += size_of::<i64>() * 3; // Ck, ni, nf

            for tag in value.2.iter(){
                total_size += tag.get_num_bytes();
            }
        }

        total_size 
    }

    pub fn fetch(&self) -> HashSet<E> {
       let mut res: HashSet<E> = HashSet::new();
        for (_, set) in self.payload.iter() {
            set.iter().for_each(|e| {
                res.insert(e.elem.clone());
            })
        }
        res
    }

    /// Adds an element to the node.
    pub fn add_elem(&mut self, elem: E) -> Payload<E> {
        let dot = self.cc.makedot(&self.id, self.ck.sck);
        let p = Payload::new(dot.n, elem);
        self.payload
            .entry(dot.id)
            .and_modify(|set| {
                set.insert(p.clone());
            })
            .or_insert(HashSet::from([p.clone()]));
        p
    }

    /// Removes an element
    pub fn rm_elem(&mut self, elem: E) {
        /// Remove from the payload
        self.remove_payload_element(&elem);
        /// Remove from the token
        self.remove_token_element(&elem);
    }

    /// Remove element from the payload.
    fn remove_payload_element(&mut self, elem: &E) {
        self.payload = self
            .payload
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

    /// Remove an element from the token.
    fn remove_token_element(&mut self, elem: &E) {
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
        self.fill_slots(other);
        self.discard_slot(other);
        self.create_slot(other);
        self.discard_transl(other);
        self.translate(other);
        self.cache_transl(other);
        self.merge_vectors(other);
        self.discard_tokens(other);
        self.create_token(other);
        self.cache_tokens(other);
    }

    /// This function deletes the slot and creates a translation.
    pub fn fill_slots(&mut self, other: &Self) {
        // Iterate throw each token to check if it can fill a slot.
        for ((src, dst), (ck, (n_initial, n_final), elems)) in other.tokens.iter() {
            // The token is the destine of this node.
            if *dst == self.id {
                if let Some(slot_ck) = self.slots.get(&src) {
                    // The clocks of the token and slot matches.
                    if slot_ck == ck {
                        // Translate and add elements to our internal state.
                        self.insert_elems(elems, *n_initial);
                        let curr_n = self.cc.get_cc(&self.id);
                        let range = n_final - n_initial;
                        let final_dot = Dot::new(self.id.clone(), curr_n + range);
                        self.cc.insert_cc(&final_dot);
                        // Creates translation.
                        self.transl.insert((src.clone(), dst.clone()), (ck.clone(), (n_final-1, curr_n + range)));
                        self.slots.remove(&src);
                    }
                }
            }
        }
    }

    /// Deletes a slot if it's not necessary anymore.
    pub fn discard_slot(&mut self, other: &Self) {
        if let Some(&ck) = self.slots.get(&other.id) {
            if other.ck.sck > ck.sck {
                self.slots.remove(&other.id);
            }
        }
    }

    /// Creates a slot.
    pub fn create_slot(&mut self, other: &Self) {
        if self.tier < other.tier && other.has_updates() && !self.slots.contains_key(&other.id) {
            self.slots
                .insert(other.id.clone(), Ck::new(other.ck.sck, self.ck.dck));
            self.ck.dck += 1;
        }
    }

    /// Translation is discarded when the element was already incorporated by the source node.
    /// This operations is just applied to servers.
    pub fn discard_transl(&mut self, other: &Self) {
        // Must be a server.
        if self.tier < other.tier {
            self.transl = self
                .transl
                .drain()
                .filter(|((src_node_id, dst_node_id), (ck, (n_final_src, n_final_dest)))| {
                    // The destination of the translation must be the source node.
                    if other.id == *src_node_id {
                        let dot = Dot::new( dst_node_id.clone(), *n_final_dest);
                        // The element was incorporated.
                        return !other.cc.dot_in(&dot);
                    }
                    return true;
                })
                .collect();
        }
    }


    /// Applies translation that came from the other node.
    pub fn translate(&mut self, other: &Self) {
        // Must be a client to apply a translation.
        if other.tier >= self.tier {
            return;
        }

        let mut res: Handoff<E> = Handoff::new(other.id.clone(), other.tier);
        // translate tokens
        for ((src, dst), ((ck), (final_n_src, final_n_dst))) in other.transl.iter() {
            // Get the token this translation targets.
            if let Some(token) = self.tokens.get(&(src.clone(), dst.clone())) {
                let range = token.1.1 - token.1.0;
                // Match translation and token
                if ck.sck == token.0.sck {
                    // Add causal contexts to the answer.
                    let dots = (final_n_dst - range + 1)..*final_n_dst+1 ;
                    dots.for_each(|n| {
                        res.cc.dc.insert(Dot::new(dst.clone(), n));
                    });
                    // Add the causal context of the non-translated elements. So that they can be deleted
                    // from the original array when merging.
                    let dot = Dot:: new(src.clone(), *final_n_src);
                    res.cc.insert_cc(&dot);

                    // Add the payloads.
                    token.2.iter().for_each(|payload| {
                        /// final_n_dst - range = beginning of dst
                        /// range - (final_n_src - payload.n) = range of n from the beginning.
                        let n = (final_n_dst - range) + (range - (final_n_src - payload.n));
                        let translated_payload = Payload::new(n, payload.elem.clone());
                        res.payload
                            .entry(dst.clone())
                            .and_modify(|set| {
                                set.insert(translated_payload.clone());
                            })
                            .or_insert(HashSet::from([(translated_payload.clone())]));
                    });
                }
            }
        }
        self.join(&res);
    }


    pub fn cache_transl(&mut self, other: &Self) {
        if self.tier == other.tier {

            let mut transl_1: HashMap<_,_> = HashMap::new();
            for (&(ref src, ref dst), &(ck, (final_src, final_dst))) in other.transl.iter() {
                let dot = Dot::new(dst.clone(), final_dst);

                // Get the translation
                let tuple = (Ck::new(0,0), (0,0));
                let translation = self.transl.get(&(src.clone(),dst.clone())).unwrap_or(&tuple);
                let has_translation = translation.0 == ck.clone() && translation.1 == (final_src, final_dst);

                // Translations not known by self.
                if !(self.cc.dot_in(&dot) && !has_translation){
                    transl_1.insert((src.clone(),dst.clone()), (ck.clone(), (final_src, final_dst)));
                }
            }


            let mut transl_2: HashMap<_,_> = self.transl.clone();

            for (&(ref src, ref dst), &(ck, (final_src, final_dst))) in transl_2.iter() {
                let dot = Dot::new(dst.clone(), final_dst);

                // Get the translation
                let tuple = (Ck::new(0,0), (0,0));
                let translation = self.transl.get(&(src.clone(),dst.clone())).unwrap_or(&tuple);
                let has_translation = translation.0 == ck.clone() && translation.1 == (final_src, final_dst);

                // Translations not known by self.
                if other.cc.dot_in(&dot) && !has_translation{
                    self.transl.remove(&(src.clone(), dst.clone()));
                }
            }

            self.transl.extend(transl_1);
        }
    }



    pub fn merge_vectors(&mut self, other: &Self) {
        // Do not merge entries with other.id as key.
        if !(self.tier == 0 && other.tier == 0) && self.tier <= other.tier {
            let mut prep_other: Self = other.clone();
            prep_other.clear_local();
            self.join(&prep_other);
        } else {
            self.join(&other);
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

    /// Creates a token in case there is a match slot in the other node.
    pub fn create_token(&mut self, other: &Self) {
        if other.slots.contains_key(&self.id) && other.slots[&self.id].sck == self.ck.sck {
            let ck = other.slots[&self.id];
            let curr_n = self.cc.get_cc(&self.id);
            // Get the elements that haven't been added to a token yet.
            // These are the ones with n > last_send_n
            let set = self.payload
                .get(&self.id)
                .unwrap_or(&HashSet::new())
                .iter()
                .filter(|&v| v.n > self.last_send_n)
                .cloned()
                .collect();
            self.tokens
                .insert((self.id.clone(), other.id.clone()), (ck, (self.last_send_n + 1, curr_n + 1), set));
            self.ck.sck += 1;
            self.last_send_n = curr_n;
        }
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


    pub fn join(&mut self, other: &Self) {
        // Intersection and elements not known by other.
        self.payload = self
            .payload
            .drain()
            .map(|(id, mut set)| {
                let new_set: HashSet<Payload<E>> = set
                    .drain()
                    .filter(|tag| {
                        (other.payload.contains_key(&id) && other.payload[&id].contains(tag))
                            || !other.cc.dot_in(&tag.to_dot(&id))
                    })
                    .collect();
                (id, new_set)
            })
            .filter(|(_, hash)| !hash.is_empty())
            .collect();

        // Elements known by other but not by self
        for (id, hash) in other.payload.iter() {
            for tag in hash.iter() {
                let dot = tag.to_dot(&id);
                if !self.cc.dot_in(&dot) {
                    self.payload
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


    /// Adds the elements to the payload.
    /// TODO: change the name of this function.
    fn insert_elems(&mut self, elems: &HashSet<Payload<E>>, start_n: i64) {
        let curr_n = self.cc.get_cc(&self.id);
        elems.iter().for_each(|source_tag_e| {
            // Create the tag element
            let tag_elem = Payload::new(
                source_tag_e.n - start_n + curr_n + 1,
                source_tag_e.elem.clone(),
            );
            self.payload
                .entry(self.id.clone())
                .and_modify(|set| {
                    set.insert(tag_elem.clone());
                })
                .or_insert(HashSet::from([tag_elem]));
        });
    }


    /// Check if the node has updates to send.
    fn has_updates(&self) -> bool {
        let dot = Dot::new(self.id.clone(), self.last_send_n + 1);
        return self.cc.dot_in(&dot);
    }

    fn clear_local(&mut self) {
        self.payload.remove(&self.id);
        self.cc.clean_id(&self.id, self.ck.sck);
    }
}

impl<E: Eq + Clone + Hash + Debug + Display> Display for Handoff<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = format!("{}, {:?}\ntier: {:?}\n", self.id, self.ck, self.tier);
        if !self.payload.is_empty() {
            s.push_str(format!("elems: {:?}\n", self.payload).as_str());
        }
        if !self.cc.cc.is_empty() {
            s.push_str(format!("cc: {:?}\n", self.cc.cc).as_str());
        }
        if !self.cc.dc.is_empty() {
            s.push_str(format!("dc: {:?}\n", self.cc.dc).as_str());
        }
        if !self.slots.is_empty() {
            s.push_str(format!("slots: {:?}\n", self.slots).as_str());
        }
        if !self.tokens.is_empty() {
            s.push_str(format!("tokens: {:?}\n", self.tokens).as_str());
        }
        if !self.transl.is_empty() {
            s.push_str(format!("transl: {:?}\n", self.transl).as_str());
        }

        s.push_str(format!("last_send_n: {}\n", self.last_send_n).as_str());

        write!(f, "{}", s)
    }
}
