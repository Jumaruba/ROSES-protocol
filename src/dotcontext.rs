use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

/// Tries to optimize mapping.
/// Source: https://github.com/CBaquero/delta-enabled-crdts/blob/master/delta-crdts.cc
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DotContext<K: PartialEq + Eq + Hash + Clone + Debug> {
    /// Compact Causal Context
    pub cc: HashMap<K, i64>,           
    /// Dot Cloud
    pub dc: HashSet<(K, i64)>,         
    /// Dot Translation Cloud
    pub dtc: HashSet<(K, i64, K, i64)>, 
}

impl<K: PartialEq + Eq + Hash + Clone + Debug> DotContext<K> {
    pub fn new() -> Self {
        Self {
            cc: HashMap::new(),
            dc: HashSet::new(),
            dtc: HashSet::new(),
        }
    }

    /// Verifies if the received argument was already seen.
    pub fn dotin(&self, d: &(K, i64)) -> bool {
        if let Some(&v) = self.cc.get(&d.0) {
            if d.1 <= v {
                return true;
            }
        } else if let Some(_) = self.dc.get(d) {
            return true;
        }
        return false;
    }

    /// Gets the maximum value associated to an id, considering that it's not compacted. 
    pub fn get_key_val(&self, id: &K) -> i64 {
        let mut max_val = self.cc.get(id).unwrap_or(&0);
        for (dc_id, dc_val) in self.dc.iter() {
            if id == dc_id {
                max_val = max(max_val, dc_val);
            }
        }
        *max_val
    }

    /// Cleans the entries of a specific id. 
    /// Attention: Translations are not removed.
    pub fn set_empty_self(&mut self, id: &K) {
        self.cc
            .entry(id.clone())
            .and_modify(|v| *v = 0)
            .or_insert(0); // Reset cc to zero
        self.dc = self.dc.drain().filter(|(key, _)| key != id).collect(); // Remove id's entries
    }

    /// Creates a new dot considering that the dots are compated.
    pub fn makedot(&mut self, id: &K) -> (K, i64) {
        match self.cc.get_mut(id) {
            // No entry, then create one.
            None => {
                self.cc.insert(id.clone(), 1);
            }
            // There is an entry, then update it.
            Some(v) => {
                *v += 1;
            }
        }
        return (id.clone(), self.cc.get(id).unwrap().clone());
    }

    /// Adds a dot to the struct.
    pub fn insert_dot(&mut self, dot: &(K, i64), compact: Option<bool>) {
        self.dc.insert(dot.clone());
        match compact {
            Some(true) => self.compact(),
            Some(false) => return,
            None => self.compact(),
        }
    }

    /// Transform translations with values less than n in dc elements.
    pub fn remove_dtdot(&mut self, target_id: &K, target_n: &i64) {
        self.dtc = self
            .dtc
            .drain()
            .filter(|(id, n, _, _)| id == target_id && n <= target_n )
            .collect();
    }

    /// Create a dot translation and add it to the dot cloud and dot translation cloud.
    pub fn make_dtdot(&mut self, id: &K, n: i64) ->(K, i64, K, i64) {
        let (new_id, new_n) = self.makedot(id);
        let dtr = (new_id, new_n, id.clone(), n);
        self.dtc.insert(dtr.clone());
        dtr
    }

    pub fn join(&mut self, other: &Self) {
        for (other_k, &other_val) in other.cc.iter() {
            match self.cc.get_mut(&other_k) {
                // No previous record, then insert.
                None => {
                    self.cc.insert(other_k.clone(), other_val);
                }
                // Get maximum between both.
                Some(self_val) => {
                    *self_val = max(*self_val, other_val);
                }
            }
        }

        self.union_dc(&other.dc);
        self.compact();
    }

    /// Performs the union between the self.dc and the one received.
    fn union_dc(&mut self, dc: &HashSet<(K, i64)>) {
        for (id, val) in dc.iter() {
            self.dc.insert((id.clone(), val.clone()));
        }
    }

    pub fn compact(&mut self) {
        let mut repeat: bool;
        loop {
            repeat = false;
            self.dc = self
                .dc
                .drain()
                .filter(|(id, dc_count)| {
                    match self.cc.get_mut(id) {
                        // No CC entry.
                        None => {
                            // If starts, with 1 (not decoupled), can compact.
                            if *dc_count == 1 {
                                self.cc.insert(id.clone(), *dc_count);
                                repeat = true;
                                return false; // Do not re-add it to dc.
                            }
                        }
                        // There is already a CC entry.
                        Some(cc_count) => {
                            // Contiguos, compact.
                            if *cc_count == *dc_count - 1 {
                                *cc_count += 1;
                                repeat = true;
                                return false; // Do not re-add it to dc.
                            }
                            // Has dc_count is already considered in cc.
                            else if *cc_count >= *dc_count {
                                return false; // Do not re-add it to dc.
                                              // No extra compact opportunities. Flag untoched.
                            }
                        }
                    }
                    return true; // cc_count <= dc_count.
                })
                .collect();

            if !repeat {
                break;
            }
        }
    }

    pub fn is_empty_set(&self) -> bool {
        self.dc.is_empty()
    }
}
