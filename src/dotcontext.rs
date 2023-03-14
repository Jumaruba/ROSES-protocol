use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

use crate::types::{Dot, NodeId};

/// Tries to optimize mapping.
/// Inspired in: https://github.com/CBaquero/delta-enabled-crdts/blob/master/delta-crdts.cc
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DotContext {
    pub cc: HashMap<(NodeId, i64), i64>, // Compact Context. {(id, sck) -> tag}}. This struct makes it easier to create tokens.
    pub dc: HashSet<Dot>, // Dot cloud. Doesn't grow much, thus it's ok to be a hashset and iterate over.
}

impl DotContext {
    pub fn new() -> Self {
        Self {
            cc: HashMap::new(),
            dc: HashSet::new(),
        }
    }
    /// Gets elements 
    pub fn get_cc(&mut self, id: &NodeId, sck: i64) -> i64 {
        *self.cc.get(&(id.clone(), sck)).unwrap_or(&0)
    }

    pub fn insert_cc(&mut self, dot: &Dot){
        self.cc.insert((dot.id.clone(), dot.sck), dot.n);
    }

    pub fn insert_dc(&mut self, dot: &Dot){
        self.dc.insert(dot.clone());
    }

    /// Adds a dot to the struct.
    pub fn insert_dot(&mut self, dot: &Dot, compact: Option<bool>) {
        self.dc.insert(dot.clone());
        match compact {
            Some(true) => self.compact(),
            Some(false) => return,
            None => self.compact(),
        }
    }

    /// Creates a new dot considering that the dots are compacted.
    /// Gets the corresponsing n in self.cc and increment it.
    pub fn makedot(&mut self, id: &NodeId, sck: i64) -> Dot {
        let n = self
            .cc
            .entry((id.clone(), sck))
            .and_modify(|val| *val += 1)
            .or_insert(1);
        Dot::new(id.clone(), sck, n.clone())
    }

    /// Joins two dot contexts.
    pub fn join(&mut self, other: &Self) {
        for ((id, sck), on) in other.cc.iter() {
            self.cc
                .entry((id.clone(), *sck))
                .and_modify(|sn| *sn = max(sn.clone(), on.clone()))
                .or_insert(*on);
        }

        self.dc.extend(other.dc.clone());
        self.compact();
    }

    pub fn compact(&mut self) {
        let mut repeat: bool = true;
        while repeat {
            repeat = false;
            self.dc = self
                .dc
                .drain()
                .filter(|dot| {
                    if let Some(n) = self.cc.get_mut(&(dot.id.clone(), dot.sck)) {
                        if *n == dot.n - 1 {
                            *n += 1;
                            repeat = true;
                            return false; // Do not re-add it to dc.
                        } else if *n >= dot.n {
                            return false; // Dot not re-add it to dc.
                                          // Repeat flag remains the same.
                        }
                    } else {
                        if dot.n == 1 {
                            self.cc.insert((dot.id.clone(), dot.sck), 1);
                            repeat = true;
                            return false; // Do not re-add it to dc.
                        }
                    }
                    return true;
                })
                .collect();
        }
    }

    /// Verifies if the received argument was already seen.
    pub fn dot_in(&self, d: &Dot) -> bool {
        if let Some(val) = self.cc.get(&(d.id.clone(), d.sck)) {
            if val >= &d.n {
                return true;
            }
        }
        self.dc.contains(&d)
    }

    /// Verifies if there is information about a node.
    pub fn id_in(&self, id: &NodeId, sck: i64) -> bool {
        if self.cc.contains_key(&(id.clone(), sck)) {
            return true;
        }
        for dot in self.dc.iter() {
            if *id == dot.id {
                return true;
            }
        }
        return false;
    }

    /// Removes id's information from the dotcontext.
    /// Entries in self.dc and self.cc are removed.
    pub fn clean_id(&mut self, id: &NodeId, sck: i64) {
        self.cc.remove(&(id.clone(), sck));
        self.dc = self.dc.drain().filter(|dot| dot.id != *id).collect();
    }
}
