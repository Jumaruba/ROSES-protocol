use core::hash::Hash;
use std::{collections::{HashSet, HashMap}, fmt::Display};
use std::fmt::Debug;
use crate::{NodeId, DotContext};

#[derive(Clone, Debug, PartialEq, Eq)]

// TODO: this gonna be the kernel 
pub struct Kernel<E>
where
    E: Eq + Display + Clone + Hash + Debug,
{
    pub id: NodeId,
    pub set: HashMap<NodeId, (E, i64, i64)>,   
    pub cc: DotContext<NodeId>, // Change this to a HashMap. 
}

impl<E> Kernel<E>
where
    E: Eq + Display + Clone + Hash + Debug,
{
    pub fn new(id: &NodeId) -> Self {
        Self {
            id: id.clone(), 
            set: HashMap::new(),
            cc: DotContext::new(),
        }
    }
    
    /// TODO: to support self and other sets;
    pub fn elements(&self) -> HashSet<E>{
        self.set.iter().map(|(_, triple)| triple.0.clone()).collect()
    }

    pub fn add(&mut self, element: E, sck: i64) {
        todo!()
    }

    /// TODO: to support self_set 
    pub fn rm(&mut self, element: E) {
        todo!()
    }

    pub fn join(&mut self, other: &mut Self){
        todo!()
    }
}

