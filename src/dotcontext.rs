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

    /// TODO: to test
    /// Compact removes the empty values?
    pub fn rm_cc_n(&mut self, dot: &Dot) {
        self.cc.entry(dot.id.clone()).and_modify(|hash| {
            *hash = hash.drain().filter(|(_, n)| *n != dot.n).collect();
        });
    }

    /// Verifies if there is information about a node.
    pub fn has_seen(&self, id: &NodeId) -> bool {
        return self.cc.contains_key(id) || self.dc.contains_key(id);
    }

    /// Verifies if the received argument was already seen.
    pub fn dotin(&self, d: &Dot) -> bool {
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

    pub fn rename_cc(&mut self, transl: (Dot, Dot)) {
        if let Some(hash) = self.cc.get(&transl.0.id).clone() {
            if let Some(n) = hash.get(&transl.0.sck).clone() {
                if *n == transl.0.n {
                    self.insert_dot(&transl.1.id, transl.1.sck, transl.1.n, Some(false));
                    self.rm_cc_n(&transl.0);
                }
            }
        }
        self.compact();
    }

    // --------------------------
    // OPERATIONS
    // --------------------------

    /// Creates a new dot considering that the dots are compacted.
    /// Gets the corresponsing n in self.cc and increment it.
    /// # Example
    /// ```
    /// use thesis_code::dotcontext::DotContext;
    /// let mut dotctx: DotContext<String> = DotContext::new();
    /// dotctx.makedot(&"A".to_string(), 1);
    /// dotctx.makedot(&"A".to_string(), 1);
    /// let dot_3 = dotctx.makedot(&"A".to_string(), 3);
    /// let res_dot = ("A".to_string(), 3, 1); // (E,sck,n)
    /// let res_dotctx = "DotContext { cc: {\"A\": {3: 1, 1: 2}}, dc: {} }";
    /// let format_dotctx = format!("{:?}", dotctx);
    /// assert_eq!(dot_3, res_dot);
    /// assert_eq!(*dotctx.cc.get(&"A".to_string()).unwrap().get(&3).unwrap(), 1);
    /// assert_eq!(*dotctx.cc.get(&"A".to_string()).unwrap().get(&1).unwrap(), 2);
    /// ```
    pub fn makedot(&mut self, id: &NodeId, sck: i64) -> (NodeId, i64, i64) {
        // Get hash (sck, n) or create it.
        let cc_hash = self
            .cc
            .entry(id.clone())
            .or_insert(HashMap::from([(sck, 0)]));
        // Get n or create it.
        cc_hash.entry(sck).and_modify(|val| *val += 1).or_insert(1);

        (id.clone(), sck, cc_hash[&sck])
    }

    /// Inserts an element in dc.
    /// If there is no entry for the id, it creates it.
    /// # Example
    /// ```
    /// use thesis_code::dotcontext::DotContext;
    /// use std::collections::HashSet;
    /// let mut dotctx: DotContext<String> = DotContext::new();
    /// dotctx.insert_dot(&"A".to_string(), 1, 4, Some(false));
    /// dotctx.insert_dot(&"A".to_string(), 1, 2, Some(false));
    /// assert_eq!(dotctx.dc[&"A".to_string()], HashSet::from([(1,2), (1,4)]));
    /// ```
    ///
    pub fn insert_dot(&mut self, id: &NodeId, sck: i64, n: i64, compact: Option<bool>) {
        self.dc
            .entry(id.clone())
            .and_modify(|hash| {
                hash.insert((sck, n));
            })
            .or_insert(HashSet::from([(sck, n)]));

        match compact {
            Some(true) => self.compact(),
            Some(false) => return,
            None => self.compact(),
        }
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

    /// TODO: make more tests on this.
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
    }

    // --------------------------
    // UTILS
    // --------------------------

    /// TODO: to test
    pub fn clean_id(&mut self, id: &NodeId) {
        self.cc.remove(id);
        self.dc.remove(id);
    }

    fn union_dc(&mut self, dc: &HashMap<NodeId, HashSet<(i64, i64)>>) {
        for (id, other_hash) in dc.iter() {
            self.dc
                .entry(id.clone())
                .and_modify(|hash| {
                    hash.extend(other_hash);
                })
                .or_insert(other_hash.clone());
        }
    }
}
