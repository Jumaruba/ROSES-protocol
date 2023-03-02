use priority_queue::PriorityQueue;
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

/// Tries to optimize mapping.
/// Source: https://github.com/CBaquero/delta-enabled-crdts/blob/master/delta-crdts.cc
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DotContext<K: PartialEq + Eq + Hash + Clone + Debug> {
    pub cc: HashMap<K, HashMap<i64, i64>>, // Compact Context. {id -> {sck -> tag}}
    pub dc: HashMap<K, HashSet<(i64, i64)>>, // Dot cloud. { id -> {(sck, tag)}}
}

impl<K: PartialEq + Eq + Hash + Clone + Debug> DotContext<K> {
    pub fn new() -> Self {
        Self {
            cc: HashMap::new(),
            dc: HashMap::new(),
        }
    }

    /// Verifies if the received argument was already seen.
    /// # Arguments
    /// - d: A triple as (id, sck, counter).
    ///
    /// # Explanation
    /// Checks if the element was already computed in cc.
    /// Case not, check in dc.
    /// !NOTE to test
    pub fn dotin(&self, d: &(K, i64, i64)) -> bool {
        if let Some(hash) = self.cc.get(&d.0) {
            if let Some(val) = hash.get(&d.1) {
                if val >= &d.2 {
                    return true;
                }
            }
        }
        if let Some(set) = self.dc.get(&d.0) {
            return set.contains(&(d.1, d.2));
        }

        false
    }

    /// Creates a new dot considering that the dots are compacted.
    /// !NOTE to test
    pub fn makedot(&mut self, id: &K, sck: i64) -> (K, i64, i64) {
        let mut curr_tag: i64 = 1;
        match self.cc.get_mut(&id) {
            Some(cc_hash) => {
                cc_hash.entry(sck).and_modify(|val| *val +=1);
                curr_tag = cc_hash[&sck];
            },
            None => {
                self.cc.insert(id.clone(), HashMap::from([(sck, 1)]));
            }
        }
        (id.clone(), sck, curr_tag)
    }

    /// Inserts an element in dc.
    /// !NOTE to test
    pub fn insert_dot(&mut self, id: &K, sck: i64, tag: i64, compact: Option<bool>) {
        // Node knows the id.
        if let Some(set) = self.dc.get_mut(id) {
            set.insert((sck, tag));
        } else {
            self.dc.insert(id.clone(), HashSet::from([(sck, tag)]));
        }
        match compact {
            Some(true) => self.compact(),
            Some(false) => return,
            None => self.compact(),
        }
    }

    /// TODO: to test
    pub fn join(&mut self, other: &Self) {
        for (id, other_hash) in other.cc.iter() {
            for (sck, other_val) in other_hash.into_iter() {
                // The id is at self.
                if let Some(self_hash) = self.cc.get_mut(id) {
                    self_hash
                        .entry(*sck)
                        .and_modify(|self_val| *self_val = max(self_val.clone(), other_val.clone()))
                        .or_insert(*other_val);
                } else {
                    self.insert_dot(id, sck.clone(), other_val.clone(), Some(false));
                }
            }
        }

        self.union_dc(&other.dc);
        self.compact();
    }

    fn union_dc(&mut self, dc: &HashMap<K, HashSet<(i64, i64)>>) {
        for (id, other_hash) in dc.iter() {
            if let Some(self_hash) = self.dc.get(id) {
                self_hash.union(other_hash);
            } else {
                self.dc.insert(id.clone(), other_hash.clone());
            }
        }
    }

    pub fn compact(&mut self) {
        let mut repeat: bool = true;
        while repeat {
            repeat = false;
            for (id, set) in self.dc.iter_mut() {
                *set = set.drain().filter(|(sck, dc_tag)| {
                    match self.cc.get_mut(&id) {
                        Some(cc_hash) => {
                            if let Some(cc_tag) = cc_hash.get_mut(sck) {
                                if *cc_tag == dc_tag.clone() - 1 {
                                    *cc_tag += 1;
                                    repeat = true;
                                    return false; // Do not re-add it to dc.
                                } else if *cc_tag >= *dc_tag {
                                    return false; // Dot not re-add it to dc.
                                                  // Repeat flag remains the same.
                                }
                            } else if *dc_tag == 1 {
                                repeat = true;
                                cc_hash.insert(sck.clone(), 1);
                                return false;
                            }
                        }
                        None => {
                            if *dc_tag == 1 {
                                self.cc.insert(id.clone(), HashMap::from([(*sck, 1)]));
                                repeat = true;
                                return false; // Do not re-add it to dc.
                            }
                        }
                    }
                    return true;
                }).collect();
            }
        }
    }
}
