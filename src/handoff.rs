use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::hash::Hash;

use crate::aworset_opt::AworsetOpt;
use crate::dotcontext::DotContext;
use crate::nodeId::NodeId;

#[derive(Debug)]
pub struct HandoffAworSet<E: Eq + Clone + Hash + Debug + Display> {
    id: NodeId,
    local: AworsetOpt<E>,
    sck: i64,
    dck: i64,
    slots: HashMap<NodeId, (i64, i64)>,
    tokens: HashMap<(NodeId, NodeId), ((i64, i64), DotContext<NodeId>, HashSet<(NodeId, E, i32)>)>,
    val: AworsetOpt<E>,
    tier: i32
}

impl<E: Eq + Clone + Hash + Debug + Display> HandoffAworSet<E> {
    pub fn new(id: NodeId, tier: i32) -> Self {
        Self {
            id: id.clone(),
            local: AworsetOpt::new(id.clone()),
            sck: 0,
            dck: 0,
            slots: HashMap::new(),
            tokens: HashMap::new(),
            val: AworsetOpt::new(id.clone()),
            tier
        }
    }

    /// Returns all the elements known by the node.
    /// It must be the merge between values not sent yet (local) and the values in the lower tiers (val). 
    pub fn fetch(&self) -> HashSet<E>{
        let mut set = self.val.elements();
        set.extend(self.local.elements());
        set
    }

    pub fn add(&mut self, element: &E) {
        self.local.add(element.clone());
    }

    pub fn create_slot(&mut self, other: &Self){
        if self.tier < other.tier && !other.local.cc.is_empty_set() && !self.slots.contains_key(&other.id) {
            self.slots.insert(other.id.clone(), (other.sck, self.dck));
            self.dck += 1;
        }
    }

    

}
