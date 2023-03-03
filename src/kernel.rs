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
    pub set: HashMap<NodeId, HashSet<(i64, i64, E)>>,   // Created as a Hash, because it's more effiecient to separate the self elements, from the others. Hash: (sck, tag, element). 
    pub cc: DotContext<NodeId>, // Change this to a HashMap. 
}

impl<E> Kernel<E>
where
    E: Eq + Display + Clone + Hash + Debug,
{
    pub fn new(id: &NodeId, sck: i64) -> Self {
        Self {
            id: id.clone(), 
            set: HashMap::new(),
            cc: DotContext::new(),
        }
    }

    pub fn get_last_tag(&self, sck: i64) -> i64{
        todo!()
    }
    
    /// TODO: to support self and other sets;
    pub fn elements(&self) -> HashSet<E>{
        todo!()
    }

    /// Adds an element and return the added entry.
    pub fn add(&mut self, element: E) -> (E, i64) {
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

