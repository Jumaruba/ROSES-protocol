use core::hash::Hash;
use std::{collections::{HashSet}, fmt::Display};
use std::fmt::Debug;
use crate::{NodeId, DotContext};

#[derive(Clone, Debug, PartialEq, Eq)]

// TODO: this gonna be the kernel 
pub struct AworsetOpt<E>
where
    E: Eq + Display + Clone + Hash + Debug,
{
    pub id: NodeId,
    pub set: HashSet<(NodeId, E, i64, i64)>,   
    pub cc: DotContext<NodeId>, // Change this to a HashMap. 
    pub transl: HashSet<(NodeId, i64, i64, NodeId, i64, i64)>    // (id_src, sck_src_clock, counter_src, id_dst, sck_dst_clock_ counter_dst) 
}

impl<E> AworsetOpt<E>
where
    E: Eq + Display + Clone + Hash + Debug,
{
    pub fn new(id: NodeId) -> Self {
        Self {
            id, 
            set: HashSet::new(),
            cc: DotContext::new(),
            transl: HashSet::new()
        }
    }
    
    pub fn elements(&self) -> HashSet<E>{
        let mut res: HashSet<E> = HashSet::new();
        for (_, e, _, _) in self.set.iter(){
            res.insert(e.clone());
        }
        res
    }

    pub fn add(&mut self, element: E, sck: i64) {
        let (id, sck, val) = self.cc.makedot(&self.id, sck);
        self.set.insert((id, element, sck, val));
    }

    pub fn rm(&mut self, element: E) {
        self.set= self.set.drain().filter(|(_, e, _, _) | {
            if element == *e { return false; }   
            true
        }).collect();
    }

    pub fn join(&mut self, other: &mut Self){
        // Intersentions and elements not known by other.
        self.set = self.set.drain().filter(|v|
            other.set.contains(v) || !other.cc.dotin(&(v.0.clone(), v.2, v.3)))
        .collect();
        
        // Elements known by other, but not by self.
        for entry in other.set.iter() {
            if !self.cc.dotin(&(entry.0.clone(), entry.2, entry.3)) {
                self.set.insert(entry.clone());
            }
        }

        self.cc.join(&other.cc);
    }
}

