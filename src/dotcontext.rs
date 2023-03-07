use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

use crate::nodeId::NodeId;
use crate::types::Dot;

/// Tries to optimize mapping.
/// Inspired in: https://github.com/CBaquero/delta-enabled-crdts/blob/master/delta-crdts.cc
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DotContext {
    pub cc: HashMap<NodeId, HashMap<i64, i64>>, // Compact Context. {id -> {sck -> tag}}
    pub dc: HashMap<NodeId, HashSet<(i64, i64)>>, // Dot cloud. { id -> {(sck, tag)}}
}

impl DotContext {
    pub fn new() -> Self {
        Self {
            cc: HashMap::new(),
            dc: HashMap::new(),
        }
    }

    // STANDARD FUNCTIONS =======================================

    /// Verifies if the received argument was already seen.
    pub fn dot_in(&self, d: &Dot) -> bool {
        if let Some(hash) = self.cc.get(&d.id) {
            if let Some(val) = hash.get(&d.sck) {
                if val >= &d.n {
                    return true;
                }
            }
        }
        if let Some(set) = self.dc.get(&d.id) {
            return set.contains(&(d.sck, d.n));
        }

        false
    }

    /// Renames a cc element based in a translation.
    pub fn rename_cc(&mut self, transl: (Dot, Dot)) {
        if let Some(hash) = self.cc.get(&transl.0.id) {
            if let Some(n) = hash.get(&transl.0.sck) {
                if *n == transl.0.n {
                    self.add_cc(transl.1);
                    self.rm_cc(&transl.0);
                }
            }
        }
    }

    /// Renames a cc element based in a translation.
    pub fn rename_dc(&mut self, transl: (Dot, Dot)) {
        if let Some(hash) = self.dc.get_mut(&transl.0.id) {
            let tuple = (transl.0.sck, transl.0.n);
            if hash.contains(&tuple) {
                hash.remove(&tuple);
                self.add_dc(&transl.1, Some(true));
            }
        }
    }

    // OPERATIONS   =================================================

    /// Creates a new dot considering that the dots are compacted.
    /// Gets the corresponsing n in self.cc and increment it.
    pub fn makedot(&mut self, id: &NodeId, sck: i64) -> Dot {
        // Get hash (sck, n) or create it.
        let cc_hash = self
            .cc
            .entry(id.clone())
            .or_insert(HashMap::from([(sck, 0)]));

        // Get n or create it.
        cc_hash.entry(sck).and_modify(|val| *val += 1).or_insert(1);

        Dot {id:id.clone(), sck:sck, n: cc_hash[&sck]}
    }

    /// Inserts an element in dc.
    /// If there is no entry for the id, it creates it.
    pub fn add_dc(&mut self, dot: &Dot, compact: Option<bool>) {
        self.dc
            .entry(dot.id.clone())
            .and_modify(|hash| {
                hash.insert((dot.sck, dot.n));
            })
            .or_insert(HashSet::from([(dot.sck, dot.n)]));

        match compact {
            Some(true) => self.compact(),
            Some(false) => return,
            None => self.compact(),
        }
    }
 
    /// Adds an entry to cc.
    pub fn add_cc(&mut self, dot: Dot) {
        self.cc
            .entry(dot.id)
            .and_modify(|map| {
                map.insert(dot.sck, dot.n);
            })
            .or_insert(HashMap::from([(dot.sck, dot.n)]));
    }

    /// Joins two dot contexts.
    pub fn join(&mut self, other: &Self) {
        for (id, other_hash) in other.cc.iter() {
            for (sck, other_val) in other_hash.iter() {
                self.cc
                    .entry(id.clone())
                    .or_insert(HashMap::new())
                    .entry(*sck)
                    .and_modify(|self_val| *self_val = max(self_val.clone(), other_val.clone()))
                    .or_insert(*other_val);
            }
        }

        self.union_dc(&other.dc);
        self.compact();
    }

    pub fn compact(&mut self) {
        let mut repeat: bool = true;
        while repeat {
            repeat = false;
            for (id, set) in self.dc.iter_mut() {
                *set = set
                    .drain()
                    .filter(|(sck, dc_tag)| {
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
                    })
                    .collect();
            }
        }
        // Remove entries with empty values
        self.dc = self.dc.drain().filter(|(_, hash)| {!hash.is_empty()}).collect();
    }

    // UTILS    =====================================================

    /// Verifies if there is information about a node.
    pub fn id_in(&self, id: &NodeId) -> bool {
        return self.cc.contains_key(id) || self.dc.contains_key(id);
    }

    /// Removes id's information from the dotcontext.
    /// Entries in self.dc and self.cc are removed.
    pub fn rm_id(&mut self, id: &NodeId) {
        self.cc.remove(id);
        self.dc.remove(id);
    }

    /// Joins a dc to the self.dc
    pub fn union_dc(&mut self, dc: &HashMap<NodeId, HashSet<(i64, i64)>>) {
        for (id, other_hash) in dc.iter() {
            self.dc
                .entry(id.clone())
                .and_modify(|hash| {
                    hash.extend(other_hash);
                })
                .or_insert(other_hash.clone());
        }
    }

    /// Parses self.cc of a specific id to a HashSet<Dot>.
    pub fn cc2set(&self, id: &NodeId) -> HashSet<Dot> {
        let mut res: HashSet<Dot> = HashSet::new();
        self.cc
            .get(id)
            .unwrap_or(&HashMap::new())
            .iter()
            .for_each(|(sck, n)| {
                res.insert(Dot {
                    id: id.clone(),
                    sck: sck.clone(),
                    n: n.clone(),
                });
            });
        res
    }

    /// Removes a dot from cc.
    /// If the entry becomes an empty HashMap, the entry is removed.
    pub fn rm_cc(&mut self, dot: &Dot) {
        self.cc.entry(dot.id.clone()).and_modify(|hash| {
            *hash = hash.drain().filter(|(_, n)| *n != dot.n).collect();
        });

        // Remove case empty entry.
        if self.cc.contains_key(&dot.id) && self.cc[&dot.id].is_empty() {
            self.cc.remove(&dot.id);
        }
    }

   

}
