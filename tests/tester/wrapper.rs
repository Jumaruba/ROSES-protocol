use std::collections::{HashSet, HashMap};

use crdt_sample::{AworsetOpt, NodeId};

use super::Op;

#[derive(Clone, Debug)]
pub struct Wrapper {
    pub local: AworsetOpt<i32>,
    pub dispatch: HashMap<String, AworsetOpt<i32>>,
    pub cache: HashMap<String, AworsetOpt<i32>>, 
}

impl Wrapper {
    pub fn new(id: NodeId) -> Self {
        Self {
            local: AworsetOpt::new(id.clone()),
            dispatch: HashMap::new(),
            cache: HashMap::new()
        }
    }

    pub fn apply_oper(&mut self, oper: Op<i32>) {
        match oper {
            Op::RM(elem) => {
                self.local.rm(elem);
                self.dispatch.iter().for_each(|aw| {
                    aw.1.rm(elem);
                });
            }
            Op::ADD(elem) => {
                self.local.add(elem);
            }
        }
    }

    /// Saves the current state to be delivered in the future. 
    pub fn prepare_dispatch(&mut self, id: String) {
        self.dispatch.entry(id).and_modify(|aw| *aw = self.local.clone()).or_insert(self.local.clone());
    }

    /// Sends the element to be delivered to another element. 
    pub fn get_deliverable(&mut self, id: String) -> AworsetOpt<i32>{

    }

    pub receive_deliverable(&mut self) {

    }


    pub fn fetch(&self) -> HashSet<i32> {
        self.local.elements()
    }

    
}
