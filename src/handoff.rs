use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::hash::Hash;

use crate::aworset_opt::AworsetOpt;
use crate::dotcontext::DotContext;
use crate::nodeId::{NodeId};

#[derive(Debug)]
pub struct HandoffAworSet<E: Eq + Clone + Hash + Debug + Display> {
    id: NodeId,
    aworset: AworsetOpt<E>,
    sck: i64,
    dck: i64,
    pub slots: HashMap<NodeId, (i64, i64)>,
    tokens: HashMap<(NodeId, NodeId), ((i64, i64), DotContext<NodeId>, HashSet<(NodeId, E, i64)>)>,
    tier: i32
}

impl<E: Eq + Clone + Hash + Debug + Display> HandoffAworSet<E> {
    pub fn new(id: NodeId, tier: i32) -> Self {
        Self {
            id: id.clone(),
            aworset: AworsetOpt::new(id.clone()),
            sck: 0,
            dck: 0,
            slots: HashMap::new(),
            tokens: HashMap::new(),
            tier
        }
    }

    /// Returns all the elements known by the node.
    /// Must be the combination of the elements in the token and in the set. 
    pub fn fetch(&self) -> HashSet<E>{
        let mut set: HashSet<E> = HashSet::new();
        set.extend(self.get_token_elements());
        set.extend(self.aworset.elements());
        set
    }

    /// Gets all the elements from the token
    fn get_token_elements(&self) -> HashSet<E>{
        let mut set: HashSet<E> = HashSet::new();
        for (_, (_, _, entries)) in self.tokens.iter() {
            for (_, element, _) in entries.iter(){
                set.insert(element.clone());
            }
        }
        set
    }

    /// Adds an element to the node.
    pub fn add(&mut self, element: E) {
        self.aworset.add(element);

    }

    pub fn create_slot(&mut self, other: &Self){
        if self.tier < other.tier && other.aworset.cc.get_key_val(&other.id) > 0 && !self.slots.contains_key(&other.id){
            self.slots.insert(other.id.clone(), (other.sck, self.dck));
            self.dck += 1;
        }
    }

    /// Creates a token in case there is a match slot in the other node.
    pub fn create_token(&mut self, other: &Self){
        // There is a slot for this node.
        if let Some(ck) = other.slots.get(&self.id) {
            if ck.0 == self.sck {
                self.tokens.insert((self.id.clone(), other.id.clone()), (*ck, self.aworset.cc.clone(), self.aworset.set.clone()));  // Create token
                self.empty_self();
                self.sck += 1;
            }   
        }
    }

    /// Set causal context and set associated to self.id to empty. 
    /// But the dot translation cloud remains intact.
    fn empty_self(&mut self) {
        self.aworset.cc.set_empty_self(&self.id);   // Empty causal context {A -> 0}.
        self.aworset.set = self.aworset.set.drain().filter(|(nodeid, _, _)| *nodeid != self.id).collect();   // 
    }

    pub fn fill_slots(&mut self, other: &Self){
        for ((_, t_dst), (t_ck, _, t_set)) in other.tokens.iter(){
            if *t_dst == self.id {
                if let Some(&s_ck) = self.slots.get(&other.id) {
                    if s_ck == *t_ck {
                        self.translate_token_set(t_set, &other.id);
                        self.slots.remove(&other.id);   // Fill correspondent slot.
                    }
                }
            }
        }

    }

    fn translate_token_set(&mut self, set: &HashSet<(NodeId, E, i64)>, target_id: &NodeId){
        for triple in set.iter(){
            self.translate_triple(triple.clone(), target_id);
        }
    }

    /// Translates an element that has the target_id. 
    /// # Example
    /// An element that comes as (A, "i", 2), will be translated to (B, "i", n), considering that n is the value of the current node causal context.
    /// cc: {nodeid -> 10}, for instance.
    fn translate_triple(&mut self, (id, element, n): (NodeId, E, i64), target_id: &NodeId){
        if id == *target_id {
            self.aworset.add_dottr(id, element, n);
        }
    }


    /// Discards a slot that can never be filled, since sck is higher than the one marked in the slot.
    pub fn discard_slot(&mut self, other: &Self) {
        if let Some(&(src, _)) = self.slots.get(&other.id) {
            if other.sck > src {
                self.slots.remove(&other.id);
            }
        }
    }

    pub fn discard_tokens(&mut self, other: &Self){
        let token = self.tokens.drain()
            .filter(|((src,dst), ((_, dck), _, _))| {
                !(*dst == other.id && match other.slots.get(&src) {
                    Some(&(_, d)) =>  d > *dck, 
                    None => other.dck > *dck
            })
        }).collect();
        self.tokens = token;
    }

    fn aggregate(){

    }
}
