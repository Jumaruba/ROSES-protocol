use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::hash::Hash;

use crate::aworset_opt::AworsetOpt;
use crate::dotcontext::DotContext;
use crate::nodeId::NodeId;

#[derive(Debug)]
pub struct HandoffAworSet<E: Eq + Clone + Hash + Debug + Display> {
    id: NodeId,
    set: AworsetOpt<E>,
    ck: i64,
    sck: i64,
    dck: i64,
    slots: HashMap<NodeId, (i64, i64)>,
    tokens: HashMap<(NodeId, NodeId), ((i64, i64), DotContext<NodeId>, HashSet<(NodeId, E, i32)>)>,
    tier: i32
}

impl<E: Eq + Clone + Hash + Debug + Display> HandoffAworSet<E> {
    pub fn new(id: NodeId, tier: i32) -> Self {
        Self {
            id: id.clone(),
            set: AworsetOpt::new(id.clone()),
            ck: 0, 
            sck: 0,
            dck: 0,
            slots: HashMap::new(),
            tokens: HashMap::new(),
            tier
        }
    }

    /// Returns all the elements known by the node.
    /// It must be the merge between values not sent yet (local) and the values in the lower tiers (val). 
    pub fn fetch(&self) -> HashSet<E>{
        todo!()
    }

    pub fn add(&mut self, element: &E) {
    }

    pub fn create_slot(&mut self, other: &Self){
        todo!()
    }

    pub fn create_token(&mut self, other: &Self){
        todo!()
    }

}
