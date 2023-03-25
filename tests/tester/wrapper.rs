use std::collections::{HashMap, HashSet};

use crdt_sample::{AworsetOpt, NodeId};

use super::Op;

#[derive(Clone, Debug)]
pub struct Wrapper {
    pub local: AworsetOpt<i32>,
    pub dispatch: HashMap<String, AworsetOpt<i32>>,
    pub cache: HashMap<String, AworsetOpt<i32>>,
    pub tier: i64,
    pub waiting: HashSet<String>,   // Is expecting a message from id.
    pub can_dispatch: HashMap<String, bool>, 
    pub id: String,
    pub blocked: bool,              // Can propagate if it is false. 
}

impl Wrapper {
    pub fn new(id: NodeId, tier: i64) -> Self {
        Self {
            local: AworsetOpt::new(id.clone()),
            dispatch: HashMap::new(),
            cache: HashMap::new(),
            tier,
            waiting: HashSet::new(),
            can_dispatch: HashMap::new(), 
            id: format!("{}", id),
        }
    }

    pub fn apply_oper(&mut self, oper: Op<i32>) {
        match oper {
            Op::RM(elem) => {
                self.local.rm(elem);
                self.dispatch.iter_mut().for_each(|(_,aw)| {
                    aw.rm(elem);
                });
            }
            Op::ADD(elem) => {
                self.local.add(elem);
            }
        }
    }

    /// If there is a dispatch, then deliver.
    /// Otherwise, prepare a dispatch.
    /// Returns None case it creates a dispatch, an aworset otherwise.
    /// The later indicates what is supposed to be joined with the receiver.
    pub fn propagate(&mut self, other: &Wrapper, tier: i32) -> Option<AworsetOpt<i32>> {
        // Needs to be higher tier and if there is not a dispatch then create one.
        if self.tier > i64::from(tier) && !self.dispatch.contains_key(&other.id) && other.waiting.get(&self.id).is_some() {
            self.dispatch.insert(other.id.clone(), self.local.clone());
            self.can_dispatch.insert(other.id.clone(), false);    
            return None;
        }
        // Return the aworset that the receiver should join.
        else {
            return self.get_deliverable(other.id.clone());
        }
    }

    /// Sends the element to be delivered to another element.
    /// Or the content is at cache or dispache. If they are in both, then merge.
    fn get_deliverable(&mut self, id: String) -> Option<AworsetOpt<i32>> {
        let mut res: AworsetOpt<i32>;
        if self.dispatch.get(&id).is_some() && self.can_dispatch[&id] {
            res = self.dispatch[&id].clone();
            self.dispatch.remove(&id);
            self.can_dispatch.remove(&id);
            if let Some(cache) = self.cache.get(&id) {
                res.join(cache);
                self.cache.remove(&id);
            }
            return Some(res);
        } else if let Some(cache) = self.cache.get(&id) {
            res = cache.clone();
            self.cache.remove(&id);
            return Some(res); 
        }
        return None;
    }

    /// Receive a state from another node.
    pub fn join(&mut self, other: &Wrapper, id: String) {
        if deliver.is_some() {
            self.local.join(&deliver.clone().unwrap());
        }
        // If there is a deliver, now it can be delivered. 
        self.can_dispatch.entry(id).and_modify(|v| *v = true); 
    }


    /// BASIC OPERATIONS ============================= 
    pub fn fetch(&self) -> HashSet<i32> {
        self.local.elements()
    }

    pub fn add(&mut self, elem: i32) {
        self.blocked = false;
        self.local.add(elem);
    }

    pub fn rm(&mut self, elem: i32) {
        self.local.rm(elem);
    }
}
