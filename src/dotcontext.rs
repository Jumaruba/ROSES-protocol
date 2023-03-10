use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

use crate::nodeId::NodeId;
use crate::types::Dot;

/// Tries to optimize mapping.
/// Inspired in: https://github.com/CBaquero/delta-enabled-crdts/blob/master/delta-crdts.cc
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DotContext {
    pub cc: HashMap<(NodeId, i64), i64>, // Compact Context. {(id,sck) -> tag}
    pub dc: HashSet<Dot>,                // Dot cloud
}

impl DotContext {
    pub fn new() -> Self {
        Self {
            cc: HashMap::new(),
            dc: HashSet::new(),
        }
    }

    // STANDARD FUNCTIONS =======================================

    pub fn is_empty(&self) -> bool {
        return self.cc.is_empty() && self.dc.is_empty();
    }

    pub fn clear(&mut self) {
        self.cc.clear();
        self.dc.clear();
    }

    /// Renames a cc element based in a translation.
    pub fn rm_dc(&mut self, dot: &Dot) -> bool {
        self.dc.remove(dot)
    }

    /// Inserts an element in dc.
    /// If there is no entry for the id, it creates it.
    pub fn add_dc(&mut self, dot: &Dot, compact: Option<bool>) {
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
        let cc_entry = self.cc.entry((id.clone(), sck)).or_insert(0);
        *cc_entry += 1;
        Dot::new(id.clone(), sck, cc_entry.clone())
    }

    /// Joins two dot contexts.
    pub fn join(&mut self, other: &Self) {
        for (key, &n) in other.cc.iter() {
            self.cc
                .entry(key.clone())
                .and_modify(|self_n| *self_n = max(*self_n, n))
                .or_insert(n);
        }

        self.dc.extend(other.dc.clone()); // union dc's
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
                    match self.cc.get_mut(&(dot.id.clone(), dot.sck)) {
                        Some(n) => {
                            if dot.n - 1 == *n {
                                *n += 1;
                                repeat = true;
                                return false; // Do not re-add it to dc.
                            } else if dot.n <= *n {
                                return false; // Dot not re-add it to dc.
                                              // Repeat flag remains the same.
                            }
                        }
                        None => {
                            if dot.n == 1 {
                                self.cc.insert((dot.id.clone(), dot.sck), 1);
                                repeat = true;
                                return false; // Do not re-add it to dc.
                            }
                        }
                    }
                    return true;
                })
                .collect();
        }
    }

    // UTILS    =====================================================

    /// Verifies if the received argument was already seen.
    pub fn dot_in(&self, d: &Dot) -> bool {
        if let Some(val) = self.cc.get(&(d.id.clone(), d.sck)) {
            if val >= &d.n {
                return true;
            }
        }
        return self.dc.contains(d);
    }
}
