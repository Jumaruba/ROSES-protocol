use std::collections::HashSet;

use crdt_sample::{AworsetOpt, NodeId};

use super::Op;

#[derive(Clone, Debug)]
pub struct Wrapper {
    pub local: AworsetOpt<i32>,
    pub dispatch: AworsetOpt<i32>,
}

impl Wrapper {
    pub fn new(id: NodeId) -> Self {
        Self {
            local: AworsetOpt::new(id.clone()),
            dispatch: AworsetOpt::new(id.clone()),
        }
    }


    pub fn apply_oper(&mut self, oper: Op<i32>) {
        match oper {
            Op::RM(elem) => {
                self.local.rm(elem);
                self.dispatch.rm(elem);
            }
            Op::ADD(elem) => {
                self.local.add(elem);
            }
        }

    }
    pub fn prepare_dispatch(&mut self) {
        self.dispatch = self.local.clone();
    }

    pub fn join(&mut self, other: &Self){
        let mut to_join = self.dispatch.clone();
        to_join.join(&other.dispatch);  // Feels like applying the translation.
        self.local.join(&to_join);
    }

    pub fn fetch(&self) -> HashSet<i32>{
        let mut res = self.local.elements();
        res.extend(self.dispatch.elements());
        return res;
    }
    
}
